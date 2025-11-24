mod plugins;

use clap::{Parser, Subcommand};
use colored_text::Colorize;
use git2::Repository;

use plugins::parser;
use plugins::colors;

use std::fs;
use std::io;
use std::env;
use std::path;
use std::path::Path;

#[derive(Parser)]
#[command(version,about,long_about = None)]
struct Cli {
        #[command(subcommand)]
        command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Pull {
        link: String,
    },
    Use {
        theme: String,
    },
    List,
}

fn config_folder() -> io::Result<()> {
    let home = env::var("HOME").unwrap();
    let p = format!("{}/.config/themey/themes", home);
    fs::create_dir_all(p);
    Ok(())
}

fn list_themes(p: &str) -> Vec<String> {
    let mut v = Vec::new();
    if let Ok(entries) = fs::read_dir(Path::new(p)) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                let m = p.join("metadata.toml");
                if m.exists() {
                    if let Some(s) = p.file_name().and_then(|x| x.to_str()) {
                        v.push(s.to_string());
                    }
                }
            }
        }
    }
    v
}

fn main() {
    let cli = Cli::parse();
    let home = env::var("HOME").unwrap();
    config_folder();

    match &cli.command {
        Commands::Pull { link } => {
            let repo_name = link.split('/').last().unwrap();
            let path = format!("{}/.config/themey/themes/{}", home, repo_name);
           
            let url = "https://github.com/".to_owned() + link;
            let _repo = match Repository::clone(&url, path) {
                Ok(r) => {
                    println!("Pulling.. {}", link);
                    
                    let head = r.revparse_single("HEAD").unwrap();
                    let tree = head.peel_to_tree().unwrap();

                    if tree.get_path(path::Path::new("metadata.toml")).is_err() {
                        let err = "-> (metadata.toml not found in root)".dim();
                        eprintln!("\nwarning, {} might not be a valid theme!\n{}", link, format!("{}", err))
                    }
                },
                Err(e) => panic!("failed to clone {}", e),
            };
        },

        Commands::List => {
            let path = format!("{}/.config/themey/themes/", home);
            for d in list_themes(&path) {
                println!("- {}", d.yellow().italic());
            }
        }

        Commands::Use { theme } => {
            let base_path = format!("{}/.config/themey/themes/{}", home, theme);
            let metadata_path = format!("{}/metadata.toml", base_path);
            
            let metadata = parser::parse_metadata(&metadata_path)
                .expect("Failed to parse metadata");
            
            let theme_file = &metadata.files[0]; // "dark.toml"
            let theme_path = format!("{}/{}", base_path, theme_file);
            
            let colors = parser::parse_colors(&theme_path)
                .expect("Failed to parse colors");
            
            println!("{:#?}", colors);

            match colors::apply_theme(&theme, &home) {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to apply theme: {}", e),
            }
        }
    }
}
