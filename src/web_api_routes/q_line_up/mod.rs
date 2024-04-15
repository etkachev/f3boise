use crate::app_state::ao_data::const_names::AO_LIST;
use crate::app_state::ao_data::AO;
use crate::db::queries::q_line_up::{
    get_q_line_up_between_dates, get_q_line_up_between_dates_for_ao, QLineUpDbData,
};
use crate::shared::common_errors::AppError;
use actix_web::{web, HttpResponse, Responder};
use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct QLineUpQuery {
    start: NaiveDate,
    end: NaiveDate,
    ao: Option<String>,
}

#[derive(Serialize)]
pub struct QLineUpResponse {
    ao: String,
    date: NaiveDate,
    closed: bool,
    text: Option<String>,
    qs: Option<Vec<String>>,
}

impl QLineUpResponse {
    fn new(ao: &str, date: NaiveDate, qs: Vec<String>) -> Self {
        QLineUpResponse {
            ao: ao.to_string(),
            date,
            closed: false,
            text: None,
            qs: Some(qs),
        }
    }
    fn new_closed(ao: &str, date: NaiveDate) -> Self {
        QLineUpResponse {
            ao: ao.to_string(),
            date,
            closed: true,
            text: Some(String::from("CLOSED")),
            qs: None,
        }
    }

    fn new_empty(ao: &str, date: NaiveDate) -> Self {
        QLineUpResponse {
            ao: ao.to_string(),
            date,
            closed: false,
            text: Some(String::from("EMPTY")),
            qs: None,
        }
    }
}

/// q line up list route.
pub async fn q_line_up_route(
    db: web::Data<PgPool>,
    query: web::Query<QLineUpQuery>,
) -> impl Responder {
    match q_line_up_results(&db, &query).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

/// get hashmap of q line up for all AOs within date range. Key being ao string name
pub async fn get_line_up_map(
    db_pool: &PgPool,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> Result<HashMap<String, Vec<QLineUpDbData>>, AppError> {
    let all = get_q_line_up_between_dates(db_pool, start_date, end_date).await?;
    let results = all.into_iter().fold(
        HashMap::<String, Vec<QLineUpDbData>>::new(),
        |mut acc, item| {
            if let Some(existing) = acc.get_mut(item.ao.as_str()) {
                existing.push(item);
            } else {
                acc.insert(item.ao.to_string(), vec![item]);
            }
            acc
        },
    );
    Ok(results)
}

/// get q line up results based on request.
async fn q_line_up_results(
    db: &PgPool,
    req: &QLineUpQuery,
) -> Result<Vec<QLineUpResponse>, AppError> {
    let ao = validate_request(req)?;

    let results = if let Some(ao) = ao {
        // for specific ao
        let results = get_q_line_up_between_dates_for_ao(db, &ao, &req.start, &req.end).await?;
        let mut response = Vec::<QLineUpResponse>::new();

        let mut current_date = req.start;
        let end_date = req.end;

        while current_date <= end_date {
            if ao.week_days().contains(&current_date.weekday()) {
                let ao_string = ao.to_string();
                let matching_entry =
                    get_q_line_up_item_from_list(&results, current_date, &ao_string);
                response.push(matching_entry);
            }
            current_date = current_date.succ_opt().unwrap();
        }

        response
    } else {
        // get for all AOs
        let results = get_line_up_map(db, &req.start, &req.end).await?;
        let mut response = Vec::<QLineUpResponse>::new();

        let mut current_date = req.start;
        let end_date = req.end;

        while current_date <= end_date {
            // loop through aos that are open on current date checked.
            for ao in AO_LIST
                .iter()
                .filter(|ao| ao.week_days().contains(&current_date.weekday()))
            {
                let ao_string = ao.to_string();
                let filtered = results.get(ao_string.as_str());
                let taken = filtered
                    .map(|list| {
                        // if found entry for ao and date.
                        get_q_line_up_item_from_list(list, current_date, &ao_string)
                    })
                    .unwrap_or_else(|| QLineUpResponse::new_empty(&ao_string, current_date));

                response.push(taken);
            }
            current_date = current_date.succ_opt().unwrap();
        }
        response
    };

    Ok(results)
}

fn get_q_line_up_item_from_list(
    list: &[QLineUpDbData],
    current_date: NaiveDate,
    ao_string: &str,
) -> QLineUpResponse {
    // if found entry for ao and date.
    let matching_entry = list.iter().find_map(|item| {
        if item.date == current_date {
            Some((item.qs.clone(), item.closed))
        } else {
            None
        }
    });

    match matching_entry {
        Some((_, true)) => QLineUpResponse::new_closed(ao_string, current_date),
        Some((qs, false)) => QLineUpResponse::new(ao_string, current_date, qs),
        None => QLineUpResponse::new_empty(ao_string, current_date),
    }
}

fn validate_request(req: &QLineUpQuery) -> Result<Option<AO>, AppError> {
    // validate dates
    let diff = NaiveDate::signed_duration_since(req.end, req.start);
    let diff_days = diff.num_days();
    if !(0..=365).contains(&diff_days) {
        return Err(AppError::General(
            "Date range cannot be more than a year.".to_string(),
        ));
    }

    // then validate AO
    if let Some(ao) = &req.ao {
        let ao = AO::from(ao.to_string());
        match ao {
            AO::Unknown(_) | AO::DR => return Err(AppError::General("Unknown AO".to_string())),
            _ => {}
        }
        Ok(Some(ao))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_non_ao() {
        let result = validate_request(&QLineUpQuery {
            start: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 2, 1).unwrap(),
            ao: None,
        })
        .unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn valid_ao() {
        let result = validate_request(&QLineUpQuery {
            start: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 2, 13).unwrap(),
            ao: Some(String::from("ao-rebel")),
        })
        .unwrap();

        assert_eq!(result, Some(AO::Rebel));
    }

    #[test]
    fn invalid_range() {
        let result = validate_request(&QLineUpQuery {
            start: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(),
            ao: None,
        });

        assert!(result.is_err());
    }

    #[test]
    fn end_before_start() {
        let result = validate_request(&QLineUpQuery {
            start: NaiveDate::from_ymd_opt(2022, 3, 2).unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 3, 1).unwrap(),
            ao: None,
        });

        assert!(result.is_err());
    }

    #[test]
    fn invalid_ao() {
        let result = validate_request(&QLineUpQuery {
            start: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
            end: NaiveDate::from_ymd_opt(2022, 2, 1).unwrap(),
            ao: Some(String::from("unknown")),
        });

        assert!(result.is_err());
    }
}
