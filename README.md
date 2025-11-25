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

### Generate Shell Completions
```bash
themey completions <shell>
```
Generate shell completion scripts for bash, zsh, fish, or powershell.

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

## Creating Your Own Theme

Creating a theme for Themey is straightforward. A theme is a GitHub repository containing at minimum two files: `metadata.toml` and one or more theme variant files.

### Basic Theme Structure

```
my-awesome-theme/
├── metadata.toml        # Required: Theme information and configuration
├── dark.toml           # Theme variant file
├── light.toml          # Optional: Additional variants
└── README.md           # Optional: Theme documentation
```

### Step-by-Step Guide

#### 1. Create a New Repository

Create a new GitHub repository for your theme. The repository name will be used when pulling the theme with `themey pull`.

#### 2. Create `metadata.toml`

This file contains all the information about your theme and tells Themey how to process it.

```toml
[theme]
name = "My Awesome Theme"
author = "Your Name"
version = "1.0.0"
description = "A brief description of your theme"
homepage = "https://github.com/yourusername/my-awesome-theme"  # Optional

variants = ["dark", "light"]           # List of available variants
files = ["dark.toml", "light.toml"]    # Corresponding theme files

targets = [                            # Applications to generate configs for
    "waybar",
    "hyprland",
    "kitty",
    "rofi",
    "dunst",
]
```

#### 3. Create Theme Variant Files

Each variant file (e.g., `dark.toml`) defines the actual colors for that variant.

```toml
[colors.normal]
black   = "#1e1e2e"
red     = "#f7768e"
green   = "#9ece6a"
yellow  = "#e0af68"
blue    = "#7aa2f7"
magenta = "#bb9af7"
cyan    = "#7dcfff"
white   = "#a9b1d6"

[colors.bright]
black   = "#6c6f93"
red     = "#f7768e"
green   = "#9ece6a"
yellow  = "#e0af68"
blue    = "#7aa2f7"
magenta = "#bb9af7"
cyan    = "#7dcfff"
white   = "#a9b1d6"

[colors.special]
background = "#1e1e2e"
foreground = "#a9b1d6"
cursor     = "#bb9af7"
```

#### 4. Test Your Theme

```bash
# Pull your theme
themey pull yourusername/my-awesome-theme

# List themes to verify it appears
themey list

# Apply your theme
themey use my-awesome-theme
```

---

## Metadata Specification

### Required Fields

#### `[theme]` Section

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `name` | String | Display name of the theme | `"NeoSleek"` |
| `author` | String | Theme creator's name | `"Auth P"` |
| `version` | String | Semantic version number | `"1.0.0"` |
| `description` | String | Brief theme description | `"A sleek, modern dark theme"` |
| `variants` | Array | List of available variants | `["dark", "light"]` |
| `files` | Array | Theme files corresponding to variants | `["dark.toml", "light.toml"]` |
| `targets` | Array | Target applications for config generation | `["kitty", "rofi"]` |

### Optional Fields

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `homepage` | String | Project or repository URL | `"https://github.com/..."` |

### Supported Targets

Themey can generate configurations for the following applications:

- `waybar` - Status bar for Wayland
- `hyprland` - Dynamic tiling Wayland compositor
- `kitty` - GPU-accelerated terminal emulator
- `rofi` - Application launcher
- `dunst` - Notification daemon

### Theme File Specification

Each theme file must contain three color sections:

#### `[colors.normal]`
Standard terminal colors (8 colors):
- `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`

#### `[colors.bright]`
Bright variants of the standard colors (8 colors):
- `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`

#### `[colors.special]`
Special UI colors (minimum 3 colors):
- `background` - Default background color
- `foreground` - Default foreground/text color
- `cursor` - Cursor color

**Color Format:** All colors must be in hex format: `#RRGGBB`

### Validation

Themey automatically validates:
- Presence of `metadata.toml` in the repository root
- Required fields in metadata
- Valid TOML syntax
- Existence of specified variant files

---

## Usage Example

```bash
# Pull a theme from GitHub
themey pull auth-xyz/neosleek-theme

# List all installed themes
themey list

# Apply the theme
themey use neosleek-theme

# Generate shell completions
themey completions bash > /etc/bash_completion.d/themey
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
* `clap_complete` - Shell completion generation
* `indicatif` - For the progress bar 
 
---

## Contributing

Contributions are welcome! Please ensure:
* Themes include proper `metadata.toml`
* Changes are documented

---

## License

Licensed under the **MIT License**.
You are free to use, modify, and redistribute this tool.

See [LICENSE](LICENSE) for more information.

---

## Acknowledgments

- Inspired by [pywal](https://github.com/dylanaraps/pywal) for unified theme management

---

## Roadmap

- [ ] Support for more applications (vim, nvim, tmux, etc.)
- [ ] Theme preview functionality
- [ ] Interactive theme creation wizard
- [ ] Theme marketplace/registry
- [ ] Export themes to other formats
- [ ] Hot-reload theme changes
