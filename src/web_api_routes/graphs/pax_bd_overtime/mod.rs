//! Line Graph for pax BDs over time

use crate::db::queries::all_back_blasts::{get_all_within_date_range, BackBlastJsonData};
use crate::shared::common_errors::AppError;
use crate::shared::string_utils::floor_ceiling_date_range;
use crate::web_api_routes::graphs::{graph_generator, GraphWrapper};
use charts::{MarkerType, PointLabelPosition};
use chrono::{Datelike, Months, NaiveDate};
use sqlx::PgPool;
use std::ops::Add;

/// post line graph for pax bds over time
pub async fn post_pax_bd_overtime(db_pool: &PgPool, date_range: &str) -> Result<(), AppError> {
    let (start_date, end_date) = floor_ceiling_date_range(date_range, 12);
    let bb_list = get_all_within_date_range(db_pool, &start_date, &end_date).await?;
    let pax_over_time = PaxBdsOverTime::new(bb_list, &start_date, &end_date);
    let file = graph_generator(pax_over_time)?;
    // TODO send to slack?
    std::fs::write("test.png", file).unwrap();
    Ok(())
}

struct PaxBdMonth {
    /// (year, month)
    month: (i32, u32),
    pax_count: usize,
}

struct PaxBdsOverTime {
    data: Vec<PaxBdMonth>,
    start: NaiveDate,
    end: NaiveDate,
}

impl PaxBdMonth {
    pub fn new(date: &NaiveDate, pax_count: usize) -> Self {
        PaxBdMonth {
            month: (date.year(), date.month()),
            pax_count,
        }
    }
}

impl PaxBdsOverTime {
    pub fn new(bb_list: Vec<BackBlastJsonData>, start: &NaiveDate, end: &NaiveDate) -> Self {
        let mut results: Vec<PaxBdMonth> = vec![];
        let mut current_month = *start;
        let end = *end;

        while current_month <= end {
            let (year, month) = (current_month.year(), current_month.month());
            let total = bb_list.iter().fold(0usize, |mut acc, bb| {
                let bb_date = bb.date;
                let (bb_year, bb_month) = (bb_date.year(), bb_date.month());
                if bb_year == year && bb_month == month {
                    acc += bb.pax.len();
                }
                acc
            });

            results.push(PaxBdMonth::new(&current_month, total));
            current_month = current_month.add(Months::new(1));
        }

        PaxBdsOverTime {
            data: results,
            start: *start,
            end,
        }
    }

    fn get_max_pax_participation(&self) -> f32 {
        self.data
            .iter()
            .map(|item| item.pax_count)
            .max()
            .unwrap_or(0) as f32
    }

    fn get_chart_data(&self) -> Vec<(String, f32)> {
        self.data
            .iter()
            .map(|item| {
                let key = format_date(&item.month);
                (key, item.pax_count as f32)
            })
            .collect()
    }

    fn get_dates(&self) -> Vec<String> {
        self.data
            .iter()
            .map(|item| format_date(&item.month))
            .collect()
    }
}

fn format_date(month: &(i32, u32)) -> String {
    let (year, month) = month;
    format!("{}/{}", month, year)
}

impl GraphWrapper for PaxBdsOverTime {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    fn generate_chart(&self) -> Result<(), String> {
        let width = self.width() as isize;
        let height = self.height() as isize;
        let (top, right, bottom, left) = (90, 40, 50, 120);

        let y = charts::ScaleLinear::new()
            .set_domain(vec![0.0, self.get_max_pax_participation()])
            .set_range(vec![height - top - bottom, 0]);

        let x = charts::ScaleBand::new()
            .set_domain(self.get_dates())
            .set_range(vec![0, width - left - right])
            .set_inner_padding(0.1)
            .set_outer_padding(0.1);

        let view = charts::LineSeriesView::new()
            .set_x_scale(&x)
            .set_y_scale(&y)
            .set_colors(charts::Color::color_scheme_dark())
            .set_marker_type(MarkerType::Circle)
            .set_label_position(PointLabelPosition::N)
            .set_label_visibility(false)
            .load_data(&self.get_chart_data())
            .unwrap();

        charts::Chart::new()
            .set_width(width)
            .set_height(height)
            .set_margins(top, right, bottom, left)
            .add_title("Pax BDs Overtime".to_string())
            .add_view(&view)
            .add_axis_bottom(&x)
            .add_axis_left(&y)
            .save(self.file_path())
    }

    fn file_name(&self) -> String {
        format!(
            "pax-bds-per-month-{}-{}-{}-{}",
            self.start.month(),
            self.start.year(),
            self.end.month(),
            self.end.year(),
        )
    }
}
