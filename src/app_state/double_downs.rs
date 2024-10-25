use chrono::NaiveDate;
use serde::Serialize;
use std::fmt::Display;
use std::ops::Range;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum DoubleDownProgram {
    WolfPax,
    KnightForge,
    ChainLinks,
    General,
}

impl DoubleDownProgram {
    pub fn date_range(&self) -> Range<NaiveDate> {
        match self {
            DoubleDownProgram::WolfPax => {
                NaiveDate::MIN..NaiveDate::from_ymd_opt(2023, 7, 9).unwrap()
            }
            DoubleDownProgram::KnightForge => {
                NaiveDate::from_ymd_opt(2023, 7, 9).unwrap()
                    ..NaiveDate::from_ymd_opt(2024, 7, 5).unwrap()
            }
            DoubleDownProgram::ChainLinks => {
                NaiveDate::from_ymd_opt(2024, 10, 3).unwrap()
                    ..NaiveDate::from_ymd_opt(2025, 10, 3).unwrap()
            }
            DoubleDownProgram::General => NaiveDate::MIN..NaiveDate::MAX,
        }
    }
}

impl Display for DoubleDownProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DoubleDownProgram::WolfPax => "WolfPax",
            DoubleDownProgram::KnightForge => "KnightForge",
            DoubleDownProgram::ChainLinks => "ChainLinks",
            DoubleDownProgram::General => "General",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

pub const PROGRAM_LIST: [DoubleDownProgram; 4] = [
    DoubleDownProgram::ChainLinks,
    DoubleDownProgram::KnightForge,
    DoubleDownProgram::WolfPax,
    DoubleDownProgram::General,
];

impl From<&NaiveDate> for DoubleDownProgram {
    fn from(value: &NaiveDate) -> Self {
        for program in PROGRAM_LIST {
            if program.date_range().contains(value) {
                return program;
            }
        }

        // default
        DoubleDownProgram::General
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_forge_date() {
        let date = NaiveDate::from_ymd_opt(2023, 7, 9).unwrap();
        let program = DoubleDownProgram::from(&date);
        assert_eq!(program, DoubleDownProgram::KnightForge);
        let end = NaiveDate::from_ymd_opt(2024, 7, 4).unwrap();
        let program = DoubleDownProgram::from(&end);
        assert_eq!(program, DoubleDownProgram::KnightForge);
    }

    #[test]
    fn test_general() {
        let date = NaiveDate::from_ymd_opt(2024, 7, 6).unwrap();
        let program = DoubleDownProgram::from(&date);
        assert_eq!(program, DoubleDownProgram::General);
    }

    #[test]
    fn test_chain_links() {
        let date = NaiveDate::from_ymd_opt(2024, 10, 3).unwrap();
        let program = DoubleDownProgram::from(&date);
        assert_eq!(program, DoubleDownProgram::ChainLinks);
    }
}
