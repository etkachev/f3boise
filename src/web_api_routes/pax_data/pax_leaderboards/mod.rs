mod leaderboard_state;

use crate::db::queries::all_back_blasts::pax_bd_stats;
use crate::db::queries::all_back_blasts::pax_bd_stats::PaxBdStats;
use crate::db::queries::processed_items::{get_processed_items, process_items, ProcessedItem};
use crate::shared::common_errors::AppError;
use crate::shared::processed_type::{NewProcessItem, ProcessedType, ResolvingProcessedItems};
use crate::shared::string_utils::map_slack_id_to_link;
use crate::shared::time::local_boise_time;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::channels::private_channels;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::web_api_routes::auth::internal_auth;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDate;
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn post_pax_leaderboards(
    db_pool: web::Data<PgPool>,
    web_state: web::Data<MutableWebState>,
    req: HttpRequest,
) -> impl Responder {
    if internal_auth::valid_internal_request(&req).is_ok() {
        let pax_data = get_pax_data(&db_pool).await;
        if pax_data.is_empty() {
            HttpResponse::Ok().body("No leaderboards")
        } else {
            let item_ids = pax_data.get_item_ids();
            let existing_processed = filter_processed_items(&db_pool, &item_ids, &pax_data).await;

            match process_and_post(&db_pool, existing_processed, &web_state).await {
                Ok(_) => HttpResponse::Ok().body("success"),
                Err(err) => {
                    println!("Err saving processing items: {:?}", err);
                    HttpResponse::BadRequest()
                        .body("Something went wrong processing items on backend")
                }
            }
        }
    } else {
        HttpResponse::Forbidden().body("Not authorized key")
    }
}

async fn process_and_post(
    db: &PgPool,
    leaderboard: LeaderboardItems,
    web_state: &MutableWebState,
) -> Result<(), AppError> {
    process_items(db, &leaderboard).await?;
    let block_builder = get_blocks_for_leaderboard(&leaderboard);
    web_state
        .post_message(PostMessageRequest::new(
            private_channels::ACHIEVEMENTS_CHANNEL_ID,
            block_builder.blocks,
        ))
        .await?;
    Ok(())
}

async fn filter_processed_items(
    db: &PgPool,
    item_ids: &[String],
    leaderboard: &LeaderboardItems,
) -> LeaderboardItems {
    match get_processed_items(db, item_ids).await {
        Ok(items) => LeaderboardItems::filtered_items(leaderboard, &items),
        Err(err) => {
            println!("Error getting processed items: {:?}", err);
            LeaderboardItems::empty()
        }
    }
}

async fn get_pax_data(db: &PgPool) -> LeaderboardItems {
    let now = local_boise_time().date_naive();
    match pax_bd_stats::get_pax_bd_stats(db).await {
        Ok(pax_data) => LeaderboardItems::new(pax_data, now),
        Err(err) => {
            println!("Error getting pax bd stats: {:?}", err);
            LeaderboardItems::new(vec![], now)
        }
    }
}

fn get_blocks_for_leaderboard(items: &LeaderboardItems) -> BlockBuilder {
    let mut block_builder = BlockBuilder::new().header("Leaderboard Summary");

    for (_, pax_list) in items.full_map.iter() {
        if let Some(msg) = pax_list.state.message() {
            let pax = pax_list
                .slack_ids
                .iter()
                .map(|(id, meta)| {
                    let mapped_slack = map_slack_id_to_link(id);
                    if let Some(meta) = meta {
                        format!("{} {}", mapped_slack, meta)
                    } else {
                        mapped_slack
                    }
                })
                .collect::<Vec<String>>()
                .join(", ");
            let msg = format!("{} - {}", msg, pax);
            block_builder.add_section_markdown(&msg);
        }
    }

    block_builder
}

struct LeaderboardItems {
    /// key is the item_type of leaderboard stat
    full_map: HashMap<String, LeaderboardPaxList>,
}

impl LeaderboardItems {
    pub fn empty() -> Self {
        LeaderboardItems {
            full_map: HashMap::new(),
        }
    }

    pub fn new(pax_data: Vec<PaxBdStats>, now: NaiveDate) -> Self {
        let mut full_map = HashMap::<String, LeaderboardPaxList>::new();
        for item in pax_data {
            let mut states = Vec::<leaderboard_state::LeaderboardState>::new();
            let bd_count = leaderboard_state::LeaderboardState::bds(item.bd_count as usize);
            states.push(bd_count);
            if let Some(anni) = item.earliest_date {
                let anni_state = leaderboard_state::LeaderboardState::b_day(anni, now);
                states.push(anni_state);
            }

            for state_item in states {
                match (state_item.get_type_id(), state_item) {
                    (None, _) | (_, leaderboard_state::LeaderboardState::None) => {}
                    (Some(type_id), state) => {
                        let entry = full_map
                            .entry(type_id)
                            .or_insert(LeaderboardPaxList::new(state, &item));
                        entry.add_user(&item);
                    }
                }
            }
        }
        LeaderboardItems { full_map }
    }

    pub fn filtered_items(original: &LeaderboardItems, processed: &[ProcessedItem]) -> Self {
        let mut full_map = HashMap::<String, LeaderboardPaxList>::new();

        for (type_id, pax_list) in original.full_map.iter() {
            for (pax, meta) in pax_list.slack_ids.iter() {
                let unique_id = format!("{}.{}", type_id, pax);
                // if not already processed, add
                if !processed
                    .iter()
                    .any(|p_item| p_item.get_unique_id() == unique_id)
                {
                    let entry = full_map.entry(type_id.to_string()).or_insert(
                        LeaderboardPaxList::new_with_meta(pax_list.state.clone(), pax, meta),
                    );
                    entry.add_with_meta(pax, meta);
                }
            }
        }

        LeaderboardItems { full_map }
    }

    pub fn is_empty(&self) -> bool {
        self.full_map.is_empty()
    }

    pub fn get_item_ids(&self) -> Vec<String> {
        self.full_map
            .iter()
            .fold(Vec::new(), |mut acc, (id, pax_list)| {
                for (pax, _) in pax_list.slack_ids.iter() {
                    acc.push(format!("{}.{}", id, pax));
                }
                acc
            })
    }
}

impl ResolvingProcessedItems for LeaderboardItems {
    fn get_processed_items(&self) -> Vec<NewProcessItem> {
        self.full_map
            .iter()
            .fold(Vec::new(), |mut list, (item_type, pax_list)| {
                let pax_ids: Vec<String> =
                    pax_list.slack_ids.keys().map(|id| id.to_string()).collect();
                list.push(NewProcessItem::new(item_type, pax_ids));
                list
            })
    }
}

struct LeaderboardPaxList {
    state: leaderboard_state::LeaderboardState,
    slack_ids: HashMap<String, Option<String>>,
}

impl LeaderboardPaxList {
    pub fn new(state: leaderboard_state::LeaderboardState, pax_bd_stats: &PaxBdStats) -> Self {
        let meta = get_meta_data(pax_bd_stats, &state);
        LeaderboardPaxList {
            state,
            slack_ids: HashMap::from([(pax_bd_stats.slack_id.to_string(), meta)]),
        }
    }

    pub fn add_user(&mut self, pax_bd_stats: &PaxBdStats) {
        let meta = get_meta_data(pax_bd_stats, &self.state);
        self.slack_ids
            .insert(pax_bd_stats.slack_id.to_string(), meta);
    }

    pub fn new_with_meta(
        state: leaderboard_state::LeaderboardState,
        slack_id: &str,
        meta: &Option<String>,
    ) -> Self {
        LeaderboardPaxList {
            state,
            slack_ids: HashMap::from([(slack_id.to_string(), meta.clone())]),
        }
    }

    pub fn add_with_meta(&mut self, slack_id: &str, meta: &Option<String>) {
        self.slack_ids.insert(slack_id.to_string(), meta.clone());
    }
}

fn get_meta_data(
    pax_bd_stats: &PaxBdStats,
    state: &leaderboard_state::LeaderboardState,
) -> Option<String> {
    match state {
        leaderboard_state::LeaderboardState::ApproachingBDBD(_) => {
            pax_bd_stats.earliest_date.map(|date| format!("({})", date))
        }
        leaderboard_state::LeaderboardState::Approaching(_) => {
            Some(format!("({})", pax_bd_stats.bd_count))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime};

    fn new_pax_stat(
        slack_id: &str,
        name: &str,
        bd_count: i64,
        start_date: NaiveDate,
    ) -> PaxBdStats {
        PaxBdStats {
            slack_id: slack_id.to_string(),
            name: name.to_string(),
            earliest_date: Some(start_date),
            bd_count,
        }
    }

    fn new_processed_item(item_id: &str, item_type: &str) -> ProcessedItem {
        ProcessedItem {
            id: uuid::Uuid::new_v4(),
            item_type: item_type.to_string(),
            item_id: item_id.to_string(),
            date_updated: NaiveDateTime::default(),
            initial_date_processed: NaiveDateTime::default(),
            amt_processed: 1,
        }
    }

    #[test]
    fn initial_pax_list_and_filtered() {
        let pax_data: Vec<PaxBdStats> = vec![
            new_pax_stat("U1", "Stinger", 299, NaiveDate::from_ymd(2022, 2, 1)),
            new_pax_stat("U2", "Backslash", 345, NaiveDate::from_ymd(2023, 3, 29)),
            new_pax_stat("U3", "Rocketman", 433, NaiveDate::from_ymd(2022, 1, 3)),
            new_pax_stat("U4", "Puff", 299, NaiveDate::from_ymd(2023, 4, 3)),
        ];
        let now = NaiveDate::from_ymd(2024, 3, 28);
        let items = LeaderboardItems::new(pax_data, now);

        println!("blocks: {:?}", get_blocks_for_leaderboard(&items));
        assert_eq!(items.full_map.len(), 2);
        let processed_items: Vec<ProcessedItem> = vec![
            new_processed_item("U1", "leaderboard.state.approaching.bd.hnd.3"),
            new_processed_item("U4", "leaderboard.state.approaching.bd.hnd.3"),
        ];
        let filtered = LeaderboardItems::filtered_items(&items, &processed_items);
        assert_eq!(filtered.full_map.len(), 1);
        let list = filtered
            .full_map
            .iter()
            .map(|(_, item)| {
                item.slack_ids
                    .keys()
                    .find_map(|id| {
                        if !id.is_empty() {
                            Some(id.to_string())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default()
            })
            .collect::<Vec<String>>();

        let first = list.first().unwrap();
        assert_eq!(first, "U2");
    }
}
