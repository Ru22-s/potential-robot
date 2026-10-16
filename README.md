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

### Windows Setup & Building

To build and run Llama Chat UI on Windows:

#### Method 1: Using MSYS2 (Recommended for GTK4 & Libadwaita on Windows)

1. Download and install [MSYS2](https://www.msys2.org/).
2. Open the **MSYS2 UCRT64** terminal and update package databases:
   ```bash
   pacman -Syu
   ```
3. Install required toolchain and libraries:
   ```bash
   pacman -S mingw-w64-ucrt-x86_64-toolchain \
             mingw-w64-ucrt-x86_64-gtk4 \
             mingw-w64-ucrt-x86_64-libadwaita \
             mingw-w64-ucrt-x86_64-pkgconf
   ```
4. Install Rust via [rustup](https://rustup.rs/) (select `x86_64-pc-windows-gnu` host or target).
5. Ensure `C:\msys64\ucrt64\bin` is added to your Windows `%PATH%` environment variable.
6. Build and run in Command Prompt / PowerShell / UCRT64 shell:
   ```cmd
   cargo run
   ```

#### Method 2: Running Ollama on Windows

1. Download and install Ollama for Windows from [ollama.com](https://ollama.com).
2. Open Command Prompt or PowerShell and start the model:
   ```cmd
   ollama run llama3
   ```
3. Launch Llama Chat UI!

## Build & Run

```bash
cargo build
cargo run
```
