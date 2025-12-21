use dialoguer::{Input, MultiSelect, theme::ColorfulTheme};
use serde::Serialize;
use std::fs::{create_dir_all, write};
use std::path::Path;
use super::parser::{
    Colors,
    ColorSet,
    SpecialColors,
    parse_metadata,
    parse_colors,
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
    let targets = &["kitty", "waybar", "hyprland", "rofi", "dunst", "foot", "gtk", "neovim"];

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

fn update_colors_interactive(variant_name: &str, existing_colors: &Colors) -> Colors {
    println!("\n-> Updating colors for variant: {}\n", variant_name);
    println!("Press Enter to keep existing value, or type new value to change.\n");

    // Update normal colors
    println!("--- Normal Colors ---");
    let normal_black: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Black:")
        .with_initial_text(&existing_colors.normal.black)
        .interact_text()
        .unwrap();

    let normal_red: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Red:")
        .with_initial_text(&existing_colors.normal.red)
        .interact_text()
        .unwrap();

    let normal_green: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Green:")
        .with_initial_text(&existing_colors.normal.green)
        .interact_text()
        .unwrap();

    let normal_yellow: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Yellow:")
        .with_initial_text(&existing_colors.normal.yellow)
        .interact_text()
        .unwrap();

    let normal_blue: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Blue:")
        .with_initial_text(&existing_colors.normal.blue)
        .interact_text()
        .unwrap();

    let normal_magenta: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Magenta:")
        .with_initial_text(&existing_colors.normal.magenta)
        .interact_text()
        .unwrap();

    let normal_cyan: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Cyan:")
        .with_initial_text(&existing_colors.normal.cyan)
        .interact_text()
        .unwrap();

    let normal_white: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("White:")
        .with_initial_text(&existing_colors.normal.white)
        .interact_text()
        .unwrap();

    // Update bright colors
    println!("\n--- Bright Colors ---");
    let bright_black: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Black:")
        .with_initial_text(&existing_colors.bright.black)
        .interact_text()
        .unwrap();

    let bright_red: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Red:")
        .with_initial_text(&existing_colors.bright.red)
        .interact_text()
        .unwrap();

    let bright_green: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Green:")
        .with_initial_text(&existing_colors.bright.green)
        .interact_text()
        .unwrap();

    let bright_yellow: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Yellow:")
        .with_initial_text(&existing_colors.bright.yellow)
        .interact_text()
        .unwrap();

    let bright_blue: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Blue:")
        .with_initial_text(&existing_colors.bright.blue)
        .interact_text()
        .unwrap();

    let bright_magenta: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Magenta:")
        .with_initial_text(&existing_colors.bright.magenta)
        .interact_text()
        .unwrap();

    let bright_cyan: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Cyan:")
        .with_initial_text(&existing_colors.bright.cyan)
        .interact_text()
        .unwrap();

    let bright_white: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("White:")
        .with_initial_text(&existing_colors.bright.white)
        .interact_text()
        .unwrap();

    // Update special colors
    println!("\n--- Special Colors ---");
    let background: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Background:")
        .with_initial_text(&existing_colors.special.background)
        .interact_text()
        .unwrap();

    let foreground: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Foreground:")
        .with_initial_text(&existing_colors.special.foreground)
        .interact_text()
        .unwrap();

    let cursor: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Cursor:")
        .with_initial_text(&existing_colors.special.cursor)
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

pub fn update_theme_package(theme_name: &str, home: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("-> Theme Updater\n");

    // Load existing theme
    let base_path = format!("{}/.config/themey/themes/{}", home, theme_name);
    let metadata_path = format!("{}/metadata.toml", base_path);

    if !Path::new(&metadata_path).exists() {
        return Err(format!("Theme '{}' not found", theme_name).into());
    }

    let existing_metadata = parse_metadata(&metadata_path)?;

    println!("Loaded theme: {} by {}\n", existing_metadata.name, existing_metadata.author);

    // Ask what to update
    let update_options = &[
        "Update metadata (name, author, version, etc.)",
        "Update colors for variants",
        "Update targets (applications)",
    ];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to update? (Space to select, Enter to confirm)")
        .items(update_options)
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("No changes selected. Exiting.");
        return Ok(());
    }

    let update_metadata = selections.contains(&0);
    let update_colors = selections.contains(&1);
    let update_targets = selections.contains(&2);

    // Update metadata if selected
    let mut theme = if update_metadata {
        println!("\n-> Updating Metadata\n");

        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Theme name:")
            .with_initial_text(&existing_metadata.name)
            .interact_text()
            .unwrap();

        let author: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Author:")
            .with_initial_text(&existing_metadata.author)
            .interact_text()
            .unwrap();

        let version: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Version:")
            .with_initial_text("1.0.0")
            .interact_text()
            .unwrap();

        // For optional fields, we need to handle None
        let homepage_default = existing_metadata.files.get(0).map(|_| "").unwrap_or("");
        let homepage_input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("(optional) homepage:")
            .with_initial_text(homepage_default)
            .allow_empty(true)
            .interact_text()
            .unwrap();

        let homepage: Option<String> = if homepage_input.trim().is_empty() {
            None
        } else {
            Some(homepage_input)
        };

        Theme {
            name,
            author,
            version,
            description: existing_metadata.name.clone(), // Keeping description simple
            homepage,
            variants: existing_metadata.files.iter().map(|f| f.replace(".toml", "")).collect(),
            files: existing_metadata.files.clone(),
            targets: existing_metadata.targets.clone(),
        }
    } else {
        // Keep existing metadata
        Theme {
            name: existing_metadata.name,
            author: existing_metadata.author,
            version: "1.0.0".to_string(),
            description: "".to_string(),
            homepage: None,
            variants: existing_metadata.files.iter().map(|f| f.replace(".toml", "")).collect(),
            files: existing_metadata.files.clone(),
            targets: existing_metadata.targets.clone(),
        }
    };

    // Update targets if selected
    if update_targets {
        println!("\n-> Updating Targets\n");
        let all_targets = &["kitty", "waybar", "hyprland", "rofi", "dunst", "foot", "gtk"];
        let defaults: Vec<bool> = all_targets.iter()
            .map(|t| theme.targets.contains(&t.to_string()))
            .collect();

        let idx = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select targets")
            .items(all_targets)
            .defaults(&defaults)
            .interact()
            .unwrap();

        let sel: Vec<String> = idx.into_iter()
            .map(|i| all_targets[i].to_string())
            .collect();

        theme.targets = sel;
    }

    // Update colors if selected
    if update_colors {
        println!("\n-> Updating Colors\n");

        // Ask which variants to update
        let variant_names: Vec<String> = theme.files.iter()
            .map(|f| f.replace(".toml", ""))
            .collect();

        let variant_selections = if variant_names.len() > 1 {
            MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Which variants would you like to update?")
                .items(&variant_names)
                .interact()
                .unwrap()
        } else {
            vec![0] // Update the only variant
        };

        for idx in variant_selections {
            let variant = &variant_names[idx];
            let filename = &theme.files[idx];
            let theme_path = format!("{}/{}", base_path, filename);

            // Load existing colors
            let existing_colors = parse_colors(&theme_path)?;

            // Update colors interactively
            let updated_colors = update_colors_interactive(variant, &existing_colors);

            // Write updated colors
            let color_file = ColorFile { colors: updated_colors };
            let toml_str = toml::to_string_pretty(&color_file)?;
            write(&theme_path, toml_str)?;

            println!("\n✓ Updated: {}", theme_path);
        }
    }

    // Write updated metadata
    let root = Root { theme };
    let metadata_str = toml::to_string_pretty(&root)?;
    write(&metadata_path, metadata_str)?;

    println!("\n✓ Updated: {}", metadata_path);
    println!("\n-> Theme '{}' updated successfully!", theme_name);

    Ok(())
}