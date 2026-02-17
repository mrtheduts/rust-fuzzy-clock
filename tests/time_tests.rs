use rust_fuzzy_clock::time::TimeInfo;

// Helper function to create TimeInfo for testing
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
fn test_midnight_conversion() {
    let time = create_time_info(0, 0);
    assert_eq!(time.hour, 12);
    assert_eq!(time.hour24, 0);
    assert_eq!(time.is_pm, false);
}

#[test]
fn test_noon_conversion() {
    let time = create_time_info(12, 0);
    assert_eq!(time.hour, 12);
    assert_eq!(time.hour24, 12);
    assert_eq!(time.is_pm, true);
}

#[test]
fn test_morning_hours() {
    let time = create_time_info(9, 30);
    assert_eq!(time.hour, 9);
    assert_eq!(time.hour24, 9);
    assert_eq!(time.is_pm, false);
}

#[test]
fn test_afternoon_hours() {
    let time = create_time_info(15, 45);
    assert_eq!(time.hour, 3);
    assert_eq!(time.hour24, 15);
    assert_eq!(time.is_pm, true);
}

#[test]
fn test_evening_hours() {
    let time = create_time_info(23, 59);
    assert_eq!(time.hour, 11);
    assert_eq!(time.hour24, 23);
    assert_eq!(time.is_pm, true);
}

#[test]
fn test_hour_boundaries() {
    // Test boundary between 11 AM and 12 PM
    let time11 = create_time_info(11, 59);
    assert_eq!(time11.hour, 11);
    assert_eq!(time11.is_pm, false);
    
    let time12 = create_time_info(12, 0);
    assert_eq!(time12.hour, 12);
    assert_eq!(time12.is_pm, true);
}

#[test]
fn test_one_am() {
    let time = create_time_info(1, 0);
    assert_eq!(time.hour, 1);
    assert_eq!(time.hour24, 1);
    assert_eq!(time.is_pm, false);
}

#[test]
fn test_one_pm() {
    let time = create_time_info(13, 0);
    assert_eq!(time.hour, 1);
    assert_eq!(time.hour24, 13);
    assert_eq!(time.is_pm, true);
}
