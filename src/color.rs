//! Color space conversions and perceptual distance.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

/// Converts RGB to CIELAB using D65 illuminant.
pub fn rgb_to_lab(r: u8, g: u8, b: u8) -> Lab {
    // 1. RGB to linear XYZ
    let mut r = r as f32 / 255.0;
    let mut g = g as f32 / 255.0;
    let mut b = b as f32 / 255.0;

    r = if r > 0.04045 { ((r + 0.055) / 1.055).powf(2.4) } else { r / 12.92 };
    g = if g > 0.04045 { ((g + 0.055) / 1.055).powf(2.4) } else { g / 12.92 };
    b = if b > 0.04045 { ((b + 0.055) / 1.055).powf(2.4) } else { b / 12.92 };

    let x = r * 0.4124 + g * 0.3576 + b * 0.1805;
    let y = r * 0.2126 + g * 0.7152 + b * 0.0722;
    let z = r * 0.0193 + g * 0.1192 + b * 0.9505;

    // 2. XYZ to CIELAB (D65)
    let x = x / 0.95047;
    let y = y / 1.00000;
    let z = z / 1.08883;

    let f = |t: f32| if t > 0.008856 { t.powf(1.0/3.0) } else { 7.787 * t + 16.0/116.0 };

    Lab {
        l: 116.0 * f(y) - 16.0,
        a: 500.0 * (f(x) - f(y)),
        b: 200.0 * (f(y) - f(z)),
    }
}

pub fn lab_distance_sq(c1: Lab, c2: Lab) -> f32 {
    let dl = c1.l - c2.l;
    let da = c1.a - c2.a;
    let db = c1.b - c2.b;
    dl * dl + da * da + db * db
}
