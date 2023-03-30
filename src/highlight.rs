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
    let mut style_options: Vec<String> = Vec::new();

    for option in style.chars() {
        match option {
            'b' => style_options.push(String::from("bold = true")),
            'i' => style_options.push(String::from("italic = true")),
            'u' => style_options.push(String::from("underline = true")),
            'c' => style_options.push(String::from("undercurl = true")),
            'd' => style_options.push(String::from("underdouble = true")),
            't' => style_options.push(String::from("underdotted = true")),
            'h' => style_options.push(String::from("underdashed = true")),
            'o' => style_options.push(String::from("standout = true")),
            's' => style_options.push(String::from("strikethrough = true")),
            'n' => style_options.push(String::from("nocombine = true")),
            'r' => style_options.push(String::from("reverse = true")),
            '-' => {}
            unknown => {
                return Err(ThemeError::UnknownStyleOption {
                    option: unknown.to_string(),
                })
            }
        }
    }

    Ok(style_options.join(", "))
}
