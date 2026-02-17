# rust-fuzzy-clock
Fuzzy clock in rust!

A CLI application that translates the current time into natural English text.

## Features

- 🕐 Converts system time to natural language (e.g., "three forty-seven PM")
- 🌍 Timezone-aware (uses system local time)
- 🎯 Extensible architecture for multiple languages and fuzzyness levels
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
  - Currently supported: `exact` (word-for-word translation, ignoring seconds)
  
- `-h, --help`: Print help information

### Examples

```bash
# Current time at 3:47 PM
$ rust-fuzzy-clock
three forty-seven PM

# Current time at 12:05 AM
$ rust-fuzzy-clock
twelve oh five AM

# With explicit options
$ rust-fuzzy-clock --language english --fuzzyness exact
nine twenty-three AM
```

## Architecture

The project is organized into modular components:

- `src/time/`: Time fetching logic (timezone-aware)
- `src/translator/`: Translation engine with trait-based design
  - `english.rs`: English language implementation
- `src/cli.rs`: Command-line argument parsing

## Future Enhancements

- Additional languages (Portuguese, Spanish, etc.)
- More fuzzyness levels:
  - `fuzzy`: "quarter to four", "half past three"
  - `very-fuzzy`: "about four o'clock", "around noon"
- Timezone override option
- 24-hour format support

## Dependencies

- `clap`: CLI argument parsing
- `chrono`: Timezone-aware date/time handling

## License

See LICENSE file for details.

