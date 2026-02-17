use rust_fuzzy_clock::{cli, time, translator};
use translator::{FuzzinessLevel, Language, get_translator};

fn main() {
    let args = cli::parse_args();

    let language = Language::from_str(&args.language).unwrap_or_else(|| {
        eprintln!("Error: Unknown language '{}'", args.language);
        std::process::exit(1);
    });

    let fuzziness = FuzzinessLevel::from_str(&args.fuzziness).unwrap_or_else(|| {
        eprintln!("Error: Unknown fuzziness level '{}'", args.fuzziness);
        std::process::exit(1);
    });

    let current_time = time::get_current_time();
    let translator = get_translator(language);
    let fuzzy_time =
        translator.translate(&current_time, fuzziness, args.hour_24, args.include_units);

    println!("{}", fuzzy_time);
}
