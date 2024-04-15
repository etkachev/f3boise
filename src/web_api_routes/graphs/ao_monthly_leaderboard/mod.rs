use crate::app_state::ao_data::const_names::AO_LIST;
use crate::db::queries::all_back_blasts::{get_all_within_date_range, BackBlastJsonData};
use crate::shared::common_errors::AppError;
use crate::shared::time::local_boise_time;
use crate::slack_api::files::request::FileUploadRequest;
use crate::web_api_routes::graphs::{graph_generator, GraphWrapper};
use crate::web_api_state::MutableWebState;
use charts::BarLabelPosition;
use chrono::{Datelike, NaiveDate};
use sqlx::PgPool;
use std::time::Instant;

/// send monthly ao leaderboard graph
pub async fn get_ao_monthly_stats_graph(
    db_pool: &PgPool,
    date: &Option<NaiveDate>,
    web_state: &MutableWebState,
    channel_id: String,
) -> Result<(), AppError> {
    let default_end_date = local_boise_time().date_naive();
    let end_date = date.unwrap_or(default_end_date);
    let start_date = NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), 1).unwrap();
    let bb_list = get_all_within_date_range(db_pool, &start_date, &end_date).await?;
    let ao_monthly_stats = AoMonthlyStatsGraph::new(bb_list, start_date);
    let file = graph_generator(ao_monthly_stats)?;

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
    // std::fs::write("ao.png", file).unwrap();
    web_state.upload_file(file_request).await?;
    println!("upload file done: {:?}", start.elapsed());
    Ok(())
}

struct AoMonthlyStatsGraph {
    bb_list: Vec<BackBlastJsonData>,
    date: NaiveDate,
}

impl AoMonthlyStatsGraph {
    fn new(bb_list: Vec<BackBlastJsonData>, date: NaiveDate) -> Self {
        AoMonthlyStatsGraph { bb_list, date }
    }

    /// get chart data from back blasts to consume into charts. This is to get BD post avg.
    fn get_chart_data(&self) -> Vec<(String, f32, String)> {
        let aos: Vec<(String, f32, String)> = get_ao_list()
            .iter()
            .map(|ao| {
                let filtered = self.bb_list.iter().filter_map(|bb| {
                    if &bb.ao == ao {
                        Some(bb.pax.len())
                    } else {
                        None
                    }
                });
                let sum = filtered.sum::<usize>() as u16;
                let filtered = self.bb_list.iter().filter_map(|bb| {
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
}

impl GraphWrapper for AoMonthlyStatsGraph {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 800;

    fn generate_chart(&self) -> Result<(), String> {
        let width = self.width() as isize;
        let height = self.height() as isize;
        let (top, right, bottom, left) = (90, 40, 50, 120);

        let y = charts::ScaleBand::new()
            .set_domain(get_ao_list())
            .set_range(vec![height - top - bottom, 0])
            .set_inner_padding(0.1)
            .set_outer_padding(0.1);

        let x = charts::ScaleLinear::new()
            .set_domain(vec![0.0, 20.0])
            .set_range(vec![0, width - left - right]);

        let data = self.get_chart_data();

        let view = charts::HorizontalBarView::new()
            .set_x_scale(&x)
            .set_y_scale(&y)
            .set_colors(charts::Color::color_scheme_dark())
            .set_label_position(BarLabelPosition::Center)
            .set_label_rounding_precision(2)
            .load_data(&data)
            .unwrap();

        charts::Chart::new()
            .set_width(width)
            .set_height(height)
            .set_margins(top, right, bottom, left)
            .add_title(self.date.format("Monthly Stats for %b %Y").to_string())
            .add_view(&view)
            .add_axis_bottom(&x)
            .add_axis_left(&y)
            .add_bottom_axis_label("Avg Pax per BD")
            .save(self.file_path())
    }

    fn file_name(&self) -> String {
        format!(
            "ao-monthly-leaderboard-{}-{}",
            self.date.month(),
            self.date.year()
        )
    }
}

fn get_ao_list() -> Vec<String> {
    AO_LIST.map(|ao| ao.to_string()).to_vec()
}
