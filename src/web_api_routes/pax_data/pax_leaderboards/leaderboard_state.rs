use crate::shared::processed_type::ProcessedType;
use chrono::{Datelike, NaiveDate};

#[derive(PartialEq, Debug, Clone)]
pub enum LeaderboardState {
    Hit(NumberOfBeatDowns),
    Approaching(NumberOfBeatDowns),
    /// approaching number of years at F3
    ApproachingBDBD(u16),
    /// number of years at F3
    HitBDBD(u16),
    None,
}

#[derive(PartialEq, Debug, Clone)]
pub enum NumberOfBeatDowns {
    Hundred(u16),
    Thousand(u16),
    Common(u16),
}

impl LeaderboardState {
    pub fn bds(count: usize) -> Self {
        let count = count as u16;

        match NumberOfBeatDowns::new(count) {
            NumberOfBeatDowns::Common(other) => match NumberOfBeatDowns::new(other + 1) {
                NumberOfBeatDowns::Common(_) => LeaderboardState::None,
                approaching => LeaderboardState::Approaching(approaching),
            },
            valid => LeaderboardState::Hit(valid),
        }
    }

    pub fn b_day(anniversary: NaiveDate, now: NaiveDate) -> Self {
        let today_year = now.year();

        let diff_years = (today_year - anniversary.year()) as u16;

        if diff_years == 0 {
            return LeaderboardState::None;
        }

        // Construct potential anniversary date for this year
        let this_years_anniversary =
            NaiveDate::from_ymd_opt(today_year, anniversary.month(), anniversary.day())
                .unwrap_or_default();

        // Calculate difference in days between today and this year's anniversary
        let days_until_anniversary = (this_years_anniversary - now).num_days();

        match days_until_anniversary {
            // if birthday bd is within 3 days
            1..=3 => LeaderboardState::ApproachingBDBD(diff_years),
            0 => LeaderboardState::HitBDBD(diff_years),
            _ => LeaderboardState::None,
        }
    }

    pub fn icon(&self) -> Option<&str> {
        match self {
            LeaderboardState::Approaching(bds) => bds.icon(),
            LeaderboardState::Hit(bds) => bds.icon(),
            LeaderboardState::ApproachingBDBD(_) | LeaderboardState::HitBDBD(_) => Some("🎂"),
            LeaderboardState::None => None,
        }
    }

    pub fn message(&self) -> Option<String> {
        let icon = self.icon().unwrap_or("");
        match self {
            LeaderboardState::Approaching(bds) => {
                Some(format!("{} - Approaching {} BDs", icon, bds.count()))
            }
            LeaderboardState::Hit(bds) => Some(format!("{} - Hit {} BDs", icon, bds.count())),
            LeaderboardState::ApproachingBDBD(year) => {
                Some(format!("{} - Approaching {} year anniversary", icon, year))
            }
            LeaderboardState::HitBDBD(year) => {
                Some(format!("{} - Today is {} year anniversary", icon, year))
            }
            LeaderboardState::None => None,
        }
    }
}

impl ProcessedType for LeaderboardState {
    fn get_type_id(&self) -> Option<String> {
        let prefix = "leaderboard.state";
        let id = match self {
            LeaderboardState::None => None,
            LeaderboardState::HitBDBD(year) => Some(format!("hit.anni.{}", year)),
            LeaderboardState::ApproachingBDBD(year) => Some(format!("approaching.anni.{}", year)),
            LeaderboardState::Hit(data) => {
                Some(format!("hit.bd.{}", data.get_type_id().unwrap_or_default()))
            }
            LeaderboardState::Approaching(data) => Some(format!(
                "approaching.bd.{}",
                data.get_type_id().unwrap_or_default()
            )),
        };

        id.map(|id| format!("{}.{}", prefix, id))
    }
}

impl NumberOfBeatDowns {
    pub fn new(count: u16) -> Self {
        if count == 0 {
            NumberOfBeatDowns::Common(0)
        } else {
            match count % 1000 {
                0 => NumberOfBeatDowns::Thousand(count / 1000),
                _ => match count % 100 {
                    0 => NumberOfBeatDowns::Hundred(count / 100),
                    _ => NumberOfBeatDowns::Common(count),
                },
            }
        }
    }

    pub fn count(&self) -> u16 {
        match self {
            NumberOfBeatDowns::Thousand(thousand) => thousand * 1_000,
            NumberOfBeatDowns::Hundred(hundred) => hundred * 100,
            NumberOfBeatDowns::Common(common) => *common,
        }
    }

    pub fn icon(&self) -> Option<&str> {
        match self {
            NumberOfBeatDowns::Common(_) => None,
            NumberOfBeatDowns::Thousand(_) => Some("👑"),
            NumberOfBeatDowns::Hundred(hundred) => match *hundred {
                1 => Some("🥳"),
                2 => Some("🔶"),
                3 => Some("🌟"),
                4 => Some("🏆"),
                5 => Some("🔷"),
                6 => Some("💠"),
                7 => Some("💎"),
                _ => Some("⚜️"),
            },
        }
    }
}

impl ProcessedType for NumberOfBeatDowns {
    fn get_type_id(&self) -> Option<String> {
        match self {
            NumberOfBeatDowns::Thousand(amt) => Some(format!("th.{}", amt)),
            NumberOfBeatDowns::Hundred(amt) => Some(format!("hnd.{}", amt)),
            NumberOfBeatDowns::Common(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_bd_valid() {
        let value = NumberOfBeatDowns::new(3_000);
        assert_eq!(value, NumberOfBeatDowns::Thousand(3));

        let value = NumberOfBeatDowns::new(200);
        assert_eq!(value, NumberOfBeatDowns::Hundred(2));
    }

    #[test]
    fn approaching_state() {
        let value = LeaderboardState::bds(99);
        assert_eq!(
            value,
            LeaderboardState::Approaching(NumberOfBeatDowns::Hundred(1))
        );

        let value = LeaderboardState::bds(2999);
        assert_eq!(
            value,
            LeaderboardState::Approaching(NumberOfBeatDowns::Thousand(3))
        );
    }

    #[test]
    fn number_conversion() {
        let value = NumberOfBeatDowns::new(1_000);
        assert_eq!(value.count(), 1_000);

        let value = NumberOfBeatDowns::new(328);
        assert_eq!(value.count(), 328);
    }

    #[test]
    fn approaching_bd_birthday() {
        let today = NaiveDate::from_ymd_opt(2024, 3, 12).unwrap();
        let bd_bd = NaiveDate::from_ymd_opt(2022, 3, 14).unwrap();
        let state = LeaderboardState::b_day(bd_bd, today);
        assert_eq!(state, LeaderboardState::ApproachingBDBD(2));
    }

    #[test]
    fn approaching_bd_message() {
        let state = LeaderboardState::bds(199);
        let message = state.message();
        assert_eq!(message.unwrap(), String::from("🔶 - Approaching 200 BDs"))
    }

    #[test]
    fn hit_bd_message() {
        let state = LeaderboardState::bds(300);
        let message = state.message();
        assert_eq!(message.unwrap(), String::from("🌟 - Hit 300 BDs"))
    }

    #[test]
    fn approaching_3year_anniversary() {
        let state = LeaderboardState::b_day(
            NaiveDate::from_ymd_opt(2021, 3, 14).unwrap(),
            NaiveDate::from_ymd_opt(2024, 3, 12).unwrap(),
        );
        let message = state.message();
        assert_eq!(
            message.unwrap(),
            String::from("🎂 - Approaching 3 year anniversary")
        )
    }

    #[test]
    fn hit_5year_anniversary() {
        let state = LeaderboardState::b_day(
            NaiveDate::from_ymd_opt(2019, 3, 14).unwrap(),
            NaiveDate::from_ymd_opt(2024, 3, 14).unwrap(),
        );
        let message = state.message();
        assert_eq!(
            message.unwrap(),
            String::from("🎂 - Today is 5 year anniversary")
        )
    }

    #[test]
    fn empty_message() {
        let state = LeaderboardState::bds(234);
        let message = state.message();
        assert_eq!(message, None);

        let state = LeaderboardState::b_day(
            NaiveDate::from_ymd_opt(2019, 3, 5).unwrap(),
            NaiveDate::from_ymd_opt(2024, 3, 14).unwrap(),
        );
        let message = state.message();
        assert_eq!(message, None);
    }

    #[test]
    fn handle_0_bds() {
        let bds = NumberOfBeatDowns::new(0);
        assert_eq!(bds, NumberOfBeatDowns::Common(0));
    }

    #[test]
    fn same_day_should_be_none() {
        let state = LeaderboardState::b_day(
            NaiveDate::from_ymd_opt(2024, 3, 14).unwrap(),
            NaiveDate::from_ymd_opt(2024, 3, 14).unwrap(),
        );

        assert_eq!(state.message(), None);
    }
}
