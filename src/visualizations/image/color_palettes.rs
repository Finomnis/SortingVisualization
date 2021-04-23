use super::clamp;

pub type ColorPalette = fn(f32) -> [u8; 3];

pub fn grayscale(value: f32) -> [u8; 3] {
    let gray = clamp((value * 255.0).round() as u32);
    [gray, gray, gray]
}

pub fn rainbow(value: f32) -> [u8; 3] {
    let rgb: palette::rgb::Rgb = palette::Hsv::new(value * 240.0, 1.0, 1.0).into();
    let red = (rgb.red * 255.0).round() as u8;
    let green = (rgb.green * 255.0).round() as u8;
    let blue = (rgb.blue * 255.0).round() as u8;
    [red, green, blue]
}
