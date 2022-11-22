use crate::app_state::ao_data::const_names::AO_LIST;
use crate::db::queries::all_back_blasts::{get_all_within_date_range, BackBlastJsonData};
use crate::shared::common_errors::AppError;
use crate::shared::time::local_boise_time;
use crate::slack_api::files::request::FileUploadRequest;
use crate::web_api_state::MutableWebState;
use actix_web::{web, HttpResponse, Responder};
use chrono::{Datelike, NaiveDate};
use serde::Deserialize;
use sqlx::PgPool;
use std::time::Instant;

#[derive(Deserialize)]
pub struct MonthLeaderboardQuery {
    pub date: Option<NaiveDate>,
}

/// test route for posting monthly ao leaderboard to bot playground channel
pub async fn ao_monthly_leaderboard_route(
    db_pool: web::Data<PgPool>,
    query: web::Query<MonthLeaderboardQuery>,
    web_state: web::Data<MutableWebState>,
) -> impl Responder {
    match get_ao_monthly_stats_graph(
        &db_pool,
        &query.date,
        &web_state,
        String::from("C03TZV5RRF1"),
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().body("Saved"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

/// send monthly ao leaderboard graph
pub async fn get_ao_monthly_stats_graph(
    db_pool: &PgPool,
    date: &Option<NaiveDate>,
    web_state: &MutableWebState,
    channel_id: String,
) -> Result<(), AppError> {
    let default_end_date = local_boise_time().date_naive();
    let end_date = date.unwrap_or(default_end_date);
    let start_date = NaiveDate::from_ymd(end_date.year(), end_date.month(), 1);
    let bb_list = get_all_within_date_range(db_pool, &start_date, &end_date).await?;
    let file_name = format!(
        "ao-monthly-leaderboard-{}-{}.svg",
        start_date.month(),
        start_date.year()
    );
    let start = Instant::now();
    generate_chart_svg(&start_date, bb_list, file_name.as_str())?;
    println!("generate svg done: {:?}", start.elapsed());
    let start = Instant::now();
    let file = convert_svg(file_name.as_str())?;
    println!("conversion done: {:?}", start.elapsed());

    let file_request = FileUploadRequest::new(
        vec![channel_id],
        file,
        "monthly-leaderboard.png",
        start_date
            .format("Here are some stats for our AOs. Showing avg posts per BD for month of %b %Y")
            .to_string()
            .as_str(),
    );
    let start = Instant::now();
    web_state.upload_file(file_request).await?;
    println!("upload file done: {:?}", start.elapsed());
    let start = Instant::now();
    std::fs::remove_file(file_name)?;
    println!("deleted file: {:?}", start.elapsed());
    Ok(())
}

/// generate chart svg for ao backblast stats. TODO extract to trait method for other charts
fn generate_chart_svg(
    date: &NaiveDate,
    bb_list: Vec<BackBlastJsonData>,
    file_name: &str,
) -> Result<(), String> {
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let x = charts::ScaleBand::new()
        .set_domain(get_ao_list())
        .set_range(vec![0, width - left - right])
        .set_inner_padding(0.1)
        .set_outer_padding(0.1);

    let y = charts::ScaleLinear::new()
        .set_domain(vec![0.0, 20.0])
        .set_range(vec![height - top - bottom, 0]);

    let data = get_chart_data(&bb_list);

    let view = charts::VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_colors(charts::Color::color_scheme_dark())
        .set_label_rounding_precision(2)
        .load_data(&data)
        .unwrap();

    charts::Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(date.format("Monthly Stats for %b %Y").to_string())
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Avg Pax per BD")
        .add_bottom_axis_label("AOs")
        .save(file_name)
}

/// convert local svg file to png bytes. Can be used to upload to slack channels
fn convert_svg(file_name: &str) -> Result<Vec<u8>, AppError> {
    let file = std::fs::read(file_name)?;
    let mut options = resvg::usvg::Options::default();
    options.fontdb.load_system_fonts();
    options.fontdb.load_fonts_dir("./assets/fonts/");
    let tree = resvg::usvg::Tree::from_data(&file, &options.to_ref())?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(800, 600).unwrap();
    resvg::render(
        &tree,
        resvg::usvg::FitTo::Original,
        resvg::tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();
    // pixmap.save_png("test.png").unwrap();
    let result = pixmap.encode_png().unwrap();
    Ok(result)
}

fn get_ao_list() -> Vec<String> {
    AO_LIST.map(|ao| ao.to_string()).to_vec()
}

/// get chart data from back blasts to consume into charts. This is to get BD post avg.
fn get_chart_data(bb_list: &[BackBlastJsonData]) -> Vec<(String, f32, String)> {
    let aos: Vec<(String, f32, String)> = get_ao_list()
        .iter()
        .map(|ao| {
            let filtered = bb_list.iter().filter_map(|bb| {
                if &bb.ao == ao {
                    Some(bb.pax.len())
                } else {
                    None
                }
            });
            let sum = filtered.sum::<usize>() as u16;
            let filtered = bb_list.iter().filter_map(|bb| {
                if &bb.ao == ao {
                    Some(bb.pax.len())
                } else {
                    None
                }
            });
            let total = f32::from(filtered.count() as u16);
            let avg = if total == 0.0 {
                0.0
            } else {
                f32::from(sum) / total
            };
            (ao.to_string(), avg, ao.to_string())
        })
        .collect();
    aos
}
