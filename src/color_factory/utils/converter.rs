macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

/*
* https://www.niwa.nu/2013/05/math-behind-colorspace-conversions-rgb-hsl/
*/

#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub struct Hsl {
    pub h: u16,
    pub s: u8,
    pub l: f32,
}

#[derive(Debug)]
pub struct Lab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

#[derive(Debug)]
pub struct Cmyk {
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
}

#[allow(unused)]
pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Rgb {
    fn to_string(&self) -> String {
        let Rgb { r, g, b } = self;
        format!("rgb({},{},{})", r, g, b)
    }
}

impl ToString for Hsl {
    fn to_string(&self) -> String {
        let Hsl { h, s, l } = self;
        format!("hsl({}°,{}%,{}%)", h, s, l)
    }
}

impl ToString for Lab {
    fn to_string(&self) -> String {
        let Lab { l, a, b } = self;
        format!("lab({},{},{})", l, a, b)
    }
}

impl ToString for Cmyk {
    fn to_string(&self) -> String {
        let Cmyk { c, m, y, k } = self;
        format!("cmyk({}%,{}%,{}%,{}%)", c, m, y, k)
    }
}

fn split_hex_string(hex_string: &str) -> (&str, &str, &str) {
    let h_r = hex_string.get(1..=2).unwrap();
    let h_g = hex_string.get(3..=4).unwrap();
    let h_b = hex_string.get(5..=6).unwrap();
    (h_r, h_g, h_b)
}

#[inline]
fn hex_to_decimal(hex_string: &str) -> u8 {
    i64::from_str_radix(hex_string, 16).unwrap() as u8
}

#[inline]
fn decimal_to_hex(d: u8) -> String {
    if d == 0 {
        "00".into()
    } else {
        format!("{:x}", d)
    }
}
#[inline]
fn normalized_value_to_linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

fn luminance_wcag(rgb: &Rgb) -> f32 {
    let Rgb { r, g, b } = rgb;
    let linear_r = normalized_value_to_linear(*r as f32 / 255.0);
    let linear_g = normalized_value_to_linear(*g as f32 / 255.0);
    let linear_b = normalized_value_to_linear(*b as f32 / 255.0);

    0.2126 * linear_r + 0.7152 * linear_g + 0.0722 * linear_b
}

fn nrgb_to_xyz(n_r: f32, n_g: f32, n_b: f32) -> (f32, f32, f32) {
    let r = normalized_value_to_linear(n_r);
    let g = normalized_value_to_linear(n_g);
    let b = normalized_value_to_linear(n_b);

    let x = r * 0.4124 + g * 0.3576 + b * 0.1805;
    let y = r * 0.2126 + g * 0.7152 + b * 0.0722;
    let z = r * 0.0193 + g * 0.1192 + b * 0.9505;

    (x, y, z)
}

#[inline]
fn f(t: f32) -> f32 {
    if t > 0.008856 {
        t.cbrt()
    } else {
        7.787 * t + 16.0 / 116.0
    }
}
fn xyz_to_lab(x: f32, y: f32, z: f32) -> Lab {
    // D65 reference white
    let xn = 0.95047;
    let yn = 1.00000;
    let zn = 1.08883;

    let fx = f(x / xn);
    let fy = f(y / yn);
    let fz = f(z / zn);

    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);

    Lab { l, a, b }
}

fn normalize_rgb(rgb: &Rgb) -> (f32, f32, f32) {
    let Rgb { r, g, b } = rgb;
    (
        (*r as f32) / 255.0,
        (*g as f32) / 255.0,
        (*b as f32) / 255.0,
    )
}

pub struct ColorConverter;

impl ColorConverter {
    pub fn rgb_to_hex(rgb: &Rgb) -> String {
        let h_r = decimal_to_hex(rgb.r);
        let h_g = decimal_to_hex(rgb.g);
        let h_b = decimal_to_hex(rgb.b);

        format!("#{}{}{}", h_r, h_g, h_b)
    }

    pub fn rgb_to_cmyk(rgb: &Rgb) -> Cmyk {
        let (r, g, b) = normalize_rgb(rgb);
        let k = 1.0 - max!(r, g, b);
        if k == 0.0 {
            return Cmyk {
                c: 0.0,
                m: 0.0,
                y: 0.0,
                k: 1.0,
            };
        }
        let c = ((1.0 - r - k) / (1.0 - k)) * 100.0;
        let m = ((1.0 - g - k) / (1.0 - k)) * 100.0;
        let y = ((1.0 - b - k) / (1.0 - k)) * 100.0;

        Cmyk {
            c,
            m,
            y,
            k: k * 100.0,
        }
    }

    pub fn hex_to_rgb(hex_string: &str) -> Rgb {
        if hex_string.len() < 7 {
            panic!()
        }
        let (h_r, h_g, h_b) = split_hex_string(hex_string);

        let r = hex_to_decimal(h_r);
        let g = hex_to_decimal(h_g);
        let b = hex_to_decimal(h_b);

        Rgb { r, g, b }
    }

    pub fn rgb_to_hsl(rgb: &Rgb) -> Hsl {
        let (n_r, n_g, n_b) = normalize_rgb(rgb);

        let min = min!(n_r, n_g, n_b);
        let max = max!(n_r, n_g, n_b);

        let l = (min + max) / 2.0;

        let s = if min == max {
            0.0
        } else if l <= 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - max - min)
        };

        /*
        If Red is max, then Hue = (G-B)/(max-min)
        If Green is max, then Hue = 2.0 + (B-R)/(max-min)
        If Blue is max, then Hue = 4.0 + (R-G)/(max-min)
        */
        let mut h = match max {
            v if v == n_r => (n_g - n_b) / (max - min),
            v if v == n_g => 4.0 + (n_b - n_r) / (max - min),
            _ => 4.0 + (n_r - n_g) / (max - min),
        };

        h *= 60.0;

        if h < 0.0 {
            h += 360.0;
        }

        Hsl {
            h: (h as u16) % 360,
            s: (s * 100.0) as u8,
            l: l * 100.0,
        }
    }
    pub fn rgb_to_lab(rgb: &Rgb) -> Lab {
        let (n_r, n_g, n_b) = normalize_rgb(rgb);
        let (x, y, z) = nrgb_to_xyz(n_r, n_g, n_b);
        xyz_to_lab(x, y, z)
    }

    pub fn rgb_to_luminance_wcag(rgb: &Rgb) -> f32 {
        luminance_wcag(rgb)
    }
}
