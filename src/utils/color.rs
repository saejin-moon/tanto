use crate::utils::math::*;

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

impl Color {
    pub fn new () -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255
        }
    }
    // sky is blue ahh functions
    pub fn red (&mut self, r: u8) -> &mut Self {
        self.red = r;
        self
    }
    pub fn green (&mut self, g: u8) -> &mut Self {
        self.green = g;
        self
    }
    pub fn blue (&mut self, b: u8) -> &mut Self {
        self.blue = b;
        self
    }
    pub fn alpha (&mut self, a: u8) -> &mut Self {
        self.alpha = a;
        self
    }
    pub fn rgb (&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.red = r;
        self.green = g;
        self.blue = b;
        self
    }
    pub fn rgba (&mut self, r: u8, g: u8, b: u8, a: u8) -> &mut Self {
        self.red = r;
        self.green = g;
        self.blue = b;
        self.alpha = a;
        self
    }
    pub fn set_cmyk(&mut self, c: f32, m: f32, y: f32, k: f32) -> &mut Self {
        // this was a lot simpler than i thought it would be
        self.red = (255. * (1. - c) * (1. - k)) as u8;
        self.green = (255. * (1. - m) * (1. - k)) as u8;
        self.blue = (255. * (1. - y) * (1. - k)) as u8;
        self
    }
    // nahhh bro what is this. what are we even doin' atp.
    pub fn set_hsl(&mut self, h: u16, s: f32, l: f32) -> Option<&mut Self> {
        // i've had an easier time with calculus
        let c = (1. - (2. * l - 1.).abs()) * s;
        let x = c * (1. - (((h / 60) as f32) % 2. - 1.).abs());
        let m = l - c / 2.;
        let (r, g, b) = match h {
            0..60 => (c, x, 0.),
            60..120 => (x, c, 0.),
            120..180 => (0., c, x),
            180..240 => (0., x, c),
            240..300 => (x, 0., c),
            300..360 => (c, 0., x),
            _ => return None
        };
        self.red = ((r + m) * 255.) as u8;
        self.green = ((g + m) * 255.) as u8;
        self.blue = ((b + m) * 255.) as u8;
        Some(self)
    }
    pub fn set_hex(&mut self, string: &str) -> Option<&mut Self> {
        let hex = string.trim_start_matches("#");
        // copy-paste demon out here fr
        let (r, g, b, a) = match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                let a = 255;
                (r, g, b, a)
            }
            4 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).ok()?;
                (r, g, b, a)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = 255;
                (r, g, b, a)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                (r, g, b, a)
            }
            _ => return None
        };
        self.red = r;
        self.green = g;
        self.blue = b;
        self.alpha = a;
        Some(self)
    }
    
    // grass is green ahh functions
    pub fn get_red(&self) -> u8 {
        self.red
    }
    pub fn get_green(&self) -> u8 {
        self.green
    }
    pub fn get_blue(&self) -> u8 {
        self.blue
    }
    pub fn get_alpha(&self) -> u8 {
        self.alpha
    }
    pub fn get_alphaf(&self) -> f32 {
        self.alpha as f32 / 255.
    }
    pub fn get_rgb(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
    pub fn get_rgbf(&self) -> (f32, f32, f32) {
        (self.red as f32 / 255., self.green as f32 / 255., self.blue as f32 / 255.)
    }
    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }
    pub fn get_rgbaf(&self) -> (f32, f32, f32, f32) {
        (self.red as f32 / 255., self.green as f32 / 255., self.blue as f32 / 255., self.alpha as f32 / 255.)
    }
    // smooooth operatorrrr
    pub fn get_hex(&self) -> String {
        let hex = format!("#{:02x}{:02x}{:02x}{:02x}", self.red, self.green, self.blue, self.alpha);
        hex
    }
    // relatively simple, just some shuffling around
    pub fn get_cmyk(&self) -> (f32, f32, f32, f32) {
        let (r, g, b) = self.get_rgbf();
        let k = 1. - max(max(r, g), b);
        let c = (1. - r - k) / (1. - k);
        let m = (1. - g - k) / (1. - k);
        let y = (1. - b - k) / (1. - k);
        (c, m, y, k)
    }
    // huhhhh
    pub fn get_hsl(&self) -> (u16, f32, f32) {
        let (r, g, b) = self.get_rgbf();
        let max_color = max(max(r, g), b);
        let min_color = min(min(r, g), b);
        let delta = max_color - min_color;
        let l = (max_color + min_color) / 2.;
        let s;
        let mut h;
        if delta == 0. {
            s = 0.;
            h = 0.;
        }
        else {
            s = delta / (1. - (2. * l - 1.).abs());
            h = match max_color {
                x if x == r => 60. * ((g - b) / delta % 6.),
                x if x == g => 60. * ((b - r) / delta + 2.),
                _ => 60. * ((r - g) / delta + 4.)
            }
        }
        if h < 0. {
            h += 360.;
        }
        (h as u16, s, l)
    }
}
