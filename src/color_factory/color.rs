use crate::color_factory::{Cmyk, ColorConverter, Hsl, Lab, Rgb};

#[derive(Debug)]
#[allow(unused)]
pub struct Color {
    pub hex: String,
    pub rgb: Rgb,
    pub hsl: Hsl,
    pub lab: Lab,
    pub cmyk: Cmyk,
    pub luminance_wcag: f32,
}

#[allow(unused)]
impl Color {
    pub fn from_hex(hex_string: &str) -> Self {
        let rgb = ColorConverter::hex_to_rgb(hex_string);
        let hsl = ColorConverter::rgb_to_hsl(&rgb);
        let hex = hex_string.into();
        let lab = ColorConverter::rgb_to_lab(&rgb);
        let luminance_wcag = ColorConverter::rgb_to_luminance_wcag(&rgb);
        let cmyk = ColorConverter::rgb_to_cmyk(&rgb);

        Self {
            hex,
            rgb,
            hsl,
            lab,
            cmyk,
            luminance_wcag,
        }
    }
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        let rgb = Rgb { r, g, b };
        let hsl = ColorConverter::rgb_to_hsl(&rgb);
        let hex = ColorConverter::rgb_to_hex(&rgb);
        let lab = ColorConverter::rgb_to_lab(&rgb);
        let luminance_wcag = ColorConverter::rgb_to_luminance_wcag(&rgb);
        let cmyk = ColorConverter::rgb_to_cmyk(&rgb);

        Self {
            hex,
            rgb,
            hsl,
            lab,
            cmyk,
            luminance_wcag,
        }
    }
}
