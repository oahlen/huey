use regex::Regex;
use std::fmt;

use crate::error::{HslColorError, RgbColorError};

pub trait Color: fmt::Display {
    fn adjust(&self, saturation: f32, lightness: f32) -> Box<dyn Color>;
    fn lighten(&self, amount: f32) -> Box<dyn Color>;
    fn darken(&self, amount: f32) -> Box<dyn Color>;
    fn hex(&self) -> String;
    fn copy(&self) -> Box<dyn Color>;
    fn to_rgb(&self) -> RgbColor;
}

pub(crate) fn mix(
    color1: &dyn Color,
    color2: &dyn Color,
    weight: f32,
) -> Result<RgbColor, RgbColorError> {
    if !(0.0..=1.0).contains(&weight) {
        return Err(RgbColorError::Mix { found: weight });
    }

    let c1 = color1.to_rgb();
    let c2 = color2.to_rgb();

    let w1 = weight;
    let w2 = 1.0 - weight;

    Ok(RgbColor::new(
        (c1.r as f32 * w1 + c2.r as f32 * w2) as u8,
        (c1.g as f32 * w1 + c2.g as f32 * w2) as u8,
        (c1.b as f32 * w1 + c2.b as f32 * w2) as u8,
    ))
}

#[derive(Debug, Copy, Clone)]
pub struct HslColor {
    hue: f32,
    saturation: f32,
    lightness: f32,
}

impl Color for HslColor {
    fn adjust(&self, saturation: f32, lightness: f32) -> Box<dyn Color> {
        Box::new(self.adjust(saturation, lightness))
    }

    fn lighten(&self, amount: f32) -> Box<dyn Color> {
        Box::new(self.lighten(amount))
    }

    fn darken(&self, amount: f32) -> Box<dyn Color> {
        Box::new(self.darken(amount))
    }

    fn hex(&self) -> String {
        self.to_rgb_color().to_string()
    }

    fn copy(&self) -> Box<dyn Color> {
        Box::new(*self)
    }

    fn to_rgb(&self) -> RgbColor {
        self.to_rgb_color()
    }
}

impl HslColor {
    pub(crate) fn new(
        hue: f32,
        saturation: f32,
        lightness: f32,
    ) -> Result<HslColor, HslColorError> {
        if !(0.0..=360.0).contains(&hue) {
            Err(HslColorError::Hue { found: hue })?
        }

        if !(0.0..=1.0).contains(&saturation) {
            Err(HslColorError::Saturation { found: saturation })?
        }

        if !(0.0..=1.0).contains(&lightness) {
            Err(HslColorError::Lightness { found: lightness })?
        }

        Ok(HslColor {
            hue: hue / 360.0,
            saturation,
            lightness,
        })
    }

    pub(crate) fn adjust(&self, saturation: f32, lightness: f32) -> HslColor {
        HslColor {
            hue: self.hue,
            saturation: (self.saturation + saturation).clamp(0.0, 1.0),
            lightness: (self.lightness + lightness).clamp(0.0, 1.0),
        }
    }

    pub(crate) fn lighten(&self, amount: f32) -> HslColor {
        HslColor {
            hue: self.hue,
            saturation: self.saturation,
            lightness: (self.lightness + amount).clamp(0.0, 1.0),
        }
    }

    pub(crate) fn darken(&self, amount: f32) -> HslColor {
        HslColor {
            hue: self.hue,
            saturation: self.saturation,
            lightness: (self.lightness - amount).clamp(0.0, 1.0),
        }
    }

    fn to_rgb_color(self) -> RgbColor {
        let HslColor {
            hue: h,
            saturation: s,
            lightness: l,
        } = self;

        // Achromatic color
        if s == 0.0 {
            return RgbColor {
                r: (l * 255.0).round() as u8,
                g: (l * 255.0).round() as u8,
                b: (l * 255.0).round() as u8,
            };
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };

        let p = 2.0 * l - q;

        fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
            if t < 0.0 {
                t += 1.0;
            }

            if t > 1.0 {
                t -= 1.0;
            }

            if t < 1.0 / 6.0 {
                return p + (q - p) * 6.0 * t;
            }

            if t < 1.0 / 2.0 {
                return q;
            }

            if t < 2.0 / 3.0 {
                return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
            }

            p
        }

        let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

        RgbColor::new(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        )
    }
}

impl From<RgbColor> for HslColor {
    fn from(color: RgbColor) -> Self {
        color.to_hsl_color()
    }
}

impl fmt::Display for HslColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Hue: {}, Saturation: {}, Lightness: {}",
            self.hue * 360.0,
            self.saturation,
            self.lightness
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

impl Color for RgbColor {
    fn adjust(&self, saturation: f32, lightness: f32) -> Box<dyn Color> {
        Box::new(self.to_hsl_color().adjust(saturation, lightness))
    }

    fn lighten(&self, amount: f32) -> Box<dyn Color> {
        Box::new(self.to_hsl_color().lighten(amount))
    }

    fn darken(&self, amount: f32) -> Box<dyn Color> {
        Box::new(self.to_hsl_color().darken(amount))
    }

    fn hex(&self) -> String {
        self.to_string()
    }

    fn copy(&self) -> Box<dyn Color> {
        Box::new(*self)
    }

    fn to_rgb(&self) -> RgbColor {
        *self
    }
}

impl RgbColor {
    pub(crate) fn new(r: u8, g: u8, b: u8) -> RgbColor {
        RgbColor { r, g, b }
    }

    pub(crate) fn parse_from_hex(hex: &str) -> Result<RgbColor, anyhow::Error> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^#([a-fA-F\d]{6})$").expect("Hex format regex is invalid");
        }

        fn extract(slice: &str) -> Result<u8, anyhow::Error> {
            Ok(i64::from_str_radix(slice, 16)? as u8)
        }

        match RE.captures(hex) {
            Some(capture) => Ok(RgbColor::new(
                extract(&capture[1][0..2])?,
                extract(&capture[1][2..4])?,
                extract(&capture[1][4..6])?,
            )),
            None => Err(RgbColorError::Format {
                found: hex.to_string(),
            }
            .into()),
        }
    }

    fn to_hsl_color(self) -> HslColor {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let mut h = (max + min) / 2.0;
        let l = h;

        // Achromatic color
        if max == min {
            return HslColor {
                hue: 0.0,
                saturation: 0.0,
                lightness: l,
            };
        }

        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };

        if r == max {
            h = (g - b) / d + (if g < b { 6.0 } else { 0.0 });
        } else if g == max {
            h = (b - r) / d + 2.0;
        } else if b == max {
            h = (r - g) / d + 4.0
        }

        HslColor {
            hue: h / 6.0,
            saturation: s,
            lightness: l,
        }
    }
}

impl From<HslColor> for RgbColor {
    fn from(color: HslColor) -> Self {
        color.to_rgb_color()
    }
}

impl fmt::Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsl_color_new() {
        let hsl = HslColor::new(120.0, 0.5, 0.5).unwrap();
        assert_eq!(hsl.hue, 120.0 / 360.0);
        assert_eq!(hsl.saturation, 0.5);
        assert_eq!(hsl.lightness, 0.5);
    }

    #[test]
    fn test_hsl_color_adjust() {
        let hsl = HslColor::new(120.0, 0.5, 0.5).unwrap();
        let adjusted = hsl.adjust(0.2, 0.1);
        assert_eq!(adjusted.saturation, 0.7);
        assert_eq!(adjusted.lightness, 0.6);
    }

    #[test]
    fn test_hsl_color_adjust_clamped() {
        let hsl = HslColor::new(120.0, 0.5, 0.5).unwrap();
        let adjusted = hsl.adjust(0.6, -0.6);
        assert_eq!(adjusted.saturation, 1.0);
        assert_eq!(adjusted.lightness, 0.0);
    }

    #[test]
    fn test_hsl_color_lighten() {
        let hsl = HslColor::new(120.0, 0.5, 0.5).unwrap();
        let lightened = hsl.lighten(0.2);
        assert_eq!(lightened.lightness, 0.7);
    }

    #[test]
    fn test_hsl_color_darken() {
        let hsl = HslColor::new(120.0, 0.5, 0.5).unwrap();
        let darkened = hsl.darken(0.2);
        assert_eq!(darkened.lightness, 0.3);
    }

    #[test]
    fn test_hsl_to_rgb() {
        assert_eq!(
            "#161822",
            HslColor::new(230.0, 0.2, 0.11).unwrap().to_rgb().hex()
        );
        assert_eq!(
            "#ff0000",
            HslColor::new(0.0, 1.0, 0.50).unwrap().to_rgb().hex()
        );
        assert_eq!(
            "#ff0000",
            HslColor::new(360.0, 1.0, 0.50).unwrap().to_rgb().hex()
        );
        assert_eq!(
            "#ffffff",
            HslColor::new(360.0, 1.0, 1.00).unwrap().to_rgb().hex()
        );
    }

    #[test]
    fn test_rgb_to_hsl() {
        let rgb = RgbColor::parse_from_hex("#40bf40").unwrap();
        let hsl = HslColor::from(rgb);

        assert_eq!(rgb.hex(), hsl.hex());
        assert!((hsl.hue - 120.0 / 360.0).abs() < 0.01);
        assert!((hsl.saturation - 0.5).abs() < 0.01);
        assert!((hsl.lightness - 0.5).abs() < 0.01);
    }
}
