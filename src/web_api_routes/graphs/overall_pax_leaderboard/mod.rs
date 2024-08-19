use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::{
    get_all_dd_within_date_range, get_all_within_date_range, BackBlastJsonData,
};
use crate::shared::common_errors::AppError;
use crate::shared::string_utils::resolve_date_range;
use crate::slack_api::files::request::FileUpload;
use crate::web_api_routes::graphs::{graph_generator, GraphWrapper};
use crate::web_api_state::MutableWebState;
use charts::BarLabelPosition;
use chrono::NaiveDate;
use sqlx::PgPool;
use std::collections::HashSet;

/// post overall pax leaderboard graph
pub async fn post_overall_pax_leaderboard_graph(
    db_pool: &PgPool,
    web_state: &MutableWebState,
    channel_id: String,
    date_range_text: &str,
) -> Result<(), AppError> {
    let (start, end) = resolve_date_range(date_range_text, 1);
    let back_blasts = get_all_within_date_range(db_pool, &start, &end).await?;
    let graph = OverallPaxGraph::new(back_blasts, (start, end));
    let png = graph_generator(graph)?;
    let start_formatted = friendly_date(start);
    let end_formatted = friendly_date(end);
    let text = format!(
        "Here are top 10 PAX overall. From {} to {}",
        start_formatted, end_formatted
    );

    let file_request =
        FileUpload::new(&channel_id, png, "top-10-pax-overall.png", "image/png").with_title(&text);

    web_state.upload_file(file_request).await?;
    Ok(())
}

pub async fn post_overall_pax_dd_leaderboard_graph(
    db_pool: &PgPool,
    web_state: &MutableWebState,
    channel_id: String,
    date_range_text: &str,
) -> Result<(), AppError> {
    let (start, end) = resolve_date_range(date_range_text, 1);
    let dd_back_blasts = get_all_dd_within_date_range(db_pool, &start, &end).await?;
    let graph = OverallPaxGraph::new(dd_back_blasts, (start, end));
    let png = graph_generator(graph)?;
    let start_formatted = friendly_date(start);
    let end_formatted = friendly_date(end);
    let text = format!(
        "Here are top 10 DD PAX overall. From {} to {}",
        start_formatted, end_formatted
    );

    let file_request = FileUpload::new(&channel_id, png, "top-10-dd-pax-overall.png", "image/png")
        .with_title(&text);

    web_state.upload_file(file_request).await?;

    // std::fs::write("dd.png", png)?;
    Ok(())
}

/// TODO util?
fn friendly_date(date: NaiveDate) -> String {
    date.format("%b %d, %Y").to_string()
}

struct OverallPaxGraph {
    bb: Vec<BackBlastData>,
    pax: HashSet<String>,
    date_range: (NaiveDate, NaiveDate),
}

impl OverallPaxGraph {
    fn new(bb: Vec<BackBlastJsonData>, date_range: (NaiveDate, NaiveDate)) -> Self {
        let (bb, pax) = bb.iter().fold(
            (Vec::<BackBlastData>::new(), HashSet::<String>::new()),
            |mut acc, bb_item| {
                let bb_item = BackBlastData::from(bb_item);
                acc.1.extend(bb_item.get_pax());
                acc.0.push(bb_item);
                acc
            },
        );
        OverallPaxGraph {
            bb,
            pax,
            date_range,
        }
    }

    fn get_data(&self) -> Vec<(String, f32, String)> {
        let mut list: Vec<(String, f32, String)> = self
            .get_pax_list()
            .iter()
            .map(|name| {
                let bb_with_pax = self
                    .bb
                    .iter()
                    .filter(|bb_item| bb_item.includes_pax(name))
                    .count() as u16;
                (name.to_string(), f32::from(bb_with_pax), name.to_string())
            })
            .collect();

        list.sort_by(|(_, a, _), (_, b, _)| b.partial_cmp(a).unwrap());
        if list.len() > 10 {
            let top_ten = &list[..10];
            top_ten.to_vec()
        } else {
            list
        }
    }
    fn get_pax_list(&self) -> Vec<String> {
        self.pax
            .iter()
            .map(|name| name.to_string())
            .collect::<Vec<String>>()
    }
}

impl GraphWrapper for OverallPaxGraph {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    fn generate_chart(&self) -> Result<(), String> {
        let width = self.width() as isize;
        let height = self.height() as isize;
        let (top, right, bottom, left) = (90, 40, 50, 80);

        let data = self.get_data();
        let domain: Vec<String> = data.iter().map(|(name, ..)| name.to_string()).collect();

        let y = charts::ScaleBand::new()
            .set_domain(domain)
            .set_range(vec![height - top - bottom, 0])
            .set_inner_padding(0.1)
            .set_outer_padding(0.1);

        let x = charts::ScaleLinear::new()
            .set_domain(vec![0.0, 25.0])
            .set_range(vec![0, width - left - right]);

        let view = charts::HorizontalBarView::new()
            .set_x_scale(&x)
            .set_y_scale(&y)
            .set_colors(charts::Color::color_scheme_dark())
            .set_label_position(BarLabelPosition::Center)
            .load_data(&data)
            .unwrap();

        charts::Chart::new()
            .set_width(width)
            .set_height(height)
            .set_margins(top, right, bottom, left)
            .add_title("Top 10 PAX Posts overall".to_string())
            .add_view(&view)
            .add_axis_bottom(&x)
            .add_axis_left(&y)
            .add_bottom_axis_label("Posts")
            .save(self.file_path())
    }

    fn file_name(&self) -> String {
        format!(
            "overall-pax-leaderboard-{}-{}",
            self.date_range.0, self.date_range.1
        )
    }
}
