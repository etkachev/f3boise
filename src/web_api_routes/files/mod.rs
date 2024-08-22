use crate::shared::time::local_boise_time;
use crate::slack_api::files::files_list::request::FilesListRequest;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};

/// testing getting files from slack
pub async fn get_files_test(web_state: web::Data<MutableWebState>) -> impl Responder {
    let date = local_boise_time();
    let _date = date
        .checked_sub_signed(chrono::Duration::days(30))
        .expect("Couldn't remove days");
    let request = FilesListRequest::new(10_000);
    match web_state.get_files(request).await {
        Ok(files) => {
            let total_files = files.len();
            let size_usage = files.iter().fold(0, |acc, item| acc + item.size);
            println!("tf: {total_files} ---- usage: {size_usage}");
            HttpResponse::Ok().json(files)
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}
