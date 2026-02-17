use super::{TimeTranslator, FuzzynessLevel};
use crate::time::TimeInfo;

pub struct SpanishTranslator;

impl SpanishTranslator {
    fn number_to_word(n: u32) -> &'static str {
        match n {
            0 => "cero",
            1 => "una",
            2 => "dos",
            3 => "tres",
            4 => "cuatro",
            5 => "cinco",
            6 => "seis",
            7 => "siete",
            8 => "ocho",
            9 => "nueve",
            10 => "diez",
            11 => "once",
            12 => "doce",
            13 => "trece",
            14 => "catorce",
            15 => "quince",
            16 => "dieciséis",
            17 => "diecisiete",
            18 => "dieciocho",
            19 => "diecinueve",
            20 => "veinte",
            21 => "veintiuno",
            22 => "veintidós",
            23 => "veintitrés",
            24 => "veinticuatro",
            25 => "veinticinco",
            26 => "veintiséis",
            27 => "veintisiete",
            28 => "veintiocho",
            29 => "veintinueve",
            30 => "treinta",
            31 => "treinta y uno",
            32 => "treinta y dos",
            33 => "treinta y tres",
            34 => "treinta y cuatro",
            35 => "treinta y cinco",
            36 => "treinta y seis",
            37 => "treinta y siete",
            38 => "treinta y ocho",
            39 => "treinta y nueve",
            40 => "cuarenta",
            41 => "cuarenta y uno",
            42 => "cuarenta y dos",
            43 => "cuarenta y tres",
            44 => "cuarenta y cuatro",
            45 => "cuarenta y cinco",
            46 => "cuarenta y seis",
            47 => "cuarenta y siete",
            48 => "cuarenta y ocho",
            49 => "cuarenta y nueve",
            50 => "cincuenta",
            51 => "cincuenta y uno",
            52 => "cincuenta y dos",
            53 => "cincuenta y tres",
            54 => "cincuenta y cuatro",
            55 => "cincuenta y cinco",
            56 => "cincuenta y seis",
            57 => "cincuenta y siete",
            58 => "cincuenta y ocho",
            59 => "cincuenta y nueve",
            _ => "desconocido",
        }
    }
    
    fn hour_word(n: u32) -> &'static str {
        if n == 1 { "una" } else { Self::number_to_word(n) }
    }
    
    fn format_minute(minute: u32) -> String {
        if minute < 10 {
            format!("cero {}", Self::number_to_word(minute))
        } else {
            Self::number_to_word(minute).to_string()
        }
    }
}

impl TimeTranslator for SpanishTranslator {
    fn translate(&self, time: &TimeInfo, level: FuzzynessLevel) -> String {
        match level {
            FuzzynessLevel::Exact => self.translate_exact(time),
            FuzzynessLevel::Fuzzy => self.translate_fuzzy(time),
            FuzzynessLevel::VeryFuzzy => self.translate_very_fuzzy(time),
            FuzzynessLevel::MaxFuzzy => self.translate_max_fuzzy(time),
        }
    }
}

impl SpanishTranslator {
    fn translate_exact(&self, time: &TimeInfo) -> String {
        let hour_word = Self::hour_word(time.hour);
        let minute_word = Self::format_minute(time.minute);
        let period = if time.is_pm { "PM" } else { "AM" };
        
        format!("{} {} {}", hour_word, minute_word, period)
    }
    
    fn translate_fuzzy(&self, time: &TimeInfo) -> String {
        let period = if time.is_pm { "PM" } else { "AM" };
        
        match time.minute {
            0 => format!("{} en punto", Self::hour_word(time.hour)),
            15 => format!("{} y cuarto {}", Self::hour_word(time.hour), period),
            30 => format!("{} y media {}", Self::hour_word(time.hour), period),
            45 => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("cuarto para {} {}", Self::hour_word(next_hour), period)
            },
            1..=7 => format!("{} y {} {}", Self::hour_word(time.hour), Self::number_to_word(time.minute), period),
            8..=14 => format!("casi {} y cuarto {}", Self::hour_word(time.hour), period),
            16..=22 => format!("{} y veinte {}", Self::hour_word(time.hour), period),
            23..=29 => format!("casi {} y media {}", Self::hour_word(time.hour), period),
            31..=37 => format!("pasando {} y media {}", Self::hour_word(time.hour), period),
            38..=44 => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("casi cuarto para {} {}", Self::hour_word(next_hour), period)
            },
            46..=52 => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("casi {} {}", Self::hour_word(next_hour), period)
            },
            _ => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("casi {} en punto", Self::hour_word(next_hour))
            }
        }
    }
    
    fn translate_very_fuzzy(&self, time: &TimeInfo) -> String {
        match time.minute {
            0..=7 => format!("{} en punto", Self::hour_word(time.hour)),
            8..=22 => format!("como {} y cuarto", Self::hour_word(time.hour)),
            23..=37 => format!("como {} y media", Self::hour_word(time.hour)),
            38..=52 => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("casi cuarto para {}", Self::hour_word(next_hour))
            },
            _ => {
                let next_hour = if time.hour == 12 { 1 } else { time.hour + 1 };
                format!("casi {} en punto", Self::hour_word(next_hour))
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
            5..=11 => "mañana".to_string(),
            12..=16 => "tarde".to_string(),
            17..=21 => "atardecer".to_string(),
            _ => "noche".to_string(),
        }
    }
}
