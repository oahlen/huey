# Huey

Neovim lua color scheme generator written in Rust.

Huey makes it easy to create a minimal lua based neovim colorscheme with
[HSL](https://en.wikipedia.org/wiki/HSL_and_HSV) based color generation.

The project takes inspiration from [colorgen-nvim](https://github.com/LunarVim/colorgen-nvim) and borrows some of the
data format semantics in the theme file but also enhanches it with HSL based functions.

## File format

Theme files are written in the toml format and contains the following sections:

* [hues] - dictionary of base hues to use in your colorscheme, can be referenced as variables in later sections
* [colors] - dictionary of colors to use in your colorscheme, can be created and manipulated with HSL based functions
* [highlights] - dictionary of the final nvim highlights, references the colors in the previous sections

## Functions

The following functions are avaiable in the `colors` section:

### hsl(hue, saturation, lightness)

Takes a hue value (0-360), saturation value (0-1.0) and a ligtness value (0-1.0) and creates a color variable.
The hue value can use a variable from the `[hues]` section with the `$` symbol.

### adjust(existing-color, saturation, lightness)

Takes an already defined color and adjusts its saturation and lightness values with a delta. Keep in mind that the delta
is not relative but absolute. A color with lightness 0.8 and a delta of -0.4 would result in a new color with lightness
0.4. Resulting saturation and lightness values are clamped between 0 and 1.

### darken(existing-color, value)

Shorthand method for decreasing the lightness component of the color. Behaves as the adjust method without a saturation
delta.

### lighten(existing-color, value)

Shorthand method for increasing the lightness component of the color. Behaves as the adjust method without a saturation
delta.

### mix(color1, color2, weight)

Mixes two existing colors into a new one based on weight. The weight is relative to the first color: 1.0 just returns the
first color, 0.5 return a 50/50 mix of the two colors, 0.0 just returns the second color.

### Regular hex color

It is also possible to declare colors with regular hex notation. The colors behave exactly the same as colors declared
with HSL.

## Highlights section

The format is very similar to colorgen-nvim:

* `foreground background style special`
* `link:<name of hl group>`

The '-' is used to skip a particular section and replace it with NONE

The style otions are the following:

* o: standout
* u: underline
* c: undercurl
* d: underdouble
* t: underdotted
* h: underdashed
* s: strikethrough
* i: italic
* b: bold
* r: reverse
* n: nocombine

## Example theme file

```toml
name = "iceberg"
background = "dark"

[hues]
hue_base = 230

hue_red    = 0
hue_orange = 25
hue_green  = 70
hue_cyan   = 190
hue_blue   = 215
hue_purple = 255
hue_pale   = 225

[colors]
# palette
blue   = "hsl($hue_blue, 0.37, 0.65)"
green  = "hsl($hue_green, 0.32, 0.63)"
cyan   = "hsl($hue_cyan, 0.32, 0.65)"
orange = "hsl($hue_orange, 0.65, 0.68)"
purple = "hsl($hue_purple, 0.32, 0.68)"
red    = "hsl($hue_red, 0.65, 0.68)"
pale   = "hsl($hue_pale, 0.28, 0.72)"

hex_color = "#ff0000"

# normal
normal_bg = "hsl($hue_base, 0.20, 0.11)"
normal_fg = "hsl($hue_base, 0.10, 0.80)"

# linenr
linenr_bg       = "adjust(normal_bg, 0.05, 0.05)"
linenr_fg       = "lighten(linenr_bg, 0.20)"
cursorlinenr_bg = "adjust(linenr_bg, 0.10, 0.10)"
cursorlinenr_fg = "adjust(linenr_fg, 0.10, 0.50)"

[highlights]
Normal           = "normal_fg normal_bg"
ColorColumn      = "- cursorline_bg"
CursorColumn     = "- cursorline_bg"
CursorLine       = "- cursorline_bg"
Constant         = "purple - i" # italic
Cursor           = "normal_bg normal_fg"
CursorLineNr     = "cursorlinenr_fg cursorlinenr_bg"
Delimiter        = "normal_fg"
Directory        = "blue - b" # bold

# treesitter
"@attribute"             = "link:Special"
"@boolean"               = "link:Constant"
"@character"             = "link:Constant"
"@character.special"     = "link:Constant"
"@comment"               = "link:Comment"
"@conditional"           = "link:Statement"
"@constant.builtin"      = "link:Constant"
"@constant"              = "link:Constant"
"@constant.macro"        = "link:Constant"
```
