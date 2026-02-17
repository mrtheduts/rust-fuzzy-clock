use rust_fuzzy_clock::time::TimeInfo;
use rust_fuzzy_clock::translator::{TimeTranslator, FuzzinessLevel, spanish::SpanishTranslator};

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
    assert_eq!(translator.translate(&time, FuzzinessLevel::Exact, false, false), "tres cuarenta y siete PM");
    
    let time = create_time_info(1, 5);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Exact, false, false), "una cero cinco AM");
}

#[test]
fn test_exact_24h_format() {
    let translator = SpanishTranslator;
    
    let time = create_time_info(15, 47);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Exact, true, false), "quince cuarenta y siete");
    
    let time = create_time_info(1, 5);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Exact, true, false), "una cero cinco");
}

#[test]
fn test_fuzzy_special_times() {
    let translator = SpanishTranslator;
    
    // En punto (on the hour)
    let time = create_time_info(3, 0);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Fuzzy, false, false), "tres en punto");
    
    // Y cuarto (quarter past)
    let time = create_time_info(9, 15);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Fuzzy, false, false), "nueve y cuarto AM");
    
    // Y media (half past)
    let time = create_time_info(14, 30);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Fuzzy, false, false), "dos y media PM");
    
    // Cuarto para (quarter to)
    let time = create_time_info(15, 45);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Fuzzy, false, false), "cuarto para cuatro PM");
}

#[test]
fn test_fuzzy_24h_format() {
    let translator = SpanishTranslator;
    
    let time = create_time_info(15, 15);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Fuzzy, true, false), "quince y cuarto");
    
    let time = create_time_info(23, 45);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Fuzzy, true, false), "cuarto para cero");
}

#[test]
fn test_very_fuzzy() {
    let translator = SpanishTranslator;
    
    let time = create_time_info(3, 5);
    assert_eq!(translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, false), "tres en punto");
    
    let time = create_time_info(3, 20);
    assert_eq!(translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, false), "como tres y cuarto");
}

#[test]
fn test_max_fuzzy() {
    let translator = SpanishTranslator;
    
    // Mañana
    let time = create_time_info(9, 30);
    assert_eq!(translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false), "mañana");
    
    // Tarde
    let time = create_time_info(14, 30);
    assert_eq!(translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false), "tarde");
    
    // Atardecer
    let time = create_time_info(19, 30);
    assert_eq!(translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false), "atardecer");
    
    // Noche
    let time = create_time_info(23, 30);
    assert_eq!(translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false), "noche");
}

#[test]
fn test_hour_one_gender() {
    let translator = SpanishTranslator;
    
    // "una" for 1 o'clock in 12h
    let time = create_time_info(1, 0);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Exact, false, false), "una cero cero AM");
    
    let time = create_time_info(13, 0);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Exact, false, false), "una cero cero PM");
}

// New tests for Phase 7

#[test]
fn test_gender_aware_feminine_hours() {
    let translator = SpanishTranslator;
    
    // Test numbers that change with gender - hours (feminine)
    let time = create_time_info(1, 30);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Exact, false, false), "una treinta AM");
    
    let time = create_time_info(21, 0);
    assert_eq!(translator.translate(&time, FuzzinessLevel::Exact, false, false), "nueve cero cero PM");
}

#[test]
fn test_gender_aware_masculine_minutes() {
    let translator = SpanishTranslator;
    
    // Minutes use masculine form (un, not una)
    let time = create_time_info(2, 1);
    let result = translator.translate(&time, FuzzinessLevel::Exact, false, false);
    assert!(result.contains("cero un")); // masculine "un minuto"
    
    let time = create_time_info(3, 21);
    let result = translator.translate(&time, FuzzinessLevel::Exact, false, false);
    assert!(result.contains("veintiún")); // masculine
}

#[test]
fn test_include_units_exact() {
    let translator = SpanishTranslator;
    
    let time = create_time_info(3, 15);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, true),
        "tres horas quince minutos AM"
    );
    
    // Test singular
    let time = create_time_info(1, 1);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, true),
        "una hora cero un minuto AM"
    );
}

#[test]
fn test_include_units_fuzzy() {
    let translator = SpanishTranslator;
    
    // Quarter past
    let time = create_time_info(3, 15);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, true),
        "tres horas y cuarto AM"
    );
}

#[test]
fn test_include_units_very_fuzzy() {
    let translator = SpanishTranslator;
    
    let time = create_time_info(3, 5);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, true),
        "tres horas en punto"
    );
}

#[test]
fn test_max_fuzzy_ignores_units() {
    let translator = SpanishTranslator;
    
    // Max fuzzy should ignore include_units flag
    let time = create_time_info(9, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, true)
    );
}
