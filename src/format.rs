use std::{collections::HashMap, fmt::Display, fs, path::Path};

use indexmap::IndexMap;
use regex::Regex;
use serde::Deserialize;
use toml::Table;

use crate::{
    color::{mix, Color, HslColor, RgbColor},
    error::{FileError, ThemeError},
    highlight::parse_highlight,
};

pub(crate) fn parse_theme(path: &str) -> Result<Theme, anyhow::Error> {
    if !Path::new(path).exists() {
        return Err(FileError::FileNotFound {
            path: path.to_string(),
        }
        .into());
    }

    Theme::new(toml::from_str(&fs::read_to_string(path)?)?)
}

#[derive(Debug, Deserialize)]
pub(crate) struct ParsedTheme {
    pub name: String,
    pub background: String,
    pub hues: Option<HashMap<String, f32>>,
    pub colors: Table,
    pub highlights: Table,
    pub globals: Table,
}

pub(crate) fn lookup_color<'a>(
    key: &str,
    palette: &'a IndexMap<String, Box<dyn Color>>,
) -> Result<&'a dyn Color, ThemeError> {
    match palette.contains_key(key) {
        true => Ok(palette[key].as_ref()),
        false => Err(ThemeError::MissingColor {
            color: key.to_string(),
        }),
    }
}

pub(crate) struct Theme {
    pub name: String,
    pub background: Background,
    pub palette: IndexMap<String, Box<dyn Color>>,
    pub highlights: Vec<String>,
    pub globals: Vec<String>,
}

impl Theme {
    fn new(parsed: ParsedTheme) -> Result<Theme, anyhow::Error> {
        let palette = parse_palette(&parsed)?;

        let mut highlights: Vec<String> = Vec::new();
        for (key, value) in &parsed.highlights {
            match value.as_str() {
                Some(value) => {
                    highlights.push(parse_highlight(key, value, &palette)?);
                }
                None => return Err(ThemeError::MissingValue.into()),
            }
        }

        let mut globals: Vec<String> = Vec::new();

        for (key, value) in &parsed.globals {
            match value.as_str() {
                Some(value) => {
                    let color = if palette.contains_key(value) {
                        palette[value].hex()
                    } else {
                        parse_palette_entry(value, &palette, &parsed.hues)?.hex()
                    };

                    globals.push(format!("    vim.g.{key} = \"{color}\"\n"));
                }
                None => return Err(ThemeError::MissingValue.into()),
            }
        }

        Ok(Theme {
            name: parsed.name,
            background: Background::new(&parsed.background)?,
            palette,
            highlights,
            globals,
        })
    }
}

fn parse_palette(input: &ParsedTheme) -> Result<IndexMap<String, Box<dyn Color>>, anyhow::Error> {
    let mut palette: IndexMap<String, Box<dyn Color>> = IndexMap::new();

    for (key, value) in &input.colors {
        match value.as_str() {
            Some(value) => {
                if palette.contains_key(value) {
                    palette.insert(key.to_string(), palette[value].copy());
                } else {
                    palette.insert(
                        key.to_string(),
                        parse_palette_entry(value, &palette, &input.hues)?,
                    );
                }
            }
            None => return Err(ThemeError::MissingValue.into()),
        }
    }

    Ok(palette)
}

fn parse_palette_entry(
    value: &str,
    palette: &IndexMap<String, Box<dyn Color>>,
    hues: &Option<HashMap<String, f32>>,
) -> Result<Box<dyn Color>, anyhow::Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?i)(hsl|adjust|lighten|darken|mix)\((.*)\)$")
            .expect("Color format regex is invalid");
    }

    if value.starts_with('#') {
        return Ok(Box::new(RgbColor::parse_from_hex(value)?));
    }

    match RE.captures(value) {
        Some(capture) => match capture[1].to_lowercase().as_str() {
            "hsl" => Ok(Box::new(parse_hsl_color(
                split_input(&capture[2], 3)?,
                hues,
            )?)),
            "adjust" => adjust_color(split_input(&capture[2], 3)?, palette),
            "lighten" => lighten_color(split_input(&capture[2], 2)?, palette),
            "darken" => darken_color(split_input(&capture[2], 2)?, palette),
            "mix" => mix_colors(split_input(&capture[2], 3)?, palette),
            _ => panic!("Unhandled color capture group option"),
        },
        None => Err(ThemeError::InvalidColor {
            color: value.to_string(),
        }
        .into()),
    }
}

fn split_input(capture: &str, expected_parts: usize) -> Result<Vec<&str>, ThemeError> {
    let parts: Vec<&str> = capture.split(',').map(|x| x.trim()).collect();

    if parts.len() != expected_parts {
        return Err(ThemeError::InvalidColor {
            color: capture.to_string(),
        });
    }

    Ok(parts)
}

fn parse_hsl_color(
    parts: Vec<&str>,
    hues: &Option<HashMap<String, f32>>,
) -> Result<HslColor, anyhow::Error> {
    if parts[0].starts_with('$') {
        let key = &parts[0][1..];

        return match hues {
            Some(hues) => match hues.contains_key(key) {
                true => Ok(HslColor::new(
                    hues[key],
                    parts[1].parse::<f32>()?,
                    parts[2].parse::<f32>()?,
                )?),
                false => Err(ThemeError::MissingHue {
                    hue: key.to_string(),
                }
                .into()),
            },
            None => Err(ThemeError::MissingHueSection {
                hue: key.to_string(),
            }
            .into()),
        };
    }

    Ok(HslColor::new(
        parts[0].parse::<f32>()?,
        parts[1].parse::<f32>()?,
        parts[2].parse::<f32>()?,
    )?)
}

fn adjust_color(
    parts: Vec<&str>,
    palette: &IndexMap<String, Box<dyn Color>>,
) -> Result<Box<dyn Color>, anyhow::Error> {
    Ok(lookup_color(parts[0], palette)?.adjust(parts[1].parse::<f32>()?, parts[2].parse::<f32>()?))
}

fn lighten_color(
    parts: Vec<&str>,
    palette: &IndexMap<String, Box<dyn Color>>,
) -> Result<Box<dyn Color>, anyhow::Error> {
    Ok(lookup_color(parts[0], palette)?.lighten(parts[1].parse::<f32>()?))
}

fn darken_color(
    parts: Vec<&str>,
    palette: &IndexMap<String, Box<dyn Color>>,
) -> Result<Box<dyn Color>, anyhow::Error> {
    Ok(lookup_color(parts[0], palette)?.darken(parts[1].parse::<f32>()?))
}

fn mix_colors(
    parts: Vec<&str>,
    palette: &IndexMap<String, Box<dyn Color>>,
) -> Result<Box<dyn Color>, anyhow::Error> {
    Ok(Box::new(mix(
        lookup_color(parts[0], palette)?,
        lookup_color(parts[1], palette)?,
        parts[2].parse::<f32>()?,
    )?) as Box<dyn Color>)
}

#[derive(Debug)]
pub enum Background {
    Dark,
    Light,
}

impl Background {
    fn new(input: &str) -> Result<Background, anyhow::Error> {
        if input.eq_ignore_ascii_case("dark") {
            return Ok(Background::Dark);
        } else if input.eq_ignore_ascii_case("light") {
            return Ok(Background::Light);
        }

        Err(ThemeError::InvalidBackground {
            background: input.to_string(),
        })?
    }
}

impl Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Background::Dark => write!(f, "dark"),
            Background::Light => write!(f, "light"),
        }
    }
}
