use chrono::NaiveDate;
use std::fmt::Display;
use std::ops::Range;

#[derive(PartialEq, Debug)]
pub enum DoubleDownProgram {
    WolfPax,
    KnightForge,
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
            DoubleDownProgram::General => NaiveDate::MIN..NaiveDate::MAX,
        }
    }
}

impl Display for DoubleDownProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DoubleDownProgram::WolfPax => "WolfPax",
            DoubleDownProgram::KnightForge => "KnightForge",
            DoubleDownProgram::General => "General",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

const PROGRAM_LIST: [DoubleDownProgram; 3] = [
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
}
