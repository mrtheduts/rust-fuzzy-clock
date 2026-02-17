use super::{FuzzinessLevel, TimeTranslator};
use crate::time::TimeInfo;

pub struct PortugueseTranslator;

impl PortugueseTranslator {
    // Gender-aware number functions for Portuguese
    // Hours are feminine (uma hora, duas horas)
    // Minutes are masculine (um minuto, dois minutos)

    /// Returns feminine form for hours
    fn hour_number(n: u32) -> &'static str {
        match n {
            0 => "zero",
            1 => "uma",  // feminine
            2 => "duas", // feminine
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
            21 => "vinte e uma",  // feminine
            22 => "vinte e duas", // feminine
            23 => "vinte e três",
            24 => "vinte e quatro",
            25 => "vinte e cinco",
            26 => "vinte e seis",
            27 => "vinte e sete",
            28 => "vinte e oito",
            29 => "vinte e nove",
            30 => "trinta",
            31 => "trinta e uma",  // feminine
            32 => "trinta e duas", // feminine
            33 => "trinta e três",
            34 => "trinta e quatro",
            35 => "trinta e cinco",
            36 => "trinta e seis",
            37 => "trinta e sete",
            38 => "trinta e oito",
            39 => "trinta e nove",
            40 => "quarenta",
            41 => "quarenta e uma",  // feminine
            42 => "quarenta e duas", // feminine
            43 => "quarenta e três",
            44 => "quarenta e quatro",
            45 => "quarenta e cinco",
            46 => "quarenta e seis",
            47 => "quarenta e sete",
            48 => "quarenta e oito",
            49 => "quarenta e nove",
            50 => "cinquenta",
            51 => "cinquenta e uma",  // feminine
            52 => "cinquenta e duas", // feminine
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

    /// Returns masculine form for minutes
    fn minute_number(n: u32) -> &'static str {
        match n {
            0 => "zero",
            1 => "um",   // masculine
            2 => "dois", // masculine
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
            21 => "vinte e um",   // masculine
            22 => "vinte e dois", // masculine
            23 => "vinte e três",
            24 => "vinte e quatro",
            25 => "vinte e cinco",
            26 => "vinte e seis",
            27 => "vinte e sete",
            28 => "vinte e oito",
            29 => "vinte e nove",
            30 => "trinta",
            31 => "trinta e um",   // masculine
            32 => "trinta e dois", // masculine
            33 => "trinta e três",
            34 => "trinta e quatro",
            35 => "trinta e cinco",
            36 => "trinta e seis",
            37 => "trinta e sete",
            38 => "trinta e oito",
            39 => "trinta e nove",
            40 => "quarenta",
            41 => "quarenta e um",   // masculine
            42 => "quarenta e dois", // masculine
            43 => "quarenta e três",
            44 => "quarenta e quatro",
            45 => "quarenta e cinco",
            46 => "quarenta e seis",
            47 => "quarenta e sete",
            48 => "quarenta e oito",
            49 => "quarenta e nove",
            50 => "cinquenta",
            51 => "cinquenta e um",   // masculine
            52 => "cinquenta e dois", // masculine
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

    fn format_minute(minute: u32) -> String {
        if minute < 10 {
            format!("zero {}", Self::minute_number(minute))
        } else {
            Self::minute_number(minute).to_string()
        }
    }

    fn get_period_suffix(is_pm: bool, hour: u32) -> &'static str {
        if hour >= 6 && hour < 12 {
            " da manhã"
        } else if is_pm && hour != 12 {
            " da tarde"
        } else {
            ""
        }
    }

    fn hour_unit(n: u32) -> &'static str {
        if n == 1 { "hora" } else { "horas" }
    }

    fn minute_unit(n: u32) -> &'static str {
        if n == 1 { "minuto" } else { "minutos" }
    }
}

impl TimeTranslator for PortugueseTranslator {
    fn translate(
        &self,
        time: &TimeInfo,
        level: FuzzinessLevel,
        use_24h: bool,
        include_units: bool,
    ) -> String {
        match level {
            FuzzinessLevel::Exact => self.translate_exact(time, use_24h, include_units),
            FuzzinessLevel::Fuzzy => self.translate_fuzzy(time, use_24h, include_units),
            FuzzinessLevel::VeryFuzzy => self.translate_very_fuzzy(time, use_24h, include_units),
            FuzzinessLevel::MaxFuzzy => self.translate_max_fuzzy(time),
        }
    }
}

impl PortugueseTranslator {
    fn translate_exact(&self, time: &TimeInfo, use_24h: bool, include_units: bool) -> String {
        if use_24h {
            let hour_word = Self::hour_number(time.hour24);
            let minute_word = Self::format_minute(time.minute);
            if include_units {
                format!(
                    "{} {} {} {}",
                    hour_word,
                    Self::hour_unit(time.hour24),
                    minute_word,
                    Self::minute_unit(time.minute)
                )
            } else {
                format!("{} {}", hour_word, minute_word)
            }
        } else {
            let hour_word = Self::hour_number(time.hour);
            let minute_word = Self::format_minute(time.minute);
            if include_units {
                format!(
                    "{} {} {} {}",
                    hour_word,
                    Self::hour_unit(time.hour),
                    minute_word,
                    Self::minute_unit(time.minute)
                )
            } else {
                format!("{} {}", hour_word, minute_word)
            }
        }
    }

    fn translate_fuzzy(&self, time: &TimeInfo, use_24h: bool, include_units: bool) -> String {
        let hour = if use_24h { time.hour24 } else { time.hour };
        let period = if use_24h {
            ""
        } else {
            Self::get_period_suffix(time.is_pm, time.hour)
        };
        let hour_unit_str = if include_units {
            format!(" {}", Self::hour_unit(hour))
        } else {
            String::new()
        };

        match time.minute {
            0 => format!("{} em ponto", Self::hour_number(hour)),
            15 => format!(
                "{}{} e quinze{}",
                Self::hour_number(hour),
                hour_unit_str,
                period
            ),
            30 => format!(
                "{}{} e meia{}",
                Self::hour_number(hour),
                hour_unit_str,
                period
            ),
            45 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 {
                        0
                    } else {
                        time.hour24 + 1
                    }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                let next_unit_str = if include_units {
                    format!(" {}", Self::hour_unit(next_hour))
                } else {
                    String::new()
                };
                format!(
                    "quinze para {}{}{}",
                    Self::hour_number(next_hour),
                    next_unit_str,
                    period
                )
            }
            1..=7 => format!(
                "{}{} e {}{}",
                Self::hour_number(hour),
                hour_unit_str,
                Self::minute_number(time.minute),
                period
            ),
            8..=14 => format!(
                "quase {}{} e quinze{}",
                Self::hour_number(hour),
                hour_unit_str,
                period
            ),
            16..=22 => format!(
                "{}{} e vinte{}",
                Self::hour_number(hour),
                hour_unit_str,
                period
            ),
            23..=29 => format!(
                "quase {}{} e meia{}",
                Self::hour_number(hour),
                hour_unit_str,
                period
            ),
            31..=37 => format!(
                "passando {}{} e meia{}",
                Self::hour_number(hour),
                hour_unit_str,
                period
            ),
            38..=44 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 {
                        0
                    } else {
                        time.hour24 + 1
                    }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                let next_unit_str = if include_units {
                    format!(" {}", Self::hour_unit(next_hour))
                } else {
                    String::new()
                };
                format!(
                    "quase quinze para {}{}{}",
                    Self::hour_number(next_hour),
                    next_unit_str,
                    period
                )
            }
            46..=52 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 {
                        0
                    } else {
                        time.hour24 + 1
                    }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                let next_unit_str = if include_units {
                    format!(" {}", Self::hour_unit(next_hour))
                } else {
                    String::new()
                };
                format!(
                    "quase {}{}{}",
                    Self::hour_number(next_hour),
                    next_unit_str,
                    period
                )
            }
            _ => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 {
                        0
                    } else {
                        time.hour24 + 1
                    }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                format!("quase {} em ponto", Self::hour_number(next_hour))
            }
        }
    }

    fn translate_very_fuzzy(&self, time: &TimeInfo, use_24h: bool, include_units: bool) -> String {
        let hour = if use_24h { time.hour24 } else { time.hour };
        let hour_unit_str = if include_units {
            format!(" {}", Self::hour_unit(hour))
        } else {
            String::new()
        };

        match time.minute {
            0..=7 => format!("{}{} em ponto", Self::hour_number(hour), hour_unit_str),
            8..=22 => format!(
                "cerca de {}{} e quinze",
                Self::hour_number(hour),
                hour_unit_str
            ),
            23..=37 => format!(
                "cerca de {}{} e meia",
                Self::hour_number(hour),
                hour_unit_str
            ),
            38..=52 => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 {
                        0
                    } else {
                        time.hour24 + 1
                    }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                let next_unit_str = if include_units {
                    format!(" {}", Self::hour_unit(next_hour))
                } else {
                    String::new()
                };
                format!(
                    "quase quinze para {}{}",
                    Self::hour_number(next_hour),
                    next_unit_str
                )
            }
            _ => {
                let next_hour = if use_24h {
                    if time.hour24 == 23 {
                        0
                    } else {
                        time.hour24 + 1
                    }
                } else {
                    if time.hour == 12 { 1 } else { time.hour + 1 }
                };
                format!("quase {} em ponto", Self::hour_number(next_hour))
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
            0..=5 => "madrugada".to_string(),
            6..=11 => "manhã".to_string(),
            12..=18 => "tarde".to_string(),
            _ => "noite".to_string(),
        }
    }
}
