pub struct HWB{
    pub hue: f64,
    pub whiteness: f64,
    pub blackness: f64,
}
pub struct HSL{
    pub hue: f64,
    pub saturation: f64,
    pub lightness: f64,
}
pub struct CMYK{
    pub cyan: f64,
    pub magenta: f64,
    pub yellow: f64,
    pub black: f64,
}
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub trait Color {
    fn to_rgb(&self) -> RGB;
    fn to_hsl(&self) -> HSL;
    fn to_hwb(&self) -> HWB;
    fn to_cmyk(&self) -> CMYK;
}

impl Color for RGB {
    fn to_rgb(&self) -> RGB {
        RGB {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
    fn to_hsl(&self) -> HSL {
        let r = self.red as f64 / 255.0;
        let g = self.green as f64 / 255.0;
        let b = self.blue as f64 / 255.0;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let mut h = (max + min) / 2.0;
        let mut s = (max + min) / 2.0;
        let l = (max + min) / 2.0;
        if max == min {
            h = 0.0;
            s = 0.0;
        } else {
            let d = max - min;
            s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
            h = if max == r {
                (g - b) / d + if g < b { 6.0 } else { 0.0 }
            } else if max == g {
                (b - r) / d + 2.0
            } else if max == b {
                (r - g) / d + 4.0
            } else {
                0.0
            };
            h /= 6.0;
        }
        HSL {
            hue: h * 360.0,
            saturation: s * 100.0,
            lightness: l * 100.0,
        }
    }
    fn to_hwb(&self) -> HWB {
        let r = self.red as f64 / 255.0;
        let g = self.green as f64 / 255.0;
        let b = self.blue as f64 / 255.0;
        let mut w = r.min(g).min(b);
        let mut b = 1.0 - r.max(g).max(b);
        let mut h = 0.0;
        if r == g && g == b {
            h = 0.0;
        } else {
            let mut h = r;
            let mut d: f64 = g - b;
            let mut e: f64 = 6.0;
            if r == w {
                h = d / e;
            } else if g == w {
                h = 2.0 + d / e;
            } else if b == w {
                h = 4.0 + d / e;
            }
            if w == 0.0 {
                b = 0.0;
            } else if w == 1.0 {
                w = 0.0;
            }
        }
        HWB {
            hue: h * 360.0,
            whiteness: w * 100.0,
            blackness: b * 100.0,
        }
    }
    fn to_cmyk(&self) -> CMYK {
        let r = self.red as f64 / 255.0;
        let g = self.green as f64 / 255.0;
        let b = self.blue as f64 / 255.0;
        let k = 1.0 - r.max(g).max(b);
        let c = (1.0 - r - k) / (1.0 - k);
        let m = (1.0 - g - k) / (1.0 - k);
        let y = (1.0 - b - k) / (1.0 - k);
        CMYK {
            cyan: c * 100.0,
            magenta: m * 100.0,
            yellow: y * 100.0,
            black: k * 100.0,
        }
    }
}
impl Color for CMYK {
    fn to_rgb(&self) -> RGB {
        let c = self.cyan / 100.0;
        let m = self.magenta / 100.0;
        let y = self.yellow / 100.0;
        let k = self.black / 100.0;
        let r = 255.0 * (1.0 - c) * (1.0 - k);
        let g = 255.0 * (1.0 - m) * (1.0 - k);
        let b = 255.0 * (1.0 - y) * (1.0 - k);
        RGB {
            red: r as u8,
            green: g as u8,
            blue: b as u8,
        }
    }
    fn to_hsl(&self) -> HSL {
        let rgb = self.to_rgb();
        rgb.to_hsl()
    }
    fn to_hwb(&self) -> HWB {
        let rgb: RGB = self.to_rgb();
        rgb.to_hwb()
    }
    fn to_cmyk(&self) -> CMYK {
        CMYK {
            cyan: self.cyan,
            magenta: self.magenta,
            yellow: self.yellow,
            black: self.black,
        }
    }
}
impl Color for HSL {
    fn to_rgb(&self) -> RGB {
        let h = self.hue / 360.0;
        let s = self.saturation / 100.0;
        fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
            let t = if t < 0.0 { t + 1.0 } else if t > 1.0 { t - 1.0 } else { t };
            if t < 1.0 / 6.0 {
                p + (q - p) * 6.0 * t
            } else if t < 1.0 / 2.0 {
                q
            } else if t < 2.0 / 3.0 {
                p + (q - p) * (2.0 / 3.0 - t) * 6.0
            } else {
                p
            }
        }

        let l = self.lightness / 100.0;
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        if s == 0.0 {
            r = l;
            g = l;
            b = l;
        } else {
            let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
            let p = 2.0 * l - q;
            r = hue_to_rgb(p, q, h + 1.0 / 3.0);
            g = hue_to_rgb(p, q, h);
            b = hue_to_rgb(p, q, h - 1.0 / 3.0);
        }
        RGB {
            red: (r * 255.0) as u8,
            green: (g * 255.0) as u8,
            blue: (b * 255.0) as u8,
        }
    }
    fn to_hsl(&self) -> HSL {
        HSL {
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness,
        }
    }
    fn to_hwb(&self) -> HWB {
        let rgb = self.to_rgb();
        rgb.to_hwb()
    }
    fn to_cmyk(&self) -> CMYK {
        let rgb = self.to_rgb();
        rgb.to_cmyk()
    }
}
impl Color for HWB {
    fn to_cmyk(&self) -> CMYK {
        let rgb = self.to_rgb();
        rgb.to_cmyk()
    }
    fn to_hsl(&self) -> HSL {
        let rgb = self.to_rgb();
        rgb.to_hsl()
    }
    fn to_hwb(&self) -> HWB {
        HWB {
            hue: self.hue,
            whiteness: self.whiteness,
            blackness: self.blackness,
        }
    }
    fn to_rgb(&self) -> RGB {
        let h = self.hue / 360.0;
        let w = self.whiteness / 100.0;
        let b = self.blackness / 100.0;
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        if w + b >= 1.0 {
            let s = w + b - 1.0;
            r = (w - s) / (w + b - s);
            g = (b - s) / (w + b - s);
            b = s;
        } else {
            let s = 1.0 - w - b;
            r = (w + s) / (1.0 - b);
            g = (b + s) / (1.0 - w);
            b = s;
        }
        RGB {
            red: (r * 255.0) as u8,
            green: (g * 255.0) as u8,
            blue: (b * 255.0) as u8,
        }
    }
}
impl CMYK {
    pub fn new(cyan: f64, magenta: f64, yellow: f64, black: f64) -> CMYK {
        CMYK {
            cyan,
            magenta,
            yellow,
            black,
        }
    }
}
impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> RGB {
        RGB {
            red,
            green,
            blue,
        }
    }
}
impl HSL {
    pub fn new(hue: f64, saturation: f64, lightness: f64) -> HSL {
        HSL {
            hue,
            saturation,
            lightness,
        }
    }
}
impl HWB {
    pub fn new(hue: f64, whiteness: f64, blackness: f64) -> HWB {
        HWB {
            hue,
            whiteness,
            blackness,
        }
    }
}

