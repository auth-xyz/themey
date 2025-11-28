mod plugins;

use clap::{CommandFactory, Parser, Subcommand};
use colored_text::Colorize;
use clap_complete::Shell;

use plugins::colors;
use plugins::utils;

use std::env;
use std::path::Path;

#[derive(Parser)]
#[command(version, about, long_about = None)]
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

    Preview {
        theme: String,
    },

    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() {
    let cli = Cli::parse();
    let home = env::var("HOME").unwrap();
    utils::config_folder().ok();
    
    match &cli.command {
        Commands::Pull { link } => {
            let repo_name = link.split('/').last().unwrap();
            let path = format!("{}/.config/themey/themes/{}", home, repo_name);
            
            utils::prep_dir(&path);

            let url = "https://github.com/".to_owned() + link;
            let r = utils::clone_pb(&url, &path);

            let head = r.revparse_single("HEAD").unwrap();
            let tree = head.peel_to_tree().unwrap();

            if tree.get_path(Path::new("metadata.toml")).is_err() {
                let err = format!("-> (metadata.toml not found in root of {})", path);
                eprintln!("\nWarning: {} might not be a valid theme!\n{}", link.blue(), err);
            }
        },
        Commands::List => {
            let path = format!("{}/.config/themey/themes/", home);
            for d in utils::list_themes(&path) {
                println!("-> {}", d.blue().italic());
            }
        },
        Commands::Use { theme } => {
            match colors::apply_theme(&theme, &home) {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to apply theme: {}", e),
            }
        },
        
        Commands::Preview { theme } => {
            match colors::preview_theme_rgb(&theme, &home) {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to preview theme: {}", e),
            }
        },

        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            utils::print_completions(*shell, &mut cmd);
        }
    }
}
