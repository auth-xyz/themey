use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct MetadataFile {
    pub theme: Metadata,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub author: String,
    pub files: Vec<String>,
    pub targets: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ColorFile {
    pub colors: Colors,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Colors {
    pub normal: ColorSet,
    pub bright: ColorSet,
    pub special: SpecialColors,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorSet {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecialColors {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

pub fn parse_metadata(path: &str) -> Result<Metadata, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let metadata_file: MetadataFile = toml::from_str(&contents)?;
    Ok(metadata_file.theme)
}

pub fn parse_colors(theme: &str) -> Result<Colors, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(theme)?;
    let color_file: ColorFile = toml::from_str(&contents)?;
    Ok(color_file.colors)
}
