# Tomogotchi CLI

A CLI-based tamagotchi app built with Rust and Ratatui TUI framework.

## Features (WIP)

- 🎮 Interactive terminal UI
- 🐾 Virtual pet with mood and stats
- 🍖 Feed your pet
- 🎾 Play with your pet
- 💝 Keep your pet happy and healthy!

## Prerequisites

- Rust 1.70 or later
- A terminal that supports Unicode and colors

## Installation

```bash
cargo build --release
```

## Running

```bash
cargo run
```

Or after building:

```bash
./target/release/tomogotchi-cli
```

## Controls

- `q` or `Ctrl+C` - Quit the application

## Architecture

- `src/main.rs` - Entry point, terminal setup/teardown, main loop
- `src/app.rs` - Application state management
- `src/event.rs` - Input and tick event handling
- `src/ui.rs` - TUI rendering with Ratatui

## License

MIT
