use rust_fuzzy_clock::translator::{FuzzinessLevel, Language};

#[test]
fn test_language_parsing_english() {
    assert!(matches!(
        Language::parse("english"),
        Some(Language::English)
    ));
    assert!(matches!(Language::parse("en"), Some(Language::English)));
    assert!(matches!(
        Language::parse("English"),
        Some(Language::English)
    ));
    assert!(matches!(Language::parse("EN"), Some(Language::English)));
}

#[test]
fn test_language_parsing_spanish() {
    assert!(matches!(
        Language::parse("spanish"),
        Some(Language::Spanish)
    ));
    assert!(matches!(Language::parse("es"), Some(Language::Spanish)));
    assert!(matches!(
        Language::parse("español"),
        Some(Language::Spanish)
    ));
    assert!(matches!(
        Language::parse("SPANISH"),
        Some(Language::Spanish)
    ));
}

#[test]
fn test_language_parsing_portuguese() {
    assert!(matches!(
        Language::parse("portuguese"),
        Some(Language::Portuguese)
    ));
    assert!(matches!(Language::parse("pt"), Some(Language::Portuguese)));
    assert!(matches!(
        Language::parse("português"),
        Some(Language::Portuguese)
    ));
    assert!(matches!(
        Language::parse("PORTUGUESE"),
        Some(Language::Portuguese)
    ));
}

#[test]
fn test_language_parsing_invalid() {
    assert!(Language::parse("french").is_none());
    assert!(Language::parse("fr").is_none());
    assert!(Language::parse("").is_none());
    assert!(Language::parse("xxx").is_none());
}

#[test]
fn test_fuzziness_parsing_valid() {
    assert!(matches!(
        FuzzinessLevel::parse("exact"),
        Some(FuzzinessLevel::Exact)
    ));
    assert!(matches!(
        FuzzinessLevel::parse("fuzzy"),
        Some(FuzzinessLevel::Fuzzy)
    ));
    assert!(matches!(
        FuzzinessLevel::parse("very-fuzzy"),
        Some(FuzzinessLevel::VeryFuzzy)
    ));
    assert!(matches!(
        FuzzinessLevel::parse("max-fuzzy"),
        Some(FuzzinessLevel::MaxFuzzy)
    ));
}

#[test]
fn test_fuzziness_parsing_case_insensitive() {
    assert!(matches!(
        FuzzinessLevel::parse("EXACT"),
        Some(FuzzinessLevel::Exact)
    ));
    assert!(matches!(
        FuzzinessLevel::parse("Fuzzy"),
        Some(FuzzinessLevel::Fuzzy)
    ));
    assert!(matches!(
        FuzzinessLevel::parse("VERY-FUZZY"),
        Some(FuzzinessLevel::VeryFuzzy)
    ));
}

#[test]
fn test_fuzziness_parsing_invalid() {
    assert!(FuzzinessLevel::parse("approximate").is_none());
    assert!(FuzzinessLevel::parse("").is_none());
    assert!(FuzzinessLevel::parse("very fuzzy").is_none());
}
