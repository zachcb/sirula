use serde_derive::Deserialize;
use serde::{de::Error, Deserializer};
use super::consts::*;
use super::util::get_config_file;
use pango::Attribute;

fn default_side() -> Side { Side::Right }
fn default_markup_default() -> Vec<Attribute> { Vec::new() }
fn default_markup_highlight() -> Vec<Attribute> { parse_attributes("foreground=\"red\" underline=\"double\"").unwrap() }
fn default_markup_exe() -> Vec<Attribute> { parse_attributes("font_style=\"italic\" font_size=\"smaller\"").unwrap() }
fn default_exclusive() -> bool { true }

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Left,
    Right
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_side")]
    pub side: Side,
    #[serde(default = "default_markup_default", deserialize_with = "deserialize_markup")]
    pub markup_default: Vec<Attribute>,
    #[serde(default = "default_markup_highlight", deserialize_with = "deserialize_markup")]
    pub markup_highlight: Vec<Attribute>,
    #[serde(default = "default_markup_exe", deserialize_with = "deserialize_markup")]
    pub markup_exe: Vec<Attribute>,
    #[serde(default = "default_exclusive")]
    pub exclusive: bool
}

fn deserialize_markup<'de, D>(deserializer: D) -> Result<Vec<Attribute>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    parse_attributes(s).map_err(D::Error::custom)
}

impl Config {
    pub fn load() -> Config {
        let config_str = match get_config_file(CONFIG_FILE) {
            Some(file) => std::fs::read_to_string(file).expect("Cannot read config"),
            _ => "".to_owned()
        };
        let config: Config = toml::from_str(&config_str).expect("Cannot parse config: {}");
        config
    }
}

fn parse_attributes(markup: &str) -> Result<Vec<Attribute>, String> {
    let (attributes, _, _) = pango::parse_markup(&format!("<span {}>X</span>", markup), '\0')
        .map_err(|err| format!("Failed to parse markup: {}", err))?;
    let mut iter = attributes.get_iterator().ok_or_else(||"Failed to parse markup")?;
    Ok(iter.get_attrs())
}