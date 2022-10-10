use crate::db::queries::all_back_blasts;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn download_back_blasts_csv_route(db_pool: web::Data<PgPool>) -> impl Responder {
    match all_back_blasts::get_all(&db_pool).await {
        Ok(bb_data) => {
            let mut wrt = csv::WriterBuilder::new().from_writer(vec![]);
            for bb in bb_data.into_iter() {
                if let Err(err) = wrt.serialize(bb) {
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
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">{}</button>
            </form>
        </body>
    </html>
        "#,
        "Submit"
    );
    HttpResponse::Ok().body(html)
}
