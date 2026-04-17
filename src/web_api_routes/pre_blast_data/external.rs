use crate::app_state::ao_data::AO;
use crate::app_state::equipment::AoEquipment;
use crate::app_state::pre_blast_data::PreBlastData;
use crate::db::queries::users::get_slack_id_map;
use crate::db::save_pre_blast;
use crate::shared::f3_api_sync::sync_preblast;
use crate::web_api_routes::slash_commands::pre_blast::pre_blast_post;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashSet;

#[derive(Deserialize, Debug)]
pub struct ExternalPreBlastRequest {
    pub ao_name: String,
    pub title: String,
    pub date: String,  // YYYY-MM-DD
    pub time: String,  // HH:MM
    pub q_slack_ids: Vec<String>,  // Users with Slack IDs
    pub q_names_without_slack: Vec<String>,  // Users without Slack IDs (fallback)
    pub why: Option<String>,
    pub equipment: Vec<String>,
    pub fng_message: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct ExternalPreBlastResponse {
    pub id: String,
    pub success: bool,
    pub message: String,
}

/// Create preblast from external source (F3 API)
pub async fn create_pre_blast_from_external(
    db_pool: web::Data<PgPool>,
    web_state: web::Data<MutableWebState>,
    payload: web::Json<ExternalPreBlastRequest>,
) -> impl Responder {
    println!("Received external preblast request: {:?}", payload);

    // 1. Parse AO from name (case-insensitive matching)
    let ao = AO::from(payload.ao_name.clone());

    // Check if AO was recognized
    if matches!(ao, AO::Unknown(_)) {
        return HttpResponse::BadRequest().json(ExternalPreBlastResponse {
            id: String::new(),
            success: false,
            message: format!("Unknown AO: {}. Skipping Slack post.", payload.ao_name),
        });
    }

    // 2. Parse date and time
    let date = match NaiveDate::parse_from_str(&payload.date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::BadRequest().json(ExternalPreBlastResponse {
                id: String::new(),
                success: false,
                message: format!("Invalid date format: {}", e),
            });
        }
    };

    let time = match NaiveTime::parse_from_str(&payload.time, "%H:%M") {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::BadRequest().json(ExternalPreBlastResponse {
                id: String::new(),
                success: false,
                message: format!("Invalid time format: {}", e),
            });
        }
    };

    // 3. Combine Q Slack IDs and fallback names
    let mut all_qs = HashSet::new();

    // Add users with Slack IDs (already in Slack format)
    for slack_id in &payload.q_slack_ids {
        all_qs.insert(slack_id.clone());
    }

    // Add users without Slack IDs (use their F3 names)
    // These will be displayed as plain text in Slack
    for name in &payload.q_names_without_slack {
        all_qs.insert(name.clone());
    }

    // 4. Parse equipment
    let equipment: HashSet<AoEquipment> = payload
        .equipment
        .iter()
        .map(|name| {
            // Try to parse as known equipment, otherwise treat as "Other"
            match AoEquipment::try_from(name.as_str()) {
                Ok(eq) => eq,
                Err(_) => AoEquipment::Other(name.clone()),
            }
        })
        .collect();

    // 5. Get slack_id to name mapping (for display purposes)
    let users_map = match get_slack_id_map(&db_pool).await {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Failed to get slack_id map: {}", e);
            std::collections::HashMap::new()
        }
    };

    // Convert Slack IDs to names for storage
    let qs_for_storage: HashSet<String> = all_qs
        .iter()
        .map(|q| {
            // If it's a Slack ID (starts with U), try to map to name
            if q.starts_with('U') {
                users_map.get(q).cloned().unwrap_or_else(|| q.clone())
            } else {
                // Already a name
                q.clone()
            }
        })
        .collect();

    // 6. Create PreBlastData
    let db_data = PreBlastData {
        id: None,
        ao: ao.clone(),
        title: payload.title.clone(),
        qs: qs_for_storage,
        date,
        start_time: time,
        why: payload.why.clone().unwrap_or_default(),
        equipment,
        fng_message: payload.fng_message.clone(),
        mole_skin: payload.description.clone(),
        img_ids: HashSet::new(),
    };

    // 7. Save to DB
    let saved_id = match save_pre_blast::save_single(&db_pool, &db_data).await {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ExternalPreBlastResponse {
                id: String::new(),
                success: false,
                message: format!("Failed to save preblast: {}", e),
            });
        }
    };

    // 8. Sync to F3 API in background
    let sync_data = db_data.clone();
    let sync_id = saved_id.clone();
    actix_rt::spawn(async move {
        sync_preblast(&sync_data, &sync_id).await;
    });

    // 9. Post to Slack (using the original Slack IDs for mentions)
    let message = create_slack_message(&db_pool, &db_data, &saved_id, &all_qs).await;

    let ts = match web_state.post_message(message).await {
        Ok(ts) => ts,
        Err(e) => {
            eprintln!("Failed to post to Slack: {}", e);
            return HttpResponse::InternalServerError().json(ExternalPreBlastResponse {
                id: saved_id,
                success: false,
                message: format!("Saved but failed to post to Slack: {}", e),
            });
        }
    };

    if let Some(ts) = ts {
        if let Err(e) = save_pre_blast::update_pre_blast_ts(&db_pool, &saved_id, ts).await {
            eprintln!("Failed to update preblast ts: {}", e);
        }
    }

    HttpResponse::Ok().json(ExternalPreBlastResponse {
        id: saved_id,
        success: true,
        message: "Preblast created and posted to Slack successfully".to_string(),
    })
}

async fn create_slack_message(
    db_pool: &PgPool,
    data: &PreBlastData,
    id: &str,
    qs_slack_format: &HashSet<String>,
) -> crate::slack_api::chat::post_message::request::PostMessageRequest {
    use crate::slack_api::block_kit::BlockBuilder;
    use crate::slack_api::chat::post_message::request::PostMessageRequest;
    use crate::web_api_routes::interactive_events::interaction_types::InteractionTypes;

    let channel_id = data.ao.channel_id().to_string();

    // Format Qs for display (with @ mentions for Slack IDs)
    let qs_display: Vec<String> = qs_slack_format
        .iter()
        .map(|q| {
            if q.starts_with('U') {
                format!("<@{}>", q)
            } else {
                q.clone()
            }
        })
        .collect();
    let qs_list = qs_display.join(" ");

    let equipment_list = if data.equipment.is_empty() {
        "None".to_string()
    } else {
        data.equipment
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    };

    let mut block_builder = BlockBuilder::new()
        .section_markdown(&format!("*Preblast: {}*", data.title))
        .section_markdown(&format!("*Date*: {}", data.date))
        .section_markdown(&format!("*Time*: {}", data.start_time.format("%H:%M")))
        .section_markdown(&format!("*Where*: <#{}>", data.ao.channel_id()))
        .section_markdown(&format!("*Q(s)*: {}", qs_list))
        .divider()
        .section_markdown(&format!("*Why*: {}", data.why))
        .section_markdown(&format!("*Equipment*: {}", equipment_list))
        .section_markdown(&format!(
            "*FNGs*: {}",
            data.fng_message.as_ref().unwrap_or(&String::new())
        ))
        .section_markdown(&format!(
            "*Moleskine*: {}",
            data.mole_skin.as_ref().unwrap_or(&String::new())
        ))
        .divider();

    if !id.is_empty() {
        let interaction_btn = InteractionTypes::new_edit_pre_blast(id);
        block_builder.add_btn("Edit Preblast", &interaction_btn.to_string(), "edit-preblast");
        block_builder.add_context("Saved Preblast (from F3 App)");
    }

    // Post as bot (no specific user)
    PostMessageRequest::new(&channel_id, block_builder.blocks)
}
