use std::collections::HashMap;

use crate::{color::HslColor, error::ThemeError, format::lookup_rgb_color};

pub(crate) fn parse_highlight(
    hl_group: &str,
    value: &str,
    palette: &HashMap<String, HslColor>,
) -> Result<String, ThemeError> {
    let values = value
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    match values[..] {
        [fg] => match fg.contains("link:") {
            true => Ok(format!(
                "\n    hl(0, \"{hl_group}\", {{ link = \"{link}\" }})",
                link = fg.to_string().replace("link:", ""),
            )),
            false => Ok(format!(
                "\n    hl(0, \"{hl_group}\", {{ fg = \"{fg}\", bg = \"NONE\" }})",
                fg = lookup_highlight(fg, palette)?,
            )),
        },
        [fg, bg] => Ok(format!(
            "\n    hl(0, \"{hl_group}\", {{ fg = \"{fg}\", bg = \"{bg}\" }})",
            fg = lookup_highlight(fg, palette)?,
            bg = lookup_highlight(bg, palette)?,
        )),
        [fg, bg, style] => Ok(format!(
            "\n    hl(0, \"{hl_group}\", {{ fg = \"{fg}\", bg = \"{bg}\", {style_options} }})",
            fg = lookup_highlight(fg, palette)?,
            bg = lookup_highlight(bg, palette)?,
            style_options = parse_style_options(style)?,
        )),
        [fg, bg, style, sp] => Ok(format!(
            "\n    hl(0, \"{hl_group}\", {{ fg = \"{fg}\", bg = \"{bg}\", sp = \"{sp}\", {style_options} }})",
            fg = lookup_highlight(fg, palette)?,
            bg = lookup_highlight(bg, palette)?,
            sp = lookup_highlight(sp, palette)?,
            style_options = parse_style_options(style)?,
        )),
        _ => Err(ThemeError::InvalidHighlight {
            highlight: value.to_string(),
        }),
    }
}

fn lookup_highlight(
    value: &str,
    palette: &HashMap<String, HslColor>,
) -> Result<String, ThemeError> {
    match value {
        "-" => Ok("NONE".to_string()),
        _ => Ok(lookup_rgb_color(value, palette)?.to_string()),
    }
}

fn parse_style_options(style: &str) -> Result<String, ThemeError> {
    let mut style_options = String::new();

    for option in style.chars() {
        match option {
            'b' => style_options += "bold = true, ",
            'i' => style_options += "italic = true, ",
            'u' => style_options += "underline = true, ",
            'c' => style_options += "undercurl = true, ",
            'd' => style_options += "underdouble = true, ",
            't' => style_options += "underdotted = true, ",
            'h' => style_options += "underdashed = true, ",
            'o' => style_options += "standout = true, ",
            's' => style_options += "strikethrough = true, ",
            'n' => style_options += "nocombine = true, ",
            'r' => style_options += "reverse = true, ",
            '-' => {}
            unknown => {
                return Err(ThemeError::UnknownStyleOption {
                    option: unknown.to_string(),
                })
            }
        }
    }

    style_options.pop();
    Ok(style_options)
}
