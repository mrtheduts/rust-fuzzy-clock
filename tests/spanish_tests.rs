use rust_fuzzy_clock::time::TimeInfo;
use rust_fuzzy_clock::translator::{TimeTranslator, FuzzynessLevel, spanish::SpanishTranslator};

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
    let translator = SpanishTranslator;
    
    let time = create_time_info(15, 47);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "tres cuarenta y siete PM");
    
    let time = create_time_info(1, 5);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "una cero cinco AM");
}

#[test]
fn test_exact_24h_format() {
    let translator = SpanishTranslator;
    
    let time = create_time_info(15, 47);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, true), "quince cuarenta y siete");
    
    let time = create_time_info(1, 5);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, true), "una cero cinco");
}

#[test]
fn test_fuzzy_special_times() {
    let translator = SpanishTranslator;
    
    // En punto (on the hour)
    let time = create_time_info(3, 0);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "tres en punto");
    
    // Y cuarto (quarter past)
    let time = create_time_info(9, 15);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "nueve y cuarto AM");
    
    // Y media (half past)
    let time = create_time_info(14, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "dos y media PM");
    
    // Cuarto para (quarter to)
    let time = create_time_info(15, 45);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, false), "cuarto para cuatro PM");
}

#[test]
fn test_fuzzy_24h_format() {
    let translator = SpanishTranslator;
    
    let time = create_time_info(15, 15);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, true), "quince y cuarto");
    
    let time = create_time_info(23, 45);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Fuzzy, true), "cuarto para cero");
}

#[test]
fn test_very_fuzzy() {
    let translator = SpanishTranslator;
    
    let time = create_time_info(3, 5);
    assert_eq!(translator.translate(&time, FuzzynessLevel::VeryFuzzy, false), "tres en punto");
    
    let time = create_time_info(3, 20);
    assert_eq!(translator.translate(&time, FuzzynessLevel::VeryFuzzy, false), "como tres y cuarto");
}

#[test]
fn test_max_fuzzy() {
    let translator = SpanishTranslator;
    
    // Mañana
    let time = create_time_info(9, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "mañana");
    
    // Tarde
    let time = create_time_info(14, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "tarde");
    
    // Atardecer
    let time = create_time_info(19, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "atardecer");
    
    // Noche
    let time = create_time_info(23, 30);
    assert_eq!(translator.translate(&time, FuzzynessLevel::MaxFuzzy, false), "noche");
}

#[test]
fn test_hour_one_gender() {
    let translator = SpanishTranslator;
    
    // "una" for 1 o'clock in 12h
    let time = create_time_info(1, 0);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "una cero cero AM");
    
    let time = create_time_info(13, 0);
    assert_eq!(translator.translate(&time, FuzzynessLevel::Exact, false), "una cero cero PM");
}
