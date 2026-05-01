# Llama Chat UI

A simple, modern Rust desktop application to chat with local Ollama models.

## Features

- Clean UI using GTK4 and Libadwaita
- Connects to local Ollama API (default: `http://localhost:11434`)
- Asynchronous communication with the Ollama model for responsive UI
- Uses the `llama3` model by default (can be changed in source code)

## Dependencies

Before building, ensure you have the required GTK4 and Libadwaita development libraries installed on your system.

On Ubuntu/Debian:
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev
```

## Build & Run

```bash
cargo build
cargo run
```
