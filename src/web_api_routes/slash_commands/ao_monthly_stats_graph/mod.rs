use crate::shared::time::local_boise_time;
use chrono::{Datelike, Months, NaiveDate};
use std::ops::Add;

/// command struct for ao monthly stats slash command
pub struct AOMonthlyStatsGraphCommand {
    /// parsed from format: "2022/08" to signify which month you want to
    pub month: NaiveDate,
}

impl AOMonthlyStatsGraphCommand {
    /// new command wrapper with passed in date from form. If invalid or empty, then defaults to local time
    pub fn new(form_text: &str) -> Self {
        let date = NaiveDate::parse_from_str(format!("{}/1", form_text).as_str(), "%Y/%m/%d")
            .unwrap_or_else(|_| local_boise_time().date_naive());
        let next_month = date.add(Months::new(1));
        let end_of_month = NaiveDate::from_ymd(next_month.year(), next_month.month(), 1).pred();
        AOMonthlyStatsGraphCommand {
            month: end_of_month,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AOMonthlyStatsGraphCommand;
    use chrono::NaiveDate;

    #[test]
    fn date_format_works() {
        let command = AOMonthlyStatsGraphCommand::new("2022/08");
        assert_eq!(command.month, NaiveDate::from_ymd(2022, 8, 31));
    }
}
