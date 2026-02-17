use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rust-fuzzy-clock")]
#[command(about = "A fuzzy clock that translates time into natural language", long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "english")]
    #[arg(help = "Language for time translation (english, spanish, portuguese)")]
    pub language: String,

    #[arg(short, long, default_value = "exact")]
    #[arg(help = "Level of fuzziness (exact, fuzzy, very-fuzzy, max-fuzzy)")]
    pub fuzziness: String,

    #[arg(long = "24-hour", default_value_t = false)]
    #[arg(help = "Use 24-hour format instead of 12-hour with AM/PM")]
    pub hour_24: bool,

    #[arg(long = "include-units", default_value_t = false)]
    #[arg(help = "Include 'hours' and 'minutes' labels in output")]
    pub include_units: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
