// For theme generation, still no interactive ui for color picking.

use dialoguer::{Input, MultiSelect, theme::ColorfulTheme};
use serde::Serialize;
use std::fs::{create_dir_all, write};
use std::path::Path;
use super::parser::{
    Colors,
    ColorSet,
    SpecialColors,
};

#[derive(Serialize)]
struct Theme {
    name: String,
    author: String,
    version: String,
    description: String,

    homepage: Option<String>,
    variants: Vec<String>,
    files: Vec<String>,

    targets: Vec<String>,
}

#[derive(Serialize)]
struct Root {
    theme: Theme
}

#[derive(Serialize)]
struct ColorFile {
    colors: Colors,
}

fn capture_metadata(default_name: Option<&str>) -> (Theme, String) {
    let targets = &["kitty", "waybar", "hyprland", "rofi", "dunst", "foot"];

    let name: String = if let Some(default) = default_name {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Theme name:")
            .with_initial_text(default)
            .interact_text()
            .unwrap()
    } else {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Theme name:")
            .interact_text()
            .unwrap()
    };

    let author: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author:")
        .interact_text()
        .unwrap();

    let version: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Version:")
        .with_initial_text("1.0.0")
        .interact_text()
        .unwrap();

    let desc: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Give it a short description")
        .with_initial_text("My awesome theme")
        .interact_text()
        .unwrap();

    let homepage_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("(optional) homepage:")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let homepage: Option<String> = if homepage_input.trim().is_empty() { 
        None
    } else {
        Some(homepage_input)
    };

    let raw_variants: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Variants: (white, dark, etc.)\nSeparate by ','")
        .interact_text()
        .unwrap();

    let variants: Vec<String> = raw_variants
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let raw_files: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Files: (theme_white.toml, theme_dark.toml, etc.)\nSeparate by ','")
        .interact_text()
        .unwrap();

    let files: Vec<String> = raw_files 
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let idx = MultiSelect::new()
        .items(targets)
        .interact()
        .unwrap();

    let sel: Vec<String> = idx.into_iter()
        .map(|i| targets[i].to_string())
        .collect();

    let theme_folder = name.to_lowercase().replace(" ", "-");

    let theme = Theme {
        name,
        author,
        version,
        description: desc,
        homepage,
        variants,
        files,
        targets: sel,
    };

    (theme, theme_folder)
}

fn capture_colors(variant_name: &str) -> Colors {
    println!("\n-> Capturing colors for variant: {} \n", variant_name);

    // Capture normal colors
    println!("--- Normal Colors ---");
    let normal_black: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Black:")
        .with_initial_text("#000000")
        .interact_text()
        .unwrap();

    let normal_red: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Red:")
        .with_initial_text("#ff0000")
        .interact_text()
        .unwrap();

    let normal_green: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Green:")
        .with_initial_text("#00ff00")
        .interact_text()
        .unwrap();

    let normal_yellow: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Yellow:")
        .with_initial_text("#ffff00")
        .interact_text()
        .unwrap();

    let normal_blue: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Blue:")
        .with_initial_text("#0000ff")
        .interact_text()
        .unwrap();

    let normal_magenta: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Magenta:")
        .with_initial_text("#ff00ff")
        .interact_text()
        .unwrap();

    let normal_cyan: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Cyan:")
        .with_initial_text("#00ffff")
        .interact_text()
        .unwrap();

    let normal_white: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("White:")
        .with_initial_text("#ffffff")
        .interact_text()
        .unwrap();

    // Capture bright colors
    println!("\n--- Bright Colors ---");
    let bright_black: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Black:")
        .with_initial_text("#808080")
        .interact_text()
        .unwrap();

    let bright_red: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Red:")
        .with_initial_text("#ff8080")
        .interact_text()
        .unwrap();

    let bright_green: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Green:")
        .with_initial_text("#80ff80")
        .interact_text()
        .unwrap();

    let bright_yellow: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Yellow:")
        .with_initial_text("#ffff80")
        .interact_text()
        .unwrap();

    let bright_blue: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Blue:")
        .with_initial_text("#8080ff")
        .interact_text()
        .unwrap();

    let bright_magenta: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Magenta:")
        .with_initial_text("#ff80ff")
        .interact_text()
        .unwrap();

    let bright_cyan: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Cyan:")
        .with_initial_text("#80ffff")
        .interact_text()
        .unwrap();

    let bright_white: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("White:")
        .with_initial_text("#ffffff")
        .interact_text()
        .unwrap();

    // Capture special colors
    println!("\n--- Special Colors ---");
    let background: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Background:")
        .with_initial_text("#000000")
        .interact_text()
        .unwrap();

    let foreground: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Foreground:")
        .with_initial_text("#ffffff")
        .interact_text()
        .unwrap();

    let cursor: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Cursor:")
        .with_initial_text("#ffffff")
        .interact_text()
        .unwrap();

    Colors {
        normal: ColorSet {
            black: normal_black,
            red: normal_red,
            green: normal_green,
            yellow: normal_yellow,
            blue: normal_blue,
            magenta: normal_magenta,
            cyan: normal_cyan,
            white: normal_white,
        },
        bright: ColorSet {
            black: bright_black,
            red: bright_red,
            green: bright_green,
            yellow: bright_yellow,
            blue: bright_blue,
            magenta: bright_magenta,
            cyan: bright_cyan,
            white: bright_white,
        },
        special: SpecialColors {
            background,
            foreground,
            cursor,
        },
    }
}

pub fn create_theme_package(theme_name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    println!("-> Theme Generator\n");

    // Capture metadata
    let (theme, theme_folder) = capture_metadata(theme_name);
    
    // Validate that variants and files match
    if theme.variants.len() != theme.files.len() {
        return Err("Number of variants must match number of files".into());
    }

    // Get home directory and construct theme path
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Could not determine home directory");
    
    let base_path = Path::new(&home).join(".config/themey/themes");
    create_dir_all(&base_path)?;
    
    let theme_path = base_path.join(&theme_folder);
    create_dir_all(&theme_path)?;

    // Capture colors for each variant and write theme files
    let mut color_files = Vec::new();
    for (variant, filename) in theme.variants.iter().zip(theme.files.iter()) {
        let colors = capture_colors(variant);
        
        let color_file = ColorFile { colors };
        let toml_str = toml::to_string_pretty(&color_file)?;
        
        let file_path = theme_path.join(filename);
        write(&file_path, toml_str)?;
        
        println!("\n✓ Created: {}", file_path.display());
        
        color_files.push(color_file);
    }

    // Write metadata.toml
    let root = Root { theme };
    let metadata_str = toml::to_string_pretty(&root)?;
    let metadata_path = theme_path.join("metadata.toml");
    write(&metadata_path, metadata_str)?;

    println!("\n✓ Created: {}", metadata_path.display());
    println!("\n-> Theme package created successfully in '{}'", theme_folder);

    Ok(())
}
