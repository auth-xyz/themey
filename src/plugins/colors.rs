use super::parser::{parse_metadata, parse_colors, Colors};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{self, Write};

pub fn apply_theme(theme_name: &str, home: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse metadata to know what to generate
    let base_path = format!("{}/.config/themey/themes/{}", home, theme_name);
    let metadata_path = format!("{}/metadata.toml", base_path);
    let metadata = parse_metadata(&metadata_path)?;
    
    // 2. Parse colors from the theme file
    let theme_file = &metadata.files[0];
    let theme_path = format!("{}/{}", base_path, theme_file);
    let colors = parse_colors(&theme_path)?;
    
    // 3. Generate files for each target
    for target in &metadata.targets {
        match target.as_str() {
            "kitty" => generate_config("kitty", &colors, home)?,
            "waybar" => generate_config("waybar", &colors, home)?,
            "hyprland" => generate_config("hyprland", &colors, home)?,
            "rofi" => generate_config("rofi", &colors, home)?,
            "dunst" => generate_config("dunst", &colors, home)?,
            "foot" => generate_config("foot", &colors, home)?,
            _ => eprintln!("Unknown target: {}", target),
        }
    }
    
    println!("Applied theme '{}' by {}", metadata.name, metadata.author);
    Ok(())
}

fn generate_config(target: &str, colors: &Colors, home: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get the config details from the template module
    let (config_path, content, reload_cmd) = super::templates::generate_config_content(target, colors, home)?;
    
    // Write the config file
    if let Some(parent) = Path::new(&config_path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&config_path, content)?;
    
    // Reload the application if needed
    if let Some((cmd, args, name)) = reload_cmd {
        match Command::new(&cmd).args(&args).output() {
            Ok(_) => println!("  ↻ Reloaded {}", name),
            Err(_) => println!("  ⚠ Could not reload {} (not running?)", name),
        }
    }
    
    Ok(())
}

pub fn preview_theme_rgb(theme_name: &str, home: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = format!("{}/.config/themey/themes/{}", home, theme_name);
    let metadata_path = format!("{}/metadata.toml", base_path);
    let metadata = parse_metadata(&metadata_path)?;
    
    let theme_file = &metadata.files[0];
    let theme_path = format!("{}/{}", base_path, theme_file);
    let colors = parse_colors(&theme_path)?;
    
    println!("\n  {} by {}\n", metadata.name, metadata.author);
    
    draw_color_panes_rgb(&colors)?;
    
    println!();
    Ok(())
}

fn draw_color_panes_rgb(colors: &Colors) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    let height = 3;
    let pane_width = 8;
    
    let normal_colors = [
        &colors.normal.black,
        &colors.normal.red,
        &colors.normal.green,
        &colors.normal.yellow,
        &colors.normal.blue,
        &colors.normal.magenta,
        &colors.normal.cyan,
        &colors.normal.white,
    ];
    
    let bright_colors = [
        &colors.bright.black,
        &colors.bright.red,
        &colors.bright.green,
        &colors.bright.yellow,
        &colors.bright.blue,
        &colors.bright.magenta,
        &colors.bright.cyan,
        &colors.bright.white,
    ];
    
    for _ in 0..height {
        for color in &normal_colors {
            let (r, g, b) = hex_to_rgb_tuple(color);
            write!(handle, "\x1b[48;2;{};{};{}m", r, g, b)?;
            write!(handle, "{}", " ".repeat(pane_width))?;
        }
        write!(handle, "\x1b[0m\n")?;
    }
    
    for _ in 0..height {
        for color in &bright_colors {
            let (r, g, b) = hex_to_rgb_tuple(color);
            write!(handle, "\x1b[48;2;{};{};{}m", r, g, b)?;
            write!(handle, "{}", " ".repeat(pane_width))?;
        }
        write!(handle, "\x1b[0m\n")?;
    }
    
    handle.flush()?;
    Ok(())
}

fn hex_to_rgb_tuple(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    (r, g, b)
}
