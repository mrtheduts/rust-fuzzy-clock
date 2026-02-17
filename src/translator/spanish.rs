use super::{FuzzinessLevel, TimeTranslator};
use crate::time::TimeInfo;

pub struct SpanishTranslator;

impl SpanishTranslator {
    // Gender-aware number functions for Spanish
    // Hours are feminine (una hora, dos horas)
    // Minutes are masculine (un minuto, dos minutos)

    /// Returns feminine form for hours
    fn hour_number(n: u32) -> &'static str {
        match n {
            0 => "cero",
            1 => "una", // feminine
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
            21 => "veintiuna", // feminine
            22 => "veintidós",
            23 => "veintitrés",
            24 => "veinticuatro",
            25 => "veinticinco",
            26 => "veintiséis",
            27 => "veintisiete",
            28 => "veintiocho",
            29 => "veintinueve",
            30 => "treinta",
            31 => "treinta y una", // feminine
            32 => "treinta y dos",
            33 => "treinta y tres",
            34 => "treinta y cuatro",
            35 => "treinta y cinco",
            36 => "treinta y seis",
            37 => "treinta y siete",
            38 => "treinta y ocho",
            39 => "treinta y nueve",
            40 => "cuarenta",
            41 => "cuarenta y una", // feminine
            42 => "cuarenta y dos",
            43 => "cuarenta y tres",
            44 => "cuarenta y cuatro",
            45 => "cuarenta y cinco",
            46 => "cuarenta y seis",
            47 => "cuarenta y siete",
            48 => "cuarenta y ocho",
            49 => "cuarenta y nueve",
            50 => "cincuenta",
            51 => "cincuenta y una", // feminine
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

    /// Returns masculine form for minutes
    fn minute_number(n: u32) -> &'static str {
        match n {
            0 => "cero",
            1 => "un", // masculine
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
            21 => "veintiún", // masculine
            22 => "veintidós",
            23 => "veintitrés",
            24 => "veinticuatro",
            25 => "veinticinco",
            26 => "veintiséis",
            27 => "veintisiete",
            28 => "veintiocho",
            29 => "veintinueve",
            30 => "treinta",
            31 => "treinta y un", // masculine
            32 => "treinta y dos",
            33 => "treinta y tres",
            34 => "treinta y cuatro",
            35 => "treinta y cinco",
            36 => "treinta y seis",
            37 => "treinta y siete",
            38 => "treinta y ocho",
            39 => "treinta y nueve",
            40 => "cuarenta",
            41 => "cuarenta y un", // masculine
            42 => "cuarenta y dos",
            43 => "cuarenta y tres",
            44 => "cuarenta y cuatro",
            45 => "cuarenta y cinco",
            46 => "cuarenta y seis",
            47 => "cuarenta y siete",
            48 => "cuarenta y ocho",
            49 => "cuarenta y nueve",
            50 => "cincuenta",
            51 => "cincuenta y un", // masculine
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

    fn format_minute(minute: u32) -> String {
        if minute < 10 {
            format!("cero {}", Self::minute_number(minute))
        } else {
            Self::minute_number(minute).to_string()
        }
    }

    fn hour_unit(n: u32) -> &'static str {
        if n == 1 { "hora" } else { "horas" }
    }

    fn minute_unit(n: u32) -> &'static str {
        if n == 1 { "minuto" } else { "minutos" }
    }
}

impl TimeTranslator for SpanishTranslator {
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

impl SpanishTranslator {
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
            let period = if time.is_pm { "PM" } else { "AM" };
            if include_units {
                format!(
                    "{} {} {} {} {}",
                    hour_word,
                    Self::hour_unit(time.hour),
                    minute_word,
                    Self::minute_unit(time.minute),
                    period
                )
            } else {
                format!("{} {} {}", hour_word, minute_word, period)
            }
        }
    }

    fn translate_fuzzy(&self, time: &TimeInfo, use_24h: bool, include_units: bool) -> String {
        let hour = if use_24h { time.hour24 } else { time.hour };
        let period = if use_24h {
            ""
        } else if time.is_pm {
            " PM"
        } else {
            " AM"
        };
        let hour_unit_str = if include_units {
            format!(" {}", Self::hour_unit(hour))
        } else {
            String::new()
        };

        match time.minute {
            0 => format!("{} en punto", Self::hour_number(hour)),
            15 => format!(
                "{}{} y cuarto{}",
                Self::hour_number(hour),
                hour_unit_str,
                period
            ),
            30 => format!(
                "{}{} y media{}",
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
                    "cuarto para {}{}{}",
                    Self::hour_number(next_hour),
                    next_unit_str,
                    period
                )
            }
            1..=7 => format!(
                "{}{} y {}{}",
                Self::hour_number(hour),
                hour_unit_str,
                Self::minute_number(time.minute),
                period
            ),
            8..=14 => format!(
                "casi {}{} y cuarto{}",
                Self::hour_number(hour),
                hour_unit_str,
                period
            ),
            16..=22 => format!(
                "{}{} y veinte{}",
                Self::hour_number(hour),
                hour_unit_str,
                period
            ),
            23..=29 => format!(
                "casi {}{} y media{}",
                Self::hour_number(hour),
                hour_unit_str,
                period
            ),
            31..=37 => format!(
                "pasando {}{} y media{}",
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
                    "casi cuarto para {}{}{}",
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
                    "casi {}{}{}",
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
                format!("casi {} en punto", Self::hour_number(next_hour))
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
            0..=7 => format!("{}{} en punto", Self::hour_number(hour), hour_unit_str),
            8..=22 => format!("como {}{} y cuarto", Self::hour_number(hour), hour_unit_str),
            23..=37 => format!("como {}{} y media", Self::hour_number(hour), hour_unit_str),
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
                    "casi cuarto para {}{}",
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
                format!("casi {} en punto", Self::hour_number(next_hour))
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
