use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::{BackBlastData, BackBlastType};
use crate::db::queries::users::get_slack_id_map;
use crate::db::save_back_blast;
use crate::web_api_routes::slash_commands::back_blast::back_blast_post;
use crate::web_api_routes::slash_commands::modal_utils::BlastWhere;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalBackBlastRequest {
    pub ao_name: String,                        // "ao-bleach"
    pub date: String,                           // "YYYY-MM-DD"
    pub q_slack_ids: Vec<String>,               // Slack User IDs for Qs (ONLY taggable Qs)
    pub q_names_without_slack: Vec<String>,     // Ignored - Qs must be taggable
    pub pax_slack_ids: Vec<String>,             // Slack User IDs for PAX
    pub pax_names_without_slack: Vec<String>,   // PAX names without Slack (untaggable PAX)
    pub fng_names: Vec<String>,                 // FNG names (plain text)
    pub title: Option<String>,
    pub moleskine: Option<String>,              // Description
    pub bb_type: Option<String>,                // "backblast", "doubledown", "otb"
}

#[derive(Debug, Serialize)]
pub struct ExternalBackBlastResponse {
    pub id: String,
    pub success: bool,
    pub message: String,
}

/// Create or update backblast from external source (F3 API)
pub async fn create_back_blast_from_external(
    db_pool: web::Data<PgPool>,
    web_state: web::Data<MutableWebState>,
    payload: web::Json<ExternalBackBlastRequest>,
) -> impl Responder {
    println!("🔍 [Backblast External] Received request: ao_name={}, date={}, title={:?}",
        payload.ao_name, payload.date, payload.title);
    println!("📦 [Backblast External] Q Slack IDs: {:?}, PAX Slack IDs: {:?}, PAX Names: {:?}, FNGs: {:?}",
        payload.q_slack_ids, payload.pax_slack_ids, payload.pax_names_without_slack, payload.fng_names);

    // 1. Parse AO from name (case-insensitive matching)
    let ao = AO::from(payload.ao_name.clone());

    // Check if AO was recognized
    if matches!(ao, AO::Unknown(_)) {
        return HttpResponse::BadRequest().json(ExternalBackBlastResponse {
            id: String::new(),
            success: false,
            message: format!("Unknown AO: {}. Skipping Slack post.", payload.ao_name),
        });
    }

    // 2. Parse date
    let workout_date = match NaiveDate::parse_from_str(&payload.date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::BadRequest().json(ExternalBackBlastResponse {
                id: String::new(),
                success: false,
                message: format!("Invalid date format: {}", e),
            });
        }
    };

    // 3. Parse workout type (default to "backblast")
    let bb_type = payload
        .bb_type
        .as_deref()
        .map(BackBlastType::from)
        .unwrap_or_default();

    // 4. Check if backblast already exists (by ao, date, bb_type)
    let existing_id = save_back_blast::get_id_by_ao_date_type(
        &db_pool,
        ao.channel_id(),
        &workout_date,
        &bb_type.to_string(),
    )
    .await;

    let backblast_id = if let Ok(Some(ref id)) = existing_id {
        println!("✅ [Backblast External] Found existing backblast: {}", id);
        id.clone()
    } else {
        println!("📝 [Backblast External] Creating new backblast");
        uuid::Uuid::new_v4().to_string()
    };

    // 5. Get slack_id to name mapping (for storage conversion)
    let users_map = match get_slack_id_map(&db_pool).await {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Failed to get slack_id map: {}", e);
            std::collections::HashMap::new()
        }
    };

    // 6. Convert Slack IDs to names for BackBlastData storage
    let qs_for_storage: HashSet<String> = payload
        .q_slack_ids
        .iter()
        .map(|slack_id| {
            users_map
                .get(slack_id)
                .cloned()
                .unwrap_or_else(|| slack_id.clone())
        })
        .collect();

    let mut pax_for_storage: HashSet<String> = payload
        .pax_slack_ids
        .iter()
        .map(|slack_id| {
            users_map
                .get(slack_id)
                .cloned()
                .unwrap_or_else(|| slack_id.clone())
        })
        .collect();

    // Add untaggable PAX names (trimmed for deduplication)
    pax_for_storage.extend(
        payload
            .pax_names_without_slack
            .iter()
            .map(|name| name.trim().to_string()),
    );

    // Add FNG names (trimmed, filter out "None")
    let fngs: HashSet<String> = payload
        .fng_names
        .iter()
        .filter_map(|name| {
            let trimmed = name.trim();
            if !matches!(trimmed, "none" | "None") {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect();

    // Add FNGs to PAX
    pax_for_storage.extend(fngs.iter().cloned());

    // Add Qs to PAX (Qs are counted as PAX) - HashSet auto-dedupes
    pax_for_storage.extend(qs_for_storage.clone());

    // 7. Create BackBlastData for DB storage (stores NAMES)
    let mut backblast_data = BackBlastData::new(
        ao.clone(),
        qs_for_storage,
        pax_for_storage,
        workout_date,
    )
    .with_type(bb_type.clone());

    backblast_data.id = Some(backblast_id.clone());
    backblast_data.title = payload.title.clone();
    backblast_data.moleskine = payload.moleskine.clone();
    backblast_data.fngs = fngs.clone();

    // 8. Save or update backblast in DB
    let save_result = if existing_id.is_ok() && existing_id.unwrap().is_some() {
        save_back_blast::update_back_blast(&db_pool, &backblast_id, &backblast_data).await
    } else {
        save_back_blast::save_single(&db_pool, &backblast_data)
            .await
            .map(|_| ())
    };

    if let Err(e) = save_result {
        eprintln!("❌ [Backblast External] Failed to save backblast: {}", e);
        return HttpResponse::InternalServerError().json(ExternalBackBlastResponse {
            id: backblast_id.clone(),
            success: false,
            message: format!("Failed to save: {}", e),
        });
    }

    // 9. Create BackBlastPost for Slack message (uses SLACK IDs, trimmed names)
    let post = back_blast_post::BackBlastPost {
        title: payload.title.clone().unwrap_or_else(|| "Backblast".to_string()),
        date: workout_date,
        ao: ao.clone(),
        qs: payload.q_slack_ids.iter().cloned().collect(),
        pax: payload.pax_slack_ids.iter().cloned().collect(),
        non_slack_pax: payload
            .pax_names_without_slack
            .iter()
            .map(|name| name.trim().to_string())
            .collect(),
        fngs,
        mole_skine: payload.moleskine.clone().unwrap_or_default(),
        blast_where: BlastWhere::AoChannel,
        bb_type,
    };

    // 10. Get first Q's Slack ID for posting as that user (or empty for bot)
    let action_user_id = payload
        .q_slack_ids
        .first()
        .map(|id| id.as_str())
        .unwrap_or("");

    // 11. Post to Slack using existing utility (formats Slack IDs as mentions)
    println!(
        "🚀 [Backblast External] Posting to Slack channel: {} as user: {}",
        ao.channel_id(),
        if action_user_id.is_empty() {
            "bot"
        } else {
            action_user_id
        }
    );
    let message = back_blast_post::convert_to_message(
        post,
        &db_pool,
        true, // is_valid
        Some(backblast_id.clone()),
        action_user_id,
    )
    .await;

    let ts = match web_state.post_message(message).await {
        Ok(ts) => {
            println!("✅ [Backblast External] Successfully posted to Slack");
            ts
        }
        Err(e) => {
            eprintln!("❌ [Backblast External] Failed to post to Slack: {}", e);
            return HttpResponse::InternalServerError().json(ExternalBackBlastResponse {
                id: backblast_id,
                success: false,
                message: format!("Saved but failed to post to Slack: {}", e),
            });
        }
    };

    // 12. Store message timestamp for future edits
    if let Some(ts) = ts {
        if let Err(e) = save_back_blast::update_back_blast_ts(&db_pool, &backblast_id, ts).await {
            eprintln!("Failed to update backblast ts: {}", e);
        }
    }

    HttpResponse::Ok().json(ExternalBackBlastResponse {
        id: backblast_id,
        success: true,
        message: "Backblast created and posted to Slack successfully".to_string(),
    })
}
