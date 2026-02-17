use std::process::Command;

fn get_bin_path() -> String {
    // Use the compiled binary from target directory
    if cfg!(debug_assertions) {
        "/data/data/com.termux/files/usr/tmp/rust-fuzzy-clock-build/debug/rust-fuzzy-clock".to_string()
    } else {
        "/data/data/com.termux/files/usr/tmp/rust-fuzzy-clock-build/release/rust-fuzzy-clock".to_string()
    }
}

#[test]
fn test_cli_default_options() {
    let output = Command::new(get_bin_path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should contain AM or PM for 12-hour format
    assert!(stdout.contains("AM") || stdout.contains("PM"));
}

#[test]
fn test_cli_english_exact() {
    let output = Command::new(get_bin_path())
        .args(&["-l", "english", "-f", "exact"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AM") || stdout.contains("PM"));
}

#[test]
fn test_cli_24_hour_format() {
    let output = Command::new(get_bin_path())
        .args(&["-l", "english", "-f", "exact", "--24-hour"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should NOT contain AM or PM in 24-hour format
    assert!(!stdout.contains("AM"));
    assert!(!stdout.contains("PM"));
}

#[test]
fn test_cli_spanish() {
    let output = Command::new(get_bin_path())
        .args(&["-l", "spanish", "-f", "exact"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Spanish output should exist (not empty)
    assert!(!stdout.trim().is_empty());
}

#[test]
fn test_cli_portuguese() {
    let output = Command::new(get_bin_path())
        .args(&["-l", "portuguese", "-f", "exact"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Portuguese output should exist (not empty)
    assert!(!stdout.trim().is_empty());
}

#[test]
fn test_cli_max_fuzzy() {
    let output = Command::new(get_bin_path())
        .args(&["-l", "english", "-f", "max-fuzzy"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let result = stdout.trim();
    
    // Should be one of the time periods
    assert!(
        result == "morning" || 
        result == "afternoon" || 
        result == "evening" || 
        result == "night"
    );
}

#[test]
fn test_cli_invalid_language() {
    let output = Command::new(get_bin_path())
        .args(&["-l", "french"])
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown language"));
}

#[test]
fn test_cli_invalid_fuzzyness() {
    let output = Command::new(get_bin_path())
        .args(&["-f", "super-fuzzy"])
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown fuzzyness level"));
}

#[test]
fn test_cli_help() {
    let output = Command::new(get_bin_path())
        .args(&["--help"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(stdout.contains("fuzzy clock"));
    assert!(stdout.contains("--language"));
    assert!(stdout.contains("--fuzzyness"));
    assert!(stdout.contains("--24-hour"));
}

#[test]
fn test_cli_short_flags() {
    let output = Command::new(get_bin_path())
        .args(&["-l", "en", "-f", "fuzzy"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
}

#[test]
fn test_cli_all_fuzzyness_levels() {
    for level in &["exact", "fuzzy", "very-fuzzy", "max-fuzzy"] {
        let output = Command::new(get_bin_path())
            .args(&["-f", level])
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success(), "Failed for fuzzyness level: {}", level);
    }
}

#[test]
fn test_cli_all_languages() {
    for lang in &["english", "spanish", "portuguese"] {
        let output = Command::new(get_bin_path())
            .args(&["-l", lang])
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success(), "Failed for language: {}", lang);
    }
}
