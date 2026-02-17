use rust_fuzzy_clock::{cli, time, translator};
use translator::{FuzzynessLevel, Language, get_translator};

fn main() {
    let args = cli::parse_args();
    
    let language = Language::from_str(&args.language)
        .unwrap_or_else(|| {
            eprintln!("Error: Unknown language '{}'", args.language);
            std::process::exit(1);
        });
    
    let fuzzyness = FuzzynessLevel::from_str(&args.fuzzyness)
        .unwrap_or_else(|| {
            eprintln!("Error: Unknown fuzzyness level '{}'", args.fuzzyness);
            std::process::exit(1);
        });
    
    let current_time = time::get_current_time();
    let translator = get_translator(language);
    let fuzzy_time = translator.translate(&current_time, fuzzyness, args.hour_24);
    
    println!("{}", fuzzy_time);
}
