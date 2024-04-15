use crate::app_state::ao_data::const_names::AO_LIST;
use crate::app_state::ao_data::AO;
use crate::db::queries::all_back_blasts::BackBlastJsonData;
use crate::db::queries::q_line_up::QLineUpDbData;
use crate::db::queries::{all_back_blasts, q_line_up};
use crate::shared::common_errors::AppError;
use crate::shared::time::local_boise_time;
use crate::slack_api::block_kit::BlockBuilder;
use crate::slack_api::chat::post_message::request::PostMessageRequest;
use crate::web_api_routes::auth::internal_auth;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::{Datelike, NaiveDate};
use sqlx::PgPool;

pub async fn remind_missing_back_blasts(
    db_pool: web::Data<PgPool>,
    web_state: web::Data<MutableWebState>,
    req: HttpRequest,
) -> impl Responder {
    if internal_auth::valid_internal_request(&req).is_ok() {
        let today = local_boise_time().date_naive();
        let yesterday = today.pred_opt().unwrap();
        match (
            yesterdays_bb(&db_pool, &yesterday, &today).await,
            yesterdays_signups(&db_pool, &yesterday, &today).await,
        ) {
            (Ok(bb), Ok(sign_ups)) => {
                let filtered: Vec<AO> = AO_LIST
                    .into_iter()
                    .filter(|ao| {
                        // if ao that meets on day yesterday
                        if ao.week_days().contains(&yesterday.weekday()) {
                            let posted = bb
                                .iter()
                                .find(|item| item.ao == ao.to_string() && item.date == yesterday);
                            let sign_up = sign_ups
                                .iter()
                                .find(|item| item.ao == ao.to_string() && item.date == yesterday);
                            let was_closed = sign_up.map(|data| data.closed).unwrap_or(false);
                            // if not posted and wasn't closed yesterday
                            posted.is_none() && !was_closed
                        } else {
                            false
                        }
                    })
                    .collect();

                for ao in filtered {
                    println!("missing BB for {:?}", ao.to_string());
                    let request = get_message_request(&ao);
                    match web_state.post_message(request).await {
                        Ok(_) => {}
                        Err(err) => println!("error calling slack: {:?}", err),
                    }
                }
                HttpResponse::Ok().body("success")
            }
            _ => HttpResponse::BadRequest().body("No data"),
        }
    } else {
        HttpResponse::Forbidden().body("Not authenticated")
    }
}

fn get_message_request(ao: &AO) -> PostMessageRequest {
    let block_builder = BlockBuilder::new()
        .section_markdown("*Reminder*")
        .section_markdown("Please don't forget to post back-blast from yesterday.");
    PostMessageRequest::new(ao.channel_id(), block_builder.blocks)
}

async fn yesterdays_bb(
    db: &PgPool,
    yesterday: &NaiveDate,
    today: &NaiveDate,
) -> Result<Vec<BackBlastJsonData>, AppError> {
    all_back_blasts::get_all_within_date_range(db, yesterday, today).await
}

async fn yesterdays_signups(
    db: &PgPool,
    yesterday: &NaiveDate,
    today: &NaiveDate,
) -> Result<Vec<QLineUpDbData>, AppError> {
    q_line_up::get_q_line_up_between_dates(db, yesterday, today).await
}
