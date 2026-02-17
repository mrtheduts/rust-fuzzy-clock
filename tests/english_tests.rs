use rust_fuzzy_clock::time::TimeInfo;
use rust_fuzzy_clock::translator::{FuzzinessLevel, TimeTranslator, english::EnglishTranslator};

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
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, false),
        "three forty-seven PM"
    );

    let time = create_time_info(9, 5);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, false),
        "nine oh five AM"
    );

    let time = create_time_info(0, 0);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, false),
        "twelve oh zero AM"
    );
}

#[test]
fn test_exact_24h_format() {
    let translator = EnglishTranslator;

    let time = create_time_info(15, 47);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, true, false),
        "fifteen forty-seven"
    );

    let time = create_time_info(9, 5);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, true, false),
        "nine oh five"
    );

    let time = create_time_info(0, 0);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, true, false),
        "zero oh zero"
    );
}

#[test]
fn test_fuzzy_special_times() {
    let translator = EnglishTranslator;

    // On the hour
    let time = create_time_info(3, 0);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, false),
        "three o'clock"
    );

    // Quarter past
    let time = create_time_info(9, 15);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, false),
        "quarter past nine AM"
    );

    // Half past
    let time = create_time_info(14, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, false),
        "half past two PM"
    );

    // Quarter to
    let time = create_time_info(15, 45);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, false),
        "quarter to four PM"
    );
}

#[test]
fn test_fuzzy_24h_format() {
    let translator = EnglishTranslator;

    let time = create_time_info(15, 15);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, true, false),
        "quarter past fifteen"
    );

    let time = create_time_info(23, 45);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, true, false),
        "quarter to zero"
    );
}

#[test]
fn test_very_fuzzy() {
    let translator = EnglishTranslator;

    let time = create_time_info(3, 5);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, false),
        "three o'clock"
    );

    let time = create_time_info(3, 20);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, false),
        "about quarter past three"
    );

    let time = create_time_info(3, 35);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, false),
        "about half past three"
    );

    let time = create_time_info(3, 50);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, false),
        "about quarter to four"
    );
}

#[test]
fn test_max_fuzzy() {
    let translator = EnglishTranslator;

    // Morning
    let time = create_time_info(9, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        "morning"
    );

    // Afternoon
    let time = create_time_info(14, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        "afternoon"
    );

    // Evening
    let time = create_time_info(19, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        "evening"
    );

    // Night
    let time = create_time_info(23, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        "night"
    );

    let time = create_time_info(2, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        "night"
    );
}

#[test]
fn test_edge_case_midnight() {
    let translator = EnglishTranslator;
    let time = create_time_info(0, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, false),
        "twelve thirty AM"
    );
}

#[test]
fn test_edge_case_noon() {
    let translator = EnglishTranslator;
    let time = create_time_info(12, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, false),
        "twelve thirty PM"
    );
}

// New tests for Phase 7

#[test]
fn test_include_units_exact() {
    let translator = EnglishTranslator;

    let time = create_time_info(3, 15);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, true),
        "three hours fifteen minutes AM"
    );

    // Test singular
    let time = create_time_info(1, 1);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, true),
        "one hour oh one minute AM"
    );

    // Test with 12
    let time = create_time_info(12, 0);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, true),
        "twelve hours oh zero minutes PM"
    );
}

#[test]
fn test_include_units_fuzzy() {
    let translator = EnglishTranslator;

    // Quarter past
    let time = create_time_info(3, 15);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, true),
        "quarter past three hours AM"
    );

    // Half past
    let time = create_time_info(3, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, true),
        "half past three hours AM"
    );
}

#[test]
fn test_include_units_very_fuzzy() {
    let translator = EnglishTranslator;

    let time = create_time_info(3, 5);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, true),
        "three hours o'clock"
    );

    let time = create_time_info(3, 20);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, true),
        "about quarter past three hours"
    );
}

#[test]
fn test_include_units_24h_format() {
    let translator = EnglishTranslator;

    let time = create_time_info(15, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, true, true),
        "fifteen hours thirty minutes"
    );

    // Test singular
    let time = create_time_info(1, 1);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, true, true),
        "one hour oh one minute"
    );
}

#[test]
fn test_max_fuzzy_ignores_units() {
    let translator = EnglishTranslator;

    // Max fuzzy should ignore include_units flag
    let time = create_time_info(9, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, true)
    );
}
