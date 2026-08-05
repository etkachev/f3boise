use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::queries::pre_blasts::get_pre_blast_by_id;
use crate::db::queries::reactions_log::get_preblast_hc_users;
use crate::db::save_reaction_log::{save_reaction_item, ReactionLogDbItem};
use crate::web_api_state::MutableWebState;

#[derive(Deserialize, Debug)]
pub struct HcRequest {
    pub slack_user_id: String,
    pub action: HcAction,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HcAction {
    Add,
    Remove,
}

#[derive(Serialize)]
pub struct HcResponse {
    pub success: bool,
    pub message: String,
}

/// Receive HC from F3 and save to reactions_log, then update Slack message
pub async fn receive_hc_from_f3(
    db_pool: web::Data<PgPool>,
    web_state: web::Data<MutableWebState>,
    path: web::Path<String>,
    payload: web::Json<HcRequest>,
) -> impl Responder {
    let preblast_uuid = path.into_inner();

    println!(
        "🔍 [HC] Received HC from F3: preblast={}, slack_user={}, action={:?}",
        preblast_uuid, payload.slack_user_id, payload.action
    );

    // Parse UUID
    let entity_id = match Uuid::parse_str(&preblast_uuid) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("❌ [HC] Invalid UUID: {}", e);
            return HttpResponse::BadRequest().json(HcResponse {
                success: false,
                message: format!("Invalid preblast UUID: {}", e),
            });
        }
    };

    // Get preblast data from DB
    let preblast_data = match get_pre_blast_by_id(&db_pool, &preblast_uuid).await {
        Ok(Some(pb)) => pb,
        Ok(None) => {
            return HttpResponse::NotFound().json(HcResponse {
                success: false,
                message: format!("Preblast {} not found", preblast_uuid),
            });
        }
        Err(e) => {
            eprintln!("❌ [HC] Database error: {}", e);
            return HttpResponse::InternalServerError().json(HcResponse {
                success: false,
                message: "Database error".to_string(),
            });
        }
    };

    // Verify preblast has been posted to Slack
    let ts = match &preblast_data.ts {
        Some(ts) => ts.clone(),
        None => {
            eprintln!("❌ [HC] Preblast has no Slack message timestamp");
            return HttpResponse::BadRequest().json(HcResponse {
                success: false,
                message: "Preblast has not been posted to Slack yet".to_string(),
            });
        }
    };

    // Save HC to reactions_log
    let reaction_added = matches!(payload.action, HcAction::Add);
    let reaction_item = ReactionLogDbItem {
        id: Uuid::new_v4(),
        entity_type: "pre_blast".to_string(),
        entity_id,
        reaction: "hc".to_string(),
        slack_user: payload.slack_user_id.clone(),
        reaction_added,
        reaction_timestamp: Utc::now().naive_utc(),
    };

    if let Err(e) = save_reaction_item(&db_pool, reaction_item).await {
        eprintln!("❌ [HC] Failed to save HC: {}", e);
        return HttpResponse::InternalServerError().json(HcResponse {
            success: false,
            message: format!("Failed to save HC: {}", e),
        });
    }

    println!(
        "✅ [HC] Saved HC to reactions_log: preblast={}, user={}, added={}",
        preblast_uuid, payload.slack_user_id, reaction_added
    );

    // Get updated list of HC users
    let hc_users = match get_preblast_hc_users(&db_pool, entity_id).await {
        Ok(users) => users,
        Err(e) => {
            eprintln!("❌ [HC] Failed to get HC users: {}", e);
            return HttpResponse::InternalServerError().json(HcResponse {
                success: false,
                message: "Failed to retrieve HC list".to_string(),
            });
        }
    };

    // Rebuild and update the Slack message
    match update_preblast_message(&web_state, &preblast_data, &ts, &hc_users).await {
        Ok(_) => {
            println!("✅ [HC] Updated Slack message with new HC list");
            HttpResponse::Ok().json(HcResponse {
                success: true,
                message: "HC recorded and Slack message updated".to_string(),
            })
        }
        Err(e) => {
            eprintln!("⚠️ [HC] Failed to update Slack message: {}", e);
            // Still return success since we saved to DB
            HttpResponse::Ok().json(HcResponse {
                success: true,
                message: "HC recorded (Slack message update failed)".to_string(),
            })
        }
    }
}

/// Rebuild and update the preblast Slack message with current HCs
async fn update_preblast_message(
    web_state: &MutableWebState,
    preblast_data: &crate::db::queries::pre_blasts::PreBlastJsonFullData,
    ts: &str,
    hc_users: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::slack_api::block_kit::BlockBuilder;
    use crate::slack_api::chat::update_message::request::UpdateMessageRequest;
    use crate::web_api_routes::interactive_events::interaction_types::InteractionTypes;

    // Build where text with location
    let where_text = match &preblast_data.location_url {
        Some(url) if !url.is_empty() => {
            format!(
                "*Where*: <#{}> - <{}|View on Map>",
                preblast_data.channel_id, url
            )
        }
        _ => format!("*Where*: <#{}>", preblast_data.channel_id),
    };

    // Build Q list
    let qs_list = preblast_data.qs.join(" ");

    // Build equipment list
    let equipment_list = match &preblast_data.equipment {
        Some(equip) if !equip.is_empty() => equip.join(", "),
        _ => "None".to_string(),
    };

    // Build HC list
    let hc_text = if hc_users.is_empty() {
        "*HCs*: None yet".to_string()
    } else {
        let hc_mentions: Vec<String> = hc_users
            .iter()
            .map(|user_id| format!("<@{}>", user_id))
            .collect();
        format!("*HCs*: {}", hc_mentions.join(", "))
    };

    let mut block_builder = BlockBuilder::new()
        .section_markdown(&format!("*Preblast: {}*", preblast_data.title))
        .section_markdown(&format!("*Date*: {}", preblast_data.date))
        .section_markdown(&format!(
            "*Time*: {}",
            preblast_data.start_time.format("%H:%M")
        ))
        .section_markdown(&where_text)
        .section_markdown(&format!("*Q(s)*: {}", qs_list))
        .divider()
        .section_markdown(&format!("*Why*: {}", preblast_data.why))
        .section_markdown(&format!("*Equipment*: {}", equipment_list))
        .section_markdown(&format!(
            "*FNGs*: {}",
            preblast_data.fng_message.as_ref().unwrap_or(&String::new())
        ))
        .section_markdown(&format!(
            "*Moleskine*: {}",
            preblast_data.mole_skin.as_ref().unwrap_or(&String::new())
        ))
        .section_markdown(&hc_text) // HC list at bottom
        .divider();

    let interaction_btn = InteractionTypes::new_edit_pre_blast(&preblast_data.id.to_string());
    block_builder.add_btn("Edit Preblast", &interaction_btn.to_string(), "edit-preblast");
    block_builder.add_context("Saved Preblast (from F3 App)");

    let update_request =
        UpdateMessageRequest::new(&preblast_data.channel_id, ts, block_builder.blocks);
    web_state.update_message(update_request).await?;

    Ok(())
}
