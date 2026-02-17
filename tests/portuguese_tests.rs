use rust_fuzzy_clock::time::TimeInfo;
use rust_fuzzy_clock::translator::{
    FuzzinessLevel, TimeTranslator, portuguese::PortugueseTranslator,
};

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
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, false),
        "três quarenta e sete"
    );

    let time = create_time_info(1, 5);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, false),
        "uma zero cinco"
    );
}

#[test]
fn test_exact_24h_format() {
    let translator = PortugueseTranslator;

    let time = create_time_info(15, 47);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, true, false),
        "quinze quarenta e sete"
    );

    let time = create_time_info(1, 5);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, true, false),
        "uma zero cinco"
    );
}

#[test]
fn test_fuzzy_special_times() {
    let translator = PortugueseTranslator;

    // Em ponto (on the hour)
    let time = create_time_info(3, 0);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, false),
        "três em ponto"
    );

    // E quinze (quarter past)
    let time = create_time_info(9, 15);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, false),
        "nove e quinze da manhã"
    );

    // E meia (half past)
    let time = create_time_info(14, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, false),
        "duas e meia da tarde"
    );

    // Para (to)
    let time = create_time_info(15, 45);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, false),
        "quinze para quatro da tarde"
    );
}

#[test]
fn test_fuzzy_24h_format() {
    let translator = PortugueseTranslator;

    let time = create_time_info(15, 15);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, true, false),
        "quinze e quinze"
    );

    let time = create_time_info(23, 45);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, true, false),
        "quinze para zero"
    );
}

#[test]
fn test_very_fuzzy() {
    let translator = PortugueseTranslator;

    let time = create_time_info(3, 5);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, false),
        "três em ponto"
    );

    let time = create_time_info(3, 20);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, false),
        "cerca de três e quinze"
    );
}

#[test]
fn test_max_fuzzy() {
    let translator = PortugueseTranslator;

    // Manhã
    let time = create_time_info(9, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        "manhã"
    );

    // Tarde
    let time = create_time_info(14, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        "tarde"
    );

    // Noite (evening hours)
    let time = create_time_info(19, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        "noite"
    );

    // Noite (night hours)
    let time = create_time_info(23, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        "noite"
    );

    // Madrugada (late night/early morning)
    let time = create_time_info(2, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        "madrugada"
    );
}

#[test]
fn test_period_suffixes() {
    let translator = PortugueseTranslator;

    // da manhã (morning)
    let time = create_time_info(9, 15);
    let result = translator.translate(&time, FuzzinessLevel::Fuzzy, false, false);
    assert!(result.contains("da manhã"));

    // da tarde (afternoon/evening)
    let time = create_time_info(15, 15);
    let result = translator.translate(&time, FuzzinessLevel::Fuzzy, false, false);
    assert!(result.contains("da tarde"));
}

#[test]
fn test_hour_one_gender() {
    let translator = PortugueseTranslator;

    // "uma" for 1 o'clock
    let time = create_time_info(1, 0);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, false),
        "uma zero zero"
    );
}

// New tests for Phase 7

#[test]
fn test_gender_aware_feminine_hours() {
    let translator = PortugueseTranslator;

    // Test feminine forms for hours
    let time = create_time_info(1, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, false),
        "uma trinta"
    );

    let time = create_time_info(2, 0);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, false),
        "duas zero zero"
    );

    let time = create_time_info(21, 0);
    let result = translator.translate(&time, FuzzinessLevel::Exact, false, false);
    assert!(result.starts_with("nove")); // 21 in 12h = 9, feminine: nove
}

#[test]
fn test_gender_aware_masculine_minutes() {
    let translator = PortugueseTranslator;

    // Minutes use masculine form (um/dois, not uma/duas)
    let time = create_time_info(3, 1);
    let result = translator.translate(&time, FuzzinessLevel::Exact, false, false);
    assert!(result.contains("zero um")); // masculine "um minuto"

    let time = create_time_info(3, 2);
    let result = translator.translate(&time, FuzzinessLevel::Exact, false, false);
    assert!(result.contains("zero dois")); // masculine "dois minutos"

    let time = create_time_info(3, 21);
    let result = translator.translate(&time, FuzzinessLevel::Exact, false, false);
    assert!(result.contains("vinte e um")); // masculine
}

#[test]
fn test_include_units_exact() {
    let translator = PortugueseTranslator;

    let time = create_time_info(3, 15);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, true),
        "três horas quinze minutos"
    );

    // Test singular
    let time = create_time_info(1, 1);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, true),
        "uma hora zero um minuto"
    );

    // Test 2 (both change with gender)
    let time = create_time_info(2, 2);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Exact, false, true),
        "duas horas zero dois minutos"
    );
}

#[test]
fn test_include_units_fuzzy() {
    let translator = PortugueseTranslator;

    // Quarter past
    let time = create_time_info(3, 15);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::Fuzzy, false, true),
        "três horas e quinze"
    );
}

#[test]
fn test_include_units_very_fuzzy() {
    let translator = PortugueseTranslator;

    let time = create_time_info(3, 5);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::VeryFuzzy, false, true),
        "três horas em ponto"
    );
}

#[test]
fn test_max_fuzzy_ignores_units() {
    let translator = PortugueseTranslator;

    // Max fuzzy should ignore include_units flag
    let time = create_time_info(9, 30);
    assert_eq!(
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, false),
        translator.translate(&time, FuzzinessLevel::MaxFuzzy, false, true)
    );
}
