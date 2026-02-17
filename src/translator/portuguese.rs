use super::{TimeTranslator, FuzzynessLevel};
use crate::time::TimeInfo;

pub struct PortugueseTranslator;

impl PortugueseTranslator {
    fn number_to_word(n: u32) -> &'static str {
        match n {
            0 => "zero",
            1 => "uma",
            2 => "duas",
            3 => "três",
            4 => "quatro",
            5 => "cinco",
            6 => "seis",
            7 => "sete",
            8 => "oito",
            9 => "nove",
            10 => "dez",
            11 => "onze",
            12 => "doze",
            13 => "treze",
            14 => "catorze",
            15 => "quinze",
            16 => "dezesseis",
            17 => "dezessete",
            18 => "dezoito",
            19 => "dezenove",
            20 => "vinte",
            21 => "vinte e um",
            22 => "vinte e dois",
            23 => "vinte e três",
            24 => "vinte e quatro",
            25 => "vinte e cinco",
            26 => "vinte e seis",
            27 => "vinte e sete",
            28 => "vinte e oito",
            29 => "vinte e nove",
            30 => "trinta",
            31 => "trinta e um",
            32 => "trinta e dois",
            33 => "trinta e três",
            34 => "trinta e quatro",
            35 => "trinta e cinco",
            36 => "trinta e seis",
            37 => "trinta e sete",
            38 => "trinta e oito",
            39 => "trinta e nove",
            40 => "quarenta",
            41 => "quarenta e um",
            42 => "quarenta e dois",
            43 => "quarenta e três",
            44 => "quarenta e quatro",
            45 => "quarenta e cinco",
            46 => "quarenta e seis",
            47 => "quarenta e sete",
            48 => "quarenta e oito",
            49 => "quarenta e nove",
            50 => "cinquenta",
            51 => "cinquenta e um",
            52 => "cinquenta e dois",
            53 => "cinquenta e três",
            54 => "cinquenta e quatro",
            55 => "cinquenta e cinco",
            56 => "cinquenta e seis",
            57 => "cinquenta e sete",
            58 => "cinquenta e oito",
            59 => "cinquenta e nove",
            _ => "desconhecido",
        }
    }
    
    fn hour_word(n: u32, is_24h: bool) -> &'static str {
        if !is_24h && n == 1 {
            "uma"
        } else {
            Self::number_to_word(n)
        }
    }
    
    fn format_minute(minute: u32) -> String {
        if minute < 10 {
            format!("zero {}", Self::number_to_word(minute))
        } else {
            Self::number_to_word(minute).to_string()
        }
    }
}

impl TimeTranslator for PortugueseTranslator {
    fn translate(&self, time: &TimeInfo, level: FuzzynessLevel, use_24h: bool) -> String {
        match level {
            FuzzynessLevel::Exact => self.translate_exact(time, use_24h),
            FuzzynessLevel::Fuzzy => self.translate_fuzzy(time, use_24h),
            FuzzynessLevel::VeryFuzzy => self.translate_very_fuzzy(time, use_24h),
            FuzzynessLevel::MaxFuzzy => self.translate_max_fuzzy(time),
        }
    }
}

impl PortugueseTranslator {
    fn translate_exact(&self, time: &TimeInfo, use_24h: bool) -> String {
        if use_24h {
            let hour_word = Self::number_to_word(time.hour24);
            let minute_word = Self::format_minute(time.minute);
            format!("{} {}", hour_word, minute_word)
        } else {
            let hour_word = Self::hour_word(time.hour, false);
            let minute_word = Self::format_minute(time.minute);
            let period = if time.is_pm { "PM" } else { "AM" };
            format!("{} {} {}", hour_word, minute_word, period)
        }
    }
    
    fn translate_fuzzy(&self, time: &TimeInfo, use_24h: bool) -> String {
        let hour = if use_24h { time.hour24 } else { time.hour };
        let period = if use_24h { "" } else if time.is_pm { " da tarde" } else { " da manhã" };
        
        match time.minute {
            0 => format!("{} em ponto", Self::hour_word(hour, use_24h)),
            15 => format!("{} e quinze{}", Self::hour_word(hour, use_24h), period),
            30 => format!("{} e meia{}", Self::hour_word(hour, use_24h), period),
            45 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                format!("quinze para {}{}", Self::hour_word(next_hour, use_24h), period)
            },
            1..=7 => format!("{} e {}{}", Self::hour_word(hour, use_24h), Self::number_to_word(time.minute), period),
            8..=14 => format!("quase {} e quinze{}", Self::hour_word(hour, use_24h), period),
            16..=22 => format!("{} e vinte{}", Self::hour_word(hour, use_24h), period),
            23..=29 => format!("quase {} e meia{}", Self::hour_word(hour, use_24h), period),
            31..=37 => format!("passando {} e meia{}", Self::hour_word(hour, use_24h), period),
            38..=44 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                format!("quase quinze para {}{}", Self::hour_word(next_hour, use_24h), period)
            },
            46..=52 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                format!("quase {}{}", Self::hour_word(next_hour, use_24h), period)
            },
            _ => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                format!("quase {} em ponto", Self::hour_word(next_hour, use_24h))
            }
        }
    }
    
    fn translate_very_fuzzy(&self, time: &TimeInfo, use_24h: bool) -> String {
        let hour = if use_24h { time.hour24 } else { time.hour };
        
        match time.minute {
            0..=7 => format!("{} em ponto", Self::hour_word(hour, use_24h)),
            8..=22 => format!("por volta de {} e quinze", Self::hour_word(hour, use_24h)),
            23..=37 => format!("por volta de {} e meia", Self::hour_word(hour, use_24h)),
            38..=52 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                format!("quase quinze para {}", Self::hour_word(next_hour, use_24h))
            },
            _ => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 { 0 } else { time.hour24 + 1 }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                format!("quase {} em ponto", Self::hour_word(next_hour, use_24h))
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
            5..=11 => "manhã".to_string(),
            12..=17 => "tarde".to_string(),
            18..=23 => "noite".to_string(),
            _ => "madrugada".to_string(),
        }
    }
}
