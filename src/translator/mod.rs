pub mod english;
pub mod portuguese;
pub mod spanish;

use crate::time::TimeInfo;

#[derive(Debug, Clone, Copy)]
pub enum FuzzinessLevel {
    Exact,
    Fuzzy,
    VeryFuzzy,
    MaxFuzzy,
}

impl FuzzinessLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "exact" => Some(FuzzinessLevel::Exact),
            "fuzzy" => Some(FuzzinessLevel::Fuzzy),
            "very-fuzzy" => Some(FuzzinessLevel::VeryFuzzy),
            "max-fuzzy" => Some(FuzzinessLevel::MaxFuzzy),
            _ => None,
        }
    }
}

pub trait TimeTranslator {
    fn translate(
        &self,
        time: &TimeInfo,
        level: FuzzinessLevel,
        use_24h: bool,
        include_units: bool,
    ) -> String;
}

pub enum Language {
    English,
    Spanish,
    Portuguese,
}

impl Language {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "english" | "en" => Some(Language::English),
            "spanish" | "es" | "español" => Some(Language::Spanish),
            "portuguese" | "pt" | "português" => Some(Language::Portuguese),
            _ => None,
        }
    }
}

pub fn get_translator(language: Language) -> Box<dyn TimeTranslator> {
    match language {
        Language::English => Box::new(english::EnglishTranslator),
        Language::Spanish => Box::new(spanish::SpanishTranslator),
        Language::Portuguese => Box::new(portuguese::PortugueseTranslator),
    }
}
