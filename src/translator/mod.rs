pub mod english;

use crate::time::TimeInfo;

#[derive(Debug, Clone, Copy)]
pub enum FuzzynessLevel {
    Exact,
}

impl FuzzynessLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "exact" => Some(FuzzynessLevel::Exact),
            _ => None,
        }
    }
}

pub trait TimeTranslator {
    fn translate(&self, time: &TimeInfo, level: FuzzynessLevel) -> String;
}

pub enum Language {
    English,
}

impl Language {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "english" | "en" => Some(Language::English),
            _ => None,
        }
    }
}

pub fn get_translator(language: Language) -> Box<dyn TimeTranslator> {
    match language {
        Language::English => Box::new(english::EnglishTranslator),
    }
}
