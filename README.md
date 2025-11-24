<p align="center">
<a href="https://git.io/typing-svg">
<img src="https://readme-typing-svg.herokuapp.com?font=JetBrains+Mono&weight=800&pause=1000&color=FFFFFF&background=FF6B6B&center=true&vCenter=true&width=380&lines=Themey" alt="Typing SVG" />
</a>
<br/>
<img src="https://img.shields.io/badge/LANGUAGE-RUST-FF6B6B?style=for-the-badge&labelColor=1A1A1A&color=FF6B6B"/>
<img src="https://img.shields.io/badge/CLI-THEME_MANAGER-FF6B6B?style=for-the-badge&labelColor=1A1A1A&color=FF6B6B"/>
</p>

---

## Overview

**Themey** is a lightweight CLI tool written in Rust for managing and applying color themes across your system.
It provides a simple interface to pull themes from GitHub repositories, list installed themes, and apply them to your environment.

---

## Features

| Feature           | Description                                      |
| ----------------- | ------------------------------------------------ |
| **Pull Themes**   | Clone theme repositories from GitHub             |
| **List Themes**   | Display all installed themes                     |
| **Apply Themes**  | Parse and apply color schemes from TOML files    |
| **Validation**    | Automatically checks for valid theme structure   |

---

## Commands

### Pull a Theme
```bash
themey pull <user>/<repo>
```
Clone a theme repository from GitHub into `~/.config/themey/themes/`.

### List Installed Themes
```bash
themey list
```
Display all themes with valid `metadata.toml` files.

### Apply a Theme
```bash
themey use <theme-name>
```
Parse and apply the specified theme to your system.

---

## Installation

### From Source

Clone and build:

```bash
git clone https://github.com/yourusername/themey.git
cd themey
cargo build --release
```

Install binary:

```bash
sudo cp target/release/themey /usr/local/bin/
```

---

## Theme Structure

A valid theme repository must contain:

```
theme-repo/
├── metadata.toml
└── dark.toml (or other theme files)
```

### Example `metadata.toml`
```toml
[theme]
name = "my theme"
author = "someone?"
version = "1.0.0"
description = "yes"
homepage = "sdaopojfaw" <- completly optional

variants = ["dark"]
files = ["dark.toml"] <- the name of the theme file

targets = [ <- the config themey is supposed to generate
    "waybar",
    "hyprland",
    "kitty",
    "rofi",
    "dunst",
]
```

### Example `dark.toml`
```toml
[colors.normal]
black   = "#1e1e2e"
red     = "#f7768e"
green   = "#9ece6a"
# ... more colors
```

---

## Requirements

* **Rust (>=1.70)**
* **Git**
* **cargo**

Dependencies managed via `Cargo.toml`:
* `clap` - Command-line argument parsing
* `colored_text` - Terminal color output
* `git2` - Git repository operations
* `toml` - TOML parsing

---

## Usage Example

```bash
# Pull a theme from GitHub
themey pull auth-xyz/nord-theme

# List all installed themes
themey list

# Apply the theme
themey use nord-theme
```

---

## License

Licensed under the **MIT License**.
You are free to use, modify, and redistribute this tool.

---

## Contributing

Contributions are welcome! Please ensure:
* Themes include proper `metadata.toml`
* Changes are documented

