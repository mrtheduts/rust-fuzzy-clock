use super::{TimeTranslator, FuzzynessLevel};
use crate::time::TimeInfo;

pub struct EnglishTranslator;

impl EnglishTranslator {
    fn number_to_word(n: u32) -> &'static str {
        match n {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            20 => "twenty",
            21 => "twenty-one",
            22 => "twenty-two",
            23 => "twenty-three",
            24 => "twenty-four",
            25 => "twenty-five",
            26 => "twenty-six",
            27 => "twenty-seven",
            28 => "twenty-eight",
            29 => "twenty-nine",
            30 => "thirty",
            31 => "thirty-one",
            32 => "thirty-two",
            33 => "thirty-three",
            34 => "thirty-four",
            35 => "thirty-five",
            36 => "thirty-six",
            37 => "thirty-seven",
            38 => "thirty-eight",
            39 => "thirty-nine",
            40 => "forty",
            41 => "forty-one",
            42 => "forty-two",
            43 => "forty-three",
            44 => "forty-four",
            45 => "forty-five",
            46 => "forty-six",
            47 => "forty-seven",
            48 => "forty-eight",
            49 => "forty-nine",
            50 => "fifty",
            51 => "fifty-one",
            52 => "fifty-two",
            53 => "fifty-three",
            54 => "fifty-four",
            55 => "fifty-five",
            56 => "fifty-six",
            57 => "fifty-seven",
            58 => "fifty-eight",
            59 => "fifty-nine",
            _ => "unknown",
        }
    }
    
    fn format_minute(minute: u32) -> String {
        if minute < 10 {
            format!("oh {}", Self::number_to_word(minute))
        } else {
            Self::number_to_word(minute).to_string()
        }
    }
}

impl TimeTranslator for EnglishTranslator {
    fn translate(&self, time: &TimeInfo, level: FuzzynessLevel) -> String {
        match level {
            FuzzynessLevel::Exact => self.translate_exact(time),
            FuzzynessLevel::Fuzzy => self.translate_fuzzy(time),
            FuzzynessLevel::VeryFuzzy => self.translate_very_fuzzy(time),
            FuzzynessLevel::MaxFuzzy => self.translate_max_fuzzy(time),
        }
    }
}

impl EnglishTranslator {
    fn translate_exact(&self, time: &TimeInfo) -> String {
        let hour_word = Self::number_to_word(time.hour);
        let minute_word = Self::format_minute(time.minute);
        let period = if time.is_pm { "PM" } else { "AM" };
        
        format!("{} {} {}", hour_word, minute_word, period)
    }
    
    fn translate_fuzzy(&self, time: &TimeInfo) -> String {
        let period = if time.is_pm { "PM" } else { "AM" };
        
        match time.minute {
            0 => format!("{} o'clock", Self::number_to_word(time.hour)),
            15 => format!("quarter past {} {}", Self::number_to_word(time.hour), period),
            30 => format!("half past {} {}", Self::number_to_word(time.hour), period),
            45 => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("quarter to {} {}", Self::number_to_word(next_hour), period)
            },
            1..=7 => format!("{} past {} {}", Self::number_to_word(time.minute), Self::number_to_word(time.hour), period),
            8..=14 => format!("about quarter past {} {}", Self::number_to_word(time.hour), period),
            16..=22 => format!("about twenty past {} {}", Self::number_to_word(time.hour), period),
            23..=29 => format!("almost half past {} {}", Self::number_to_word(time.hour), period),
            31..=37 => format!("about half past {} {}", Self::number_to_word(time.hour), period),
            38..=44 => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("almost quarter to {} {}", Self::number_to_word(next_hour), period)
            },
            46..=52 => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("about quarter to {} {}", Self::number_to_word(next_hour), period)
            },
            _ => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("almost {} o'clock", Self::number_to_word(next_hour))
            }
        }
    }
    
    fn translate_very_fuzzy(&self, time: &TimeInfo) -> String {
        match time.minute {
            0..=7 => format!("{} o'clock", Self::number_to_word(time.hour)),
            8..=22 => format!("about quarter past {}", Self::number_to_word(time.hour)),
            23..=37 => format!("about half past {}", Self::number_to_word(time.hour)),
            38..=52 => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("about quarter to {}", Self::number_to_word(next_hour))
            },
            _ => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("almost {} o'clock", Self::number_to_word(next_hour))
            }
        }
    }
    
    fn translate_max_fuzzy(&self, time: &TimeInfo) -> String {
        let hour24 = if time.is_pm && time.hour != 12 {
            time.hour + 12
        } else if !time.is_pm && time.hour == 12 {
            0
        } else {
            time.hour
        };
        
        match hour24 {
            5..=11 => "morning".to_string(),
            12..=16 => "afternoon".to_string(),
            17..=21 => "evening".to_string(),
            _ => "night".to_string(),
        }
    }
}
