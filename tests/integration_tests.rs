use std::process::Command;
use std::path::PathBuf;

fn get_bin_path() -> PathBuf {
    // Try the cargo-provided environment variable first (for CI/CD)
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_rust_fuzzy_clock") {
        return PathBuf::from(path);
    }
    
    // Fallback to target directory relative to manifest (for local builds)
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    
    // Check if we're using a custom CARGO_TARGET_DIR
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| format!("{}/target", manifest_dir));
    
    // Try debug build first, then release
    let debug_path = PathBuf::from(&target_dir).join("debug/rust-fuzzy-clock");
    if debug_path.exists() {
        return debug_path;
    }
    
    let release_path = PathBuf::from(&target_dir).join("release/rust-fuzzy-clock");
    if release_path.exists() {
        return release_path;
    }
    
    panic!("Could not find rust-fuzzy-clock binary in {:?}", target_dir);
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
