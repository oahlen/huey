#[derive(thiserror::Error, Debug, PartialEq)]
pub enum HslColorError {
    #[error("Invalid hue value (expected 0-360, got {found:?})")]
    Hue { found: f32 },
    #[error("Invalid saturation value (expected 0-1, got {found:?})")]
    Saturation { found: f32 },
    #[error("Invalid lightness value (expected 0-1, got {found:?})")]
    Lightness { found: f32 },
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum RgbColorError {
    #[error("Invalid hex format {found:?}")]
    Format { found: String },
    #[error("Invalid mix value (expected 0-1) got {found:?}")]
    Mix { found: f32 },
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ThemeError {
    #[error("Invalid background {background:?}")]
    InvalidBackground { background: String },
    #[error("Missing toml value")]
    MissingValue,
    #[error("Invalid color format {color:?}")]
    InvalidColor { color: String },
    #[error("Referenced color {color:?} is not present in palette")]
    MissingColor { color: String },
    #[error("Referenced hue {hue:?} is not present in palette")]
    MissingHue { hue: String },
    #[error("Can't lookup hue {hue:?} due to toml section missing")]
    MissingHueSection { hue: String },
    #[error("Invalid highlight {highlight:?}")]
    InvalidHighlight { highlight: String },
    #[error("Unknown style option {option:?}")]
    UnknownStyleOption { option: String },
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum FileError {
    #[error("File {path:?} not found")]
    FileNotFound { path: String },
}
