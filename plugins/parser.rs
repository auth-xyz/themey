use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub variant: Vec<String>,
    pub targets: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Colors {
    pub normal: ColorSet,
    pub bright: ColorSet,
    pub special: SpecialColors,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct SpecialColors {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

pub fn parse_metadata(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    
}


