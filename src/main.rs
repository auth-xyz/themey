use clap::{Parser, Subcommand};
use git2::Repository;

use std::fs;
use std::io;
use std::env;
use std::path;

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
    Apply {
        theme: String,
    },
}

fn config_folder() -> io::Result<()> {
    let home = env::var("HOME").unwrap();
    let p = format!("{}/.config/themey/themes", home);
    fs::create_dir_all(p);
    Ok(())
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
                        eprintln!("\nwarning, {} might not be a valid theme!\n-> (metadata.toml not found in root)", link)
                    }
                },
                Err(e) => panic!("failed to clone {}", e),
            };
        },

        Commands::Apply { theme } => {
        }
    }
}
