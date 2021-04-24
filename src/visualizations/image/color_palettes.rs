use std::collections::HashMap;

use super::clamp;
use super::colormaps::register_colormaps;

pub type ColorPalette = fn(f32) -> [u8; 3];

fn grayscale(value: f32) -> [u8; 3] {
    let gray = clamp((value * 255.0).round() as u32);
    [gray, gray, gray]
}

fn rainbow(value: f32) -> [u8; 3] {
    let rgb: palette::rgb::Rgb = palette::Hsv::new(value * 240.0, 1.0, 1.0).into();
    let red = (rgb.red * 255.0).round() as u8;
    let green = (rgb.green * 255.0).round() as u8;
    let blue = (rgb.blue * 255.0).round() as u8;
    [red, green, blue]
}

pub fn get_palettes() -> HashMap<&'static str, ColorPalette> {
    let mut palettes: HashMap<&'static str, ColorPalette> = HashMap::new();

    palettes.insert("gray", grayscale);
    palettes.insert("rainbow", rainbow);
    register_colormaps(&mut palettes);

    palettes
}
