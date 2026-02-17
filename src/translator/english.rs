use super::{TimeTranslator, FuzzinessLevel};
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
    
    fn hour_unit(n: u32) -> &'static str {
        if n == 1 { "hour" } else { "hours" }
    }
    
    fn minute_unit(n: u32) -> &'static str {
        if n == 1 { "minute" } else { "minutes" }
    }
}

impl TimeTranslator for EnglishTranslator {
    fn translate(&self, time: &TimeInfo, level: FuzzinessLevel, use_24h: bool, include_units: bool) -> String {
        match level {
            FuzzinessLevel::Exact => self.translate_exact(time, use_24h, include_units),
            FuzzinessLevel::Fuzzy => self.translate_fuzzy(time, use_24h, include_units),
            FuzzinessLevel::VeryFuzzy => self.translate_very_fuzzy(time, use_24h, include_units),
            FuzzinessLevel::MaxFuzzy => self.translate_max_fuzzy(time),
        }
    }
}

impl EnglishTranslator {
    fn translate_exact(&self, time: &TimeInfo, use_24h: bool, include_units: bool) -> String {
        if use_24h {
            let hour_word = Self::number_to_word(time.hour24);
            let minute_word = Self::format_minute(time.minute);
            if include_units {
                format!("{} {} {} {}", 
                    hour_word, Self::hour_unit(time.hour24),
                    minute_word, Self::minute_unit(time.minute))
            } else {
                format!("{} {}", hour_word, minute_word)
            }
        } else {
            let hour_word = Self::number_to_word(time.hour);
            let minute_word = Self::format_minute(time.minute);
            let period = if time.is_pm { "PM" } else { "AM" };
            if include_units {
                format!("{} {} {} {} {}", 
                    hour_word, Self::hour_unit(time.hour),
                    minute_word, Self::minute_unit(time.minute), period)
            } else {
                format!("{} {} {}", hour_word, minute_word, period)
            }
        }
    }
    
    fn translate_fuzzy(&self, time: &TimeInfo, use_24h: bool, include_units: bool) -> String {
        let hour = if use_24h { time.hour24 } else { time.hour };
        let period = if use_24h { "" } else if time.is_pm { " PM" } else { " AM" };
        let hour_unit_str = if include_units { format!(" {}", Self::hour_unit(hour)) } else { String::new() };
        
        match time.minute {
            0 => format!("{} o'clock", Self::number_to_word(hour)),
            15 => format!("quarter past {}{}{}", Self::number_to_word(hour), hour_unit_str, period),
            30 => format!("half past {}{}{}", Self::number_to_word(hour), hour_unit_str, period),
            45 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                let next_unit_str = if include_units { format!(" {}", Self::hour_unit(next_hour)) } else { String::new() };
                format!("quarter to {}{}{}", Self::number_to_word(next_hour), next_unit_str, period)
            },
            1..=7 => format!("{} past {}{}{}", Self::number_to_word(time.minute), Self::number_to_word(hour), hour_unit_str, period),
            8..=14 => format!("about quarter past {}{}{}", Self::number_to_word(hour), hour_unit_str, period),
            16..=22 => format!("about twenty past {}{}{}", Self::number_to_word(hour), hour_unit_str, period),
            23..=29 => format!("almost half past {}{}{}", Self::number_to_word(hour), hour_unit_str, period),
            31..=37 => format!("about half past {}{}{}", Self::number_to_word(hour), hour_unit_str, period),
            38..=44 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                let next_unit_str = if include_units { format!(" {}", Self::hour_unit(next_hour)) } else { String::new() };
                format!("almost quarter to {}{}{}", Self::number_to_word(next_hour), next_unit_str, period)
            },
            46..=52 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                let next_unit_str = if include_units { format!(" {}", Self::hour_unit(next_hour)) } else { String::new() };
                format!("about quarter to {}{}{}", Self::number_to_word(next_hour), next_unit_str, period)
            },
            _ => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                format!("almost {} o'clock", Self::number_to_word(next_hour))
            }
        }
    }
    
    fn translate_very_fuzzy(&self, time: &TimeInfo, use_24h: bool, include_units: bool) -> String {
        let hour = if use_24h { time.hour24 } else { time.hour };
        let hour_unit_str = if include_units { format!(" {}", Self::hour_unit(hour)) } else { String::new() };
        
        match time.minute {
            0..=7 => format!("{}{} o'clock", Self::number_to_word(hour), hour_unit_str),
            8..=22 => format!("about quarter past {}{}", Self::number_to_word(hour), hour_unit_str),
            23..=37 => format!("about half past {}{}", Self::number_to_word(hour), hour_unit_str),
            38..=52 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                let next_unit_str = if include_units { format!(" {}", Self::hour_unit(next_hour)) } else { String::new() };
                format!("about quarter to {}{}", Self::number_to_word(next_hour), next_unit_str)
            },
            _ => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
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
