use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::reactions_log::PreBlastReactionLogItem;
use crate::db::queries::{all_back_blasts, reactions_log};
use crate::db::save_reaction_log::ReactionLogDbItem;
use actix_web::{web, HttpResponse, Responder};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ReactionLogRow {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub reaction: String,
    pub slack_user: String,
    pub reaction_added: bool,
    pub reaction_timestamp: NaiveDateTime,
}

impl From<ReactionLogDbItem> for ReactionLogRow {
    fn from(value: ReactionLogDbItem) -> Self {
        ReactionLogRow {
            id: value.id.to_string(),
            entity_type: value.entity_type.to_string(),
            entity_id: value.entity_id.to_string(),
            reaction: value.reaction.to_string(),
            slack_user: value.slack_user.to_string(),
            reaction_added: value.reaction_added,
            reaction_timestamp: value.reaction_timestamp,
        }
    }
}

/// route to download full reactions log for csv
pub async fn download_full_reactions_log_csv(db: web::Data<PgPool>) -> impl Responder {
    match reactions_log::get_full_reaction_log(&db).await {
        Ok(results) => {
            let mut wrt = csv::WriterBuilder::new().from_writer(vec![]);
            for item in results.into_iter() {
                if let Err(err) = wrt.serialize(ReactionLogRow::from(item)) {
                    println!("error serializing {:?}", err);
                    return HttpResponse::BadRequest().body(err.to_string());
                }
            }

            if let Ok(bytes) = wrt.into_inner() {
                HttpResponse::Ok().body(bytes)
            } else {
                HttpResponse::BadRequest().body("Could not parse csv")
            }
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

#[derive(Deserialize)]
pub struct PreBlastReactionReq {
    date: NaiveDate,
}

pub struct PreBlastPaxReactionData {
    date: NaiveDate,
    bds: HashMap<AO, HashMap<String, PaxReactionLogData>>,
}

impl PreBlastPaxReactionData {
    pub fn new(
        date: NaiveDate,
        log_items: Vec<PreBlastReactionLogItem>,
        bb_list: Vec<BackBlastData>,
    ) -> Self {
        let mut bds = HashMap::<AO, HashMap<String, PaxReactionLogData>>::new();

        for item in log_items.into_iter() {
            let ao = AO::from_channel_id(item.channel_id.as_str());
            bds.entry(ao.clone())
                .and_modify(|pax_map| {
                    pax_map
                        .entry(item.slack_user.to_string())
                        .and_modify(|pax_data| {
                            pax_data.reactions.push(PaxReactionLogItem::from(&item));
                        })
                        .or_insert(PaxReactionLogData::from(&item));
                })
                .or_insert(HashMap::from([(
                    item.slack_user.to_string(),
                    PaxReactionLogData::from(&item),
                )]));
        }

        // calculate fart-sackers
        for (ao, pax_items) in bds.iter_mut() {
            for pax in pax_items.values_mut() {
                pax.with_bb_data(ao, &bb_list);
            }
        }
        PreBlastPaxReactionData { date, bds }
    }

    pub fn full_summary(&self) -> String {
        let mut summary = format!("Summary for expected BDs on {}.\n\n", self.date);

        for (ao, pax_data) in self.bds.iter() {
            summary.push_str(ao.friendly_name());
            self.add_dashed_new_line(&mut summary);
            self.add_new_line(&mut summary);

            for user_data in pax_data.values() {
                summary.push_str(&format!("{}:\n\n", user_data.pax_name));

                if let Some(attended_bd) = user_data.attended_bd {
                    if !attended_bd {
                        summary.push_str("DID NOT ATTEND BD.... Fart-sack!\n\n");
                    }
                } else {
                    summary.push_str("BD backblast has not been posted yet.\n\n");
                }

                summary.push_str("Reactions log:\n\n");

                for reaction in user_data.reactions.iter() {
                    summary.push_str(&format!("{}:\n", reaction.reaction));
                    if let Some(first_added) = reaction.first_addition {
                        summary.push_str(&format!("first added {}.\n", first_added));
                    }

                    if reaction.removed {
                        summary.push_str(&format!("Removed {}", reaction.reaction));
                        if let Some(last_removed) = reaction.last_removal {
                            summary.push_str(&format!(" last time at {}.", last_removed));
                        }
                        self.add_new_line(&mut summary);
                    }

                    self.add_dashed_new_line(&mut summary);
                }
            }
        }

        summary
    }

    fn add_dashed_new_line(&self, summary: &mut String) {
        summary.push_str("\n--------------------\n");
    }

    fn add_new_line(&self, summary: &mut String) {
        summary.push('\n');
    }
}

pub struct PaxReactionLogData {
    pax_name: String,
    reactions: Vec<PaxReactionLogItem>,
    attended_bd: Option<bool>,
}

impl From<&PreBlastReactionLogItem> for PaxReactionLogData {
    fn from(value: &PreBlastReactionLogItem) -> Self {
        PaxReactionLogData {
            pax_name: value.name.to_string(),
            reactions: vec![PaxReactionLogItem::from(value)],
            attended_bd: None,
        }
    }
}

impl PaxReactionLogData {
    pub fn with_bb_data(&mut self, ao: &AO, bb_list: &[BackBlastData]) {
        if self.did_hc() {
            let matching_bb = bb_list.iter().find(|item| &item.ao == ao);
            if let Some(bb) = matching_bb {
                let attended = bb
                    .get_pax()
                    .iter()
                    .map(|p| p.to_lowercase())
                    .collect::<Vec<String>>()
                    .contains(&self.pax_name.to_lowercase());
                self.attended_bd = Some(attended);
            }
        }
    }

    fn did_hc(&self) -> bool {
        self.reactions.iter().any(|r| r.reaction.as_str() == "hc")
    }
}

pub struct PaxReactionLogItem {
    reaction: String,
    removed: bool,
    first_addition: Option<NaiveDateTime>,
    last_removal: Option<NaiveDateTime>,
}

impl From<&PreBlastReactionLogItem> for PaxReactionLogItem {
    fn from(value: &PreBlastReactionLogItem) -> Self {
        // sometimes data is off with slack
        let removed = if value.first_added_time.is_none() {
            false
        } else {
            !value.final_reaction_status
        };

        let first_addition = if value.first_added_time.is_none() {
            value.last_removal_time
        } else {
            value.first_added_time
        };

        let last_removal = if value.first_added_time.is_some() {
            value.last_removal_time
        } else {
            None
        };
        PaxReactionLogItem {
            reaction: value.reaction.to_string(),
            removed,
            first_addition,
            last_removal,
        }
    }
}

/// route for compiling some reaction data on a preblast date.
pub async fn pre_blast_reaction_data_route(
    db: web::Data<PgPool>,
    req: web::Query<PreBlastReactionReq>,
) -> impl Responder {
    match reactions_log::get_pre_blast_reaction_data(
        &db,
        req.date,
        vec!["hc".to_string(), "sc".to_string()],
    )
    .await
    {
        Ok(results) => {
            let related_back_blasts: Vec<BackBlastData> =
                all_back_blasts::get_all_within_date_range(&db, &req.date, &req.date)
                    .await
                    .unwrap_or_default()
                    .iter()
                    .map(BackBlastData::from)
                    .collect();

            let data = PreBlastPaxReactionData::new(req.date, results, related_back_blasts);

            HttpResponse::Ok().body(data.full_summary())
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
