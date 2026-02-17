use rust_fuzzy_clock::time::TimeInfo;
use rust_fuzzy_clock::translator::{TimeTranslator, FuzzynessLevel, portuguese::PortugueseTranslator};

fn create_time_info(hour24: u32, minute: u32) -> TimeInfo {
    let is_pm = hour24 >= 12;
    let hour12 = match hour24 {
        0 => 12,
        13..=23 => hour24 - 12,
        _ => hour24,
    };
    
    TimeInfo {
        hour: hour12,
        hour24,
        minute,
        is_pm,
    }
}

#[test]
fn test_exact_12h_format() {
    let translator = PortugueseTranslator;
    
    let time = create_time_info(15, 47);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "três quarenta e sete PM");
    
    let time = create_time_info(1, 5);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "uma zero cinco AM");
}

#[test]
fn test_exact_24h_format() {
    let translator = PortugueseTranslator;
    
    let time = create_time_info(15, 47);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, true), "quinze quarenta e sete");
    
    let time = create_time_info(1, 5);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, true), "uma zero cinco");
}

#[test]
fn test_fuzzy_special_times() {
    let translator = PortugueseTranslator;
    
    // Em ponto (on the hour)
    let time = create_time_info(3, 0);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "três em ponto");
    
    // E quinze (quarter past)
    let time = create_time_info(9, 15);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "nove e quinze da manhã");
    
    // E meia (half past)
    let time = create_time_info(14, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "duas e meia da tarde");
    
    // Para (to)
    let time = create_time_info(15, 45);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "quinze para quatro da tarde");
}

#[test]
fn test_fuzzy_24h_format() {
    let translator = PortugueseTranslator;
    
    let time = create_time_info(15, 15);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, true), "quinze e quinze");
    
    let time = create_time_info(23, 45);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, true), "quinze para zero");
}

#[test]
fn test_very_fuzzy() {
    let translator = PortugueseTranslator;
    
    let time = create_time_info(3, 5);
    assert_eq!(translator.translate(&time, FuzzynessLevel::VeryFuzzy, false), "três em ponto");
    
    let time = create_time_info(3, 20);
    assert_eq!(translator.translate(&time, FuzzynessLevel::VeryFuzzy, false), "por volta de três e quinze");
}

#[test]
fn test_max_fuzzy() {
    let translator = PortugueseTranslator;
    
    // Manhã
    let time = create_time_info(9, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "manhã");
    
    // Tarde
    let time = create_time_info(14, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "tarde");
    
    // Noite (evening hours)
    let time = create_time_info(19, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "noite");
    
    // Noite (night hours)
    let time = create_time_info(23, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "noite");
    
    // Madrugada (late night/early morning)
    let time = create_time_info(2, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "madrugada");
}

#[test]
fn test_period_suffixes() {
    let translator = PortugueseTranslator;
    
    // da manhã (morning)
    let time = create_time_info(9, 15);
    let result = translator.translate(&time, FuzzynessLevel::Fuzzy, false);
    assert!(result.contains("da manhã"));
    
    // da tarde (afternoon/evening)
    let time = create_time_info(15, 15);
    let result = translator.translate(&time, FuzzynessLevel::Fuzzy, false);
    assert!(result.contains("da tarde"));
}

#[test]
fn test_hour_one_gender() {
    let translator = PortugueseTranslator;
    
    // "uma" for 1 o'clock
    let time = create_time_info(1, 0);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "uma zero zero AM");
}
