use chrono::NaiveDate;
use std::ops::Range;

#[derive(PartialEq, Debug)]
pub enum DoubleDownProgram {
    WolfPax,
    KnightForge,
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
        }
    }
}

impl ToString for DoubleDownProgram {
    fn to_string(&self) -> String {
        match self {
            DoubleDownProgram::WolfPax => "WolfPax",
            DoubleDownProgram::KnightForge => "KnightForge",
        }
        .to_string()
    }
}

const PROGRAM_LIST: [DoubleDownProgram; 2] =
    [DoubleDownProgram::KnightForge, DoubleDownProgram::WolfPax];

impl From<&NaiveDate> for DoubleDownProgram {
    fn from(value: &NaiveDate) -> Self {
        for program in PROGRAM_LIST {
            if program.date_range().contains(value) {
                return program;
            }
        }

        // default
        DoubleDownProgram::WolfPax
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
}
