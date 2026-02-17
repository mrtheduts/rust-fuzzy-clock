use rust_fuzzy_clock::translator::{FuzzinessLevel, Language};

#[test]
fn test_language_parsing_english() {
    assert!(matches!(
        Language::from_str("english"),
        Some(Language::English)
    ));
    assert!(matches!(Language::from_str("en"), Some(Language::English)));
    assert!(matches!(
        Language::from_str("English"),
        Some(Language::English)
    ));
    assert!(matches!(Language::from_str("EN"), Some(Language::English)));
}

#[test]
fn test_language_parsing_spanish() {
    assert!(matches!(
        Language::from_str("spanish"),
        Some(Language::Spanish)
    ));
    assert!(matches!(Language::from_str("es"), Some(Language::Spanish)));
    assert!(matches!(
        Language::from_str("español"),
        Some(Language::Spanish)
    ));
    assert!(matches!(
        Language::from_str("SPANISH"),
        Some(Language::Spanish)
    ));
}

#[test]
fn test_language_parsing_portuguese() {
    assert!(matches!(
        Language::from_str("portuguese"),
        Some(Language::Portuguese)
    ));
    assert!(matches!(
        Language::from_str("pt"),
        Some(Language::Portuguese)
    ));
    assert!(matches!(
        Language::from_str("português"),
        Some(Language::Portuguese)
    ));
    assert!(matches!(
        Language::from_str("PORTUGUESE"),
        Some(Language::Portuguese)
    ));
}

#[test]
fn test_language_parsing_invalid() {
    assert!(Language::from_str("french").is_none());
    assert!(Language::from_str("fr").is_none());
    assert!(Language::from_str("").is_none());
    assert!(Language::from_str("xxx").is_none());
}

#[test]
fn test_fuzziness_parsing_valid() {
    assert!(matches!(
        FuzzinessLevel::from_str("exact"),
        Some(FuzzinessLevel::Exact)
    ));
    assert!(matches!(
        FuzzinessLevel::from_str("fuzzy"),
        Some(FuzzinessLevel::Fuzzy)
    ));
    assert!(matches!(
        FuzzinessLevel::from_str("very-fuzzy"),
        Some(FuzzinessLevel::VeryFuzzy)
    ));
    assert!(matches!(
        FuzzinessLevel::from_str("max-fuzzy"),
        Some(FuzzinessLevel::MaxFuzzy)
    ));
}

#[test]
fn test_fuzziness_parsing_case_insensitive() {
    assert!(matches!(
        FuzzinessLevel::from_str("EXACT"),
        Some(FuzzinessLevel::Exact)
    ));
    assert!(matches!(
        FuzzinessLevel::from_str("Fuzzy"),
        Some(FuzzinessLevel::Fuzzy)
    ));
    assert!(matches!(
        FuzzinessLevel::from_str("VERY-FUZZY"),
        Some(FuzzinessLevel::VeryFuzzy)
    ));
}

#[test]
fn test_fuzziness_parsing_invalid() {
    assert!(FuzzinessLevel::from_str("approximate").is_none());
    assert!(FuzzinessLevel::from_str("").is_none());
    assert!(FuzzinessLevel::from_str("very fuzzy").is_none());
}
