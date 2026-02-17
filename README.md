# rust-fuzzy-clock
Fuzzy clock in rust!

A CLI application that translates the current time into natural English text.

## Features

- 🕐 Converts system time to natural language with multiple fuzzyness levels
- 🎚️ **Four fuzzyness levels**:
  - `exact`: Word-for-word translation (e.g., "three forty-seven PM")
  - `fuzzy`: Natural expressions (e.g., "quarter to four PM")
  - `very-fuzzy`: Approximate time (e.g., "about quarter to four")
  - `max-fuzzy`: Time period only (e.g., "morning", "afternoon", "evening", "night")
- 🌍 Timezone-aware (uses system local time)
- 🎯 Extensible architecture for multiple languages
- ⚡ Fast and lightweight

## Installation

Build from source:

```bash
cargo build --release
```

**Note for Termux/Android users**: Due to `noexec` restrictions on `/storage/emulated`, use:
```bash
CARGO_TARGET_DIR=/data/data/com.termux/files/usr/tmp/rust-fuzzy-clock-build cargo build
```

## Usage

Basic usage (prints current time in natural English):
```bash
rust-fuzzy-clock
```

With options:
```bash
rust-fuzzy-clock --language english --fuzzyness exact
```

### CLI Options

- `-l, --language <LANGUAGE>`: Language for time translation (default: `english`)
  - Currently supported: `english`, `en`
  
- `-f, --fuzzyness <FUZZYNESS>`: Level of fuzzyness (default: `exact`)
  - `exact`: Word-for-word time translation, ignoring seconds (e.g., "three forty-seven PM")
  - `fuzzy`: Natural time expressions (e.g., "quarter past three PM", "half past nine AM")
  - `very-fuzzy`: Approximate descriptions (e.g., "about quarter to four", "almost noon")
  - `max-fuzzy`: Time period only based on hour:
    - "morning" (5 AM - 11:59 AM)
    - "afternoon" (12 PM - 4:59 PM)
    - "evening" (5 PM - 9 PM)
    - "night" (9 PM - 4:59 AM)
  
- `-h, --help`: Print help information

### Examples

```bash
# Exact level - word-for-word translation
$ rust-fuzzy-clock
three forty-seven PM

$ rust-fuzzy-clock --fuzzyness exact
twelve oh five AM

# Fuzzy level - natural expressions
$ rust-fuzzy-clock --fuzzyness fuzzy
quarter past nine AM

$ rust-fuzzy-clock -f fuzzy
half past three PM

# Very fuzzy level - approximate time
$ rust-fuzzy-clock --fuzzyness very-fuzzy
about quarter to four

$ rust-fuzzy-clock -f very-fuzzy
almost noon

# Max fuzzy level - time period only
$ rust-fuzzy-clock --fuzzyness max-fuzzy
morning

$ rust-fuzzy-clock -f max-fuzzy
evening

# With explicit language
$ rust-fuzzy-clock --language english --fuzzyness fuzzy
quarter to four PM
```

## Architecture

The project is organized into modular components:

- `src/time/`: Time fetching logic (timezone-aware)
- `src/translator/`: Translation engine with trait-based design
  - `english.rs`: English language implementation
- `src/cli.rs`: Command-line argument parsing

## Future Enhancements

- Additional languages (Portuguese, Spanish, etc.)
- Timezone override option
- 24-hour format support
- Configurable time period ranges for max-fuzzy mode

## Dependencies

- `clap`: CLI argument parsing
- `chrono`: Timezone-aware date/time handling

## License

See LICENSE file for details.

