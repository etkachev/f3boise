use crate::db::queries::all_back_blasts;
use crate::db::queries::all_back_blasts::BackBlastFullJsonData;
use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize, Debug)]
pub struct BackBlastRow {
    pub ao: String,
    /// flat comma separated list
    pub q: String,
    /// flat comma separated list
    pub pax: String,
    /// flat comma separated list
    pub fngs: String,
    pub date: NaiveDate,
    pub title: String,
    pub moleskine: String,
}

impl From<BackBlastFullJsonData> for BackBlastRow {
    fn from(row: BackBlastFullJsonData) -> Self {
        BackBlastRow {
            ao: row.ao,
            q: row.q.join(","),
            pax: row.pax.join(","),
            date: row.date,
            fngs: row.fngs.map(|items| items.join(",")).unwrap_or_default(),
            title: row.title.unwrap_or_default(),
            moleskine: row.moleskine.unwrap_or_default(),
        }
    }
}

pub async fn download_back_blasts_csv_route(db_pool: web::Data<PgPool>) -> impl Responder {
    match all_back_blasts::get_full_db_back_blasts(&db_pool).await {
        Ok(bb_data) => {
            let mut wrt = csv::WriterBuilder::new().from_writer(vec![]);
            for bb in bb_data.into_iter() {
                if let Err(err) = wrt.serialize(BackBlastRow::from(bb)) {
                    println!("Error serializing data: {:?}", err);
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

pub async fn back_blasts_csv_html() -> impl Responder {
    let html = format!(
        r#"
        <html>
        <head><title>Download BackBlast data</title></head>
        <script>
        function downloadFile() {{
            var aLink = document.createElement('a');
            var evt = document.createEvent("HTMLEvents");
            evt.initEvent("click");
            var currentUrl = new URL(window.location);
            var requestUrl = currentUrl.origin + "/back_blasts/download-csv" + currentUrl.search;
            aLink.href = requestUrl;
            aLink.download = "f3-boise.csv";
            aLink.click(evt);
        }}
        downloadFile();
        </script>
        <body>
            {}
        </body>
    </html>
        "#,
        "Requesting Download"
    );
    HttpResponse::Ok().body(html)
}
