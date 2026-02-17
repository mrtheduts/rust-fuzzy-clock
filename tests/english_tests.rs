use rust_fuzzy_clock::time::TimeInfo;
use rust_fuzzy_clock::translator::{TimeTranslator, FuzzynessLevel, english::EnglishTranslator};

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
    let translator = EnglishTranslator;
    
    let time = create_time_info(15, 47);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "three forty-seven PM");
    
    let time = create_time_info(9, 5);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "nine oh five AM");
    
    let time = create_time_info(0, 0);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "twelve oh zero AM");
}

#[test]
fn test_exact_24h_format() {
    let translator = EnglishTranslator;
    
    let time = create_time_info(15, 47);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, true), "fifteen forty-seven");
    
    let time = create_time_info(9, 5);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, true), "nine oh five");
    
    let time = create_time_info(0, 0);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, true), "zero oh zero");
}

#[test]
fn test_fuzzy_special_times() {
    let translator = EnglishTranslator;
    
    // On the hour
    let time = create_time_info(3, 0);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "three o'clock");
    
    // Quarter past
    let time = create_time_info(9, 15);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "quarter past nine AM");
    
    // Half past
    let time = create_time_info(14, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "half past two PM");
    
    // Quarter to
    let time = create_time_info(15, 45);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "quarter to four PM");
}

#[test]
fn test_fuzzy_24h_format() {
    let translator = EnglishTranslator;
    
    let time = create_time_info(15, 15);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, true), "quarter past fifteen");
    
    let time = create_time_info(23, 45);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, true), "quarter to zero");
}

#[test]
fn test_very_fuzzy() {
    let translator = EnglishTranslator;
    
    let time = create_time_info(3, 5);
    assert_eq!(translator.translate(&time, FuzzynessLevel::VeryFuzzy, false), "three o'clock");
    
    let time = create_time_info(3, 20);
    assert_eq!(translator.translate(&time, FuzzynessLevel::VeryFuzzy, false), "about quarter past three");
    
    let time = create_time_info(3, 35);
    assert_eq!(translator.translate(&time, FuzzynessLevel::VeryFuzzy, false), "about half past three");
    
    let time = create_time_info(3, 50);
    assert_eq!(translator.translate(&time, FuzzynessLevel::VeryFuzzy, false), "about quarter to four");
}

#[test]
fn test_max_fuzzy() {
    let translator = EnglishTranslator;
    
    // Morning
    let time = create_time_info(9, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "morning");
    
    // Afternoon
    let time = create_time_info(14, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "afternoon");
    
    // Evening
    let time = create_time_info(19, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "evening");
    
    // Night
    let time = create_time_info(23, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "night");
    
    let time = create_time_info(2, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "night");
}

#[test]
fn test_edge_case_midnight() {
    let translator = EnglishTranslator;
    let time = create_time_info(0, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "twelve thirty AM");
}

#[test]
fn test_edge_case_noon() {
    let translator = EnglishTranslator;
    let time = create_time_info(12, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "twelve thirty PM");
}
