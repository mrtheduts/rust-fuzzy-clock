pub mod english;
pub mod spanish;
pub mod portuguese;

use crate::time::TimeInfo;

#[derive(Debug, Clone, Copy)]
pub enum FuzzynessLevel {
    Exact,
    Fuzzy,
    VeryFuzzy,
    MaxFuzzy,
}

impl FuzzynessLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "exact" => Some(FuzzynessLevel::Exact),
            "fuzzy" => Some(FuzzynessLevel::Fuzzy),
            "very-fuzzy" => Some(FuzzynessLevel::VeryFuzzy),
            "max-fuzzy" => Some(FuzzynessLevel::MaxFuzzy),
            _ => None,
        }
    }
}

pub trait TimeTranslator {
    fn translate(&self, time: &TimeInfo, level: FuzzynessLevel, use_24h: bool) -> String;
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
