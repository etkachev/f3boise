use crate::app_state::ao_data::AO;
use crate::app_state::backblast_data::BackBlastData;
use crate::db::queries::all_back_blasts::back_blasts_by_ao::back_blasts_by_channel_id_and_date_range;
use crate::db::queries::all_back_blasts::BackBlastJsonData;
use crate::shared::common_errors::AppError;
use crate::shared::time::local_boise_time;
use crate::slack_api::files::request::FileUploadRequest;
use crate::web_api_routes::graphs::{graph_generator, GraphWrapper};
use crate::web_api_state::MutableWebState;
use charts::BarLabelPosition;
use chrono::{Months, NaiveDate};
use sqlx::PgPool;
use std::collections::HashSet;
use std::ops::Sub;

/// post ao pax leaderboard graph
pub async fn post_ao_pax_leaderboard_graph(
    db_pool: &PgPool,
    web_state: &MutableWebState,
    channel_id: String,
) -> Result<(), AppError> {
    let ao = AO::from_channel_id(channel_id.as_str());
    let now = local_boise_time().date_naive();
    // TODO temp
    // let now = now.sub(Months::new(3));
    let thirty_days_ago = now.sub(Months::new(1));
    let bb = back_blasts_by_channel_id_and_date_range(
        db_pool,
        channel_id.as_str(),
        (thirty_days_ago, now),
    )
    .await?;
    let ao_name = ao.to_string();
    let graph = AoPaxGraph::new(ao, bb, (thirty_days_ago, now));
    let png = graph_generator(graph)?;
    let text = format!(
        "Here are top 10 PAX for {}. From {} to {}",
        ao_name, thirty_days_ago, now
    );
    let file_request =
        FileUploadRequest::new(vec![channel_id], png, "top-10-pax.png", text.as_str());
    web_state.upload_file(file_request).await?;
    // std::fs::write("pax.png", png)?;
    Ok(())
}

struct AoPaxGraph {
    ao: AO,
    bb: Vec<BackBlastData>,
    date_range: (NaiveDate, NaiveDate),
    pax: HashSet<String>,
}

impl AoPaxGraph {
    fn new(ao: AO, bb: Vec<BackBlastJsonData>, date_range: (NaiveDate, NaiveDate)) -> Self {
        let (bb, pax) = bb.iter().fold(
            (Vec::<BackBlastData>::new(), HashSet::<String>::new()),
            |mut acc, bb_item| {
                let bb_item = BackBlastData::from(bb_item);
                acc.1.extend(bb_item.get_pax());
                acc.0.push(bb_item);
                acc
            },
        );
        AoPaxGraph {
            ao,
            bb,
            date_range,
            pax,
        }
    }

    /// get pax and their post counts. Order by top posts and top 10.
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
            let top_top = &list[..10];
            top_top.to_vec()
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

impl GraphWrapper for AoPaxGraph {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 800;
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
            .set_domain(vec![0.0, 15.0])
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
            .add_title(format!("Top 10 PAX Posts for {}", self.ao))
            .add_view(&view)
            .add_axis_bottom(&x)
            .add_axis_left(&y)
            .add_bottom_axis_label("Posts")
            .save(self.file_path())
    }

    fn file_name(&self) -> String {
        format!(
            "ao-pax-leaderboard-{}-{}-{}",
            self.ao, self.date_range.0, self.date_range.1
        )
    }
}
