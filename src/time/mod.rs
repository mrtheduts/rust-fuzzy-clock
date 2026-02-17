use chrono::{Local, Timelike};

pub struct TimeInfo {
    pub hour: u32,
    pub hour24: u32,
    pub minute: u32,
    pub is_pm: bool,
}

pub fn get_current_time() -> TimeInfo {
    let now = Local::now();
    let hour24 = now.hour();
    let minute = now.minute();
    
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
