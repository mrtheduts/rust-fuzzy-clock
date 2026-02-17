# rust-fuzzy-clock
Fuzzy clock in rust!

A CLI application that translates the current time into natural English text.

## Features

- 🕐 Converts system time to natural language with multiple fuzzyness levels
- 🌍 **Three languages supported**:
  - English
  - Spanish (Latin American)
  - Portuguese (Brazilian)
- 🎚️ **Four fuzzyness levels**:
  - `exact`: Word-for-word translation (e.g., "three forty-seven PM")
  - `fuzzy`: Natural expressions (e.g., "quarter to four PM")
  - `very-fuzzy`: Approximate time (e.g., "about quarter to four")
  - `max-fuzzy`: Time period only (e.g., "morning", "afternoon", "evening", "night")
- ⏰ Timezone-aware (uses system local time)
- 🎯 Extensible architecture for adding more languages
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
  - Supported languages:
    - `english` or `en`: English
    - `spanish`, `es`, or `español`: Spanish (Latin American variation)
    - `portuguese`, `pt`, or `português`: Portuguese (Brazilian variation)
  
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

#### English
```bash
$ rust-fuzzy-clock --language english --fuzzyness exact
three forty-seven PM

$ rust-fuzzy-clock -l en -f fuzzy
quarter past nine AM

$ rust-fuzzy-clock -l en -f very-fuzzy
about half past three

$ rust-fuzzy-clock -l en -f max-fuzzy
afternoon
```

#### Spanish (Latin American)
```bash
$ rust-fuzzy-clock --language spanish --fuzzyness exact
tres cuarenta y siete PM

$ rust-fuzzy-clock -l es -f fuzzy
nueve y cuarto AM

$ rust-fuzzy-clock -l español -f very-fuzzy
como tres y media

$ rust-fuzzy-clock -l es -f max-fuzzy
tarde
```

#### Portuguese (Brazilian)
```bash
$ rust-fuzzy-clock --language portuguese --fuzzyness exact
três quarenta e sete PM

$ rust-fuzzy-clock -l pt -f fuzzy
nove e quinze da manhã

$ rust-fuzzy-clock -l português -f very-fuzzy
por volta de três e meia

$ rust-fuzzy-clock -l pt -f max-fuzzy
tarde
```

## Architecture

The project is organized into modular components:

- `src/time/`: Time fetching logic (timezone-aware)
- `src/translator/`: Translation engine with trait-based design
  - `english.rs`: English language implementation
  - `spanish.rs`: Spanish (Latin American) language implementation
  - `portuguese.rs`: Portuguese (Brazilian) language implementation
- `src/cli.rs`: Command-line argument parsing

Each language translator implements all four fuzzyness levels independently.

## Future Enhancements

- Additional languages (French, German, Italian, etc.)
- Timezone override option
- 24-hour format support
- Configurable time period ranges for max-fuzzy mode
- Color output support

## Dependencies

- `clap`: CLI argument parsing
- `chrono`: Timezone-aware date/time handling

## License

See LICENSE file for details.

