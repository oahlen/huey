use indexmap::IndexMap;

use crate::{color::Color, error::ThemeError, format::lookup_color};

pub(crate) fn parse_highlight(
    hl_group: &str,
    value: &str,
    palette: &IndexMap<String, Box<dyn Color>>,
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
    palette: &IndexMap<String, Box<dyn Color>>,
) -> Result<String, ThemeError> {
    match value {
        "-" => Ok("NONE".to_string()),
        _ => Ok(lookup_color(value, palette)?.hex()),
    }
}

fn parse_style_options(style: &str) -> Result<String, ThemeError> {
    let mut style_options: Vec<&str> = Vec::new();

    for option in style.chars() {
        match option {
            'b' => style_options.push("bold = true"),
            'i' => style_options.push("italic = true"),
            'u' => style_options.push("underline = true"),
            'c' => style_options.push("undercurl = true"),
            'd' => style_options.push("underdouble = true"),
            't' => style_options.push("underdotted = true"),
            'h' => style_options.push("underdashed = true"),
            'o' => style_options.push("standout = true"),
            's' => style_options.push("strikethrough = true"),
            'n' => style_options.push("nocombine = true"),
            'r' => style_options.push("reverse = true"),
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
