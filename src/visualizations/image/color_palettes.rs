use std::collections::HashMap;

use super::clamp;

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

macro_rules! add_colorous {
    ($palettes:expr, $gradient:expr) => {
        $palettes.insert(stringify!($gradient), move |val| {
            let col = $gradient.eval_continuous(val as f64);
            [col.r, col.g, col.b]
        });
    };
}

pub fn get_palettes() -> HashMap<&'static str, ColorPalette> {
    let mut palettes: HashMap<&'static str, ColorPalette> = HashMap::new();
    palettes.insert("gray", grayscale);
    palettes.insert("rainbow", rainbow);
    add_colorous!(palettes, colorous::VIRIDIS);
    add_colorous!(palettes, colorous::BLUES);
    add_colorous!(palettes, colorous::BLUE_GREEN);
    add_colorous!(palettes, colorous::BLUE_PURPLE);
    add_colorous!(palettes, colorous::BROWN_GREEN);
    add_colorous!(palettes, colorous::CIVIDIS);
    add_colorous!(palettes, colorous::COOL);
    add_colorous!(palettes, colorous::CUBEHELIX);
    add_colorous!(palettes, colorous::GREENS);
    add_colorous!(palettes, colorous::GREEN_BLUE);
    add_colorous!(palettes, colorous::GREYS);
    add_colorous!(palettes, colorous::INFERNO);
    add_colorous!(palettes, colorous::MAGMA);
    add_colorous!(palettes, colorous::ORANGES);
    add_colorous!(palettes, colorous::ORANGE_RED);
    add_colorous!(palettes, colorous::PINK_GREEN);
    add_colorous!(palettes, colorous::PLASMA);
    add_colorous!(palettes, colorous::PURPLES);
    add_colorous!(palettes, colorous::PURPLE_BLUE);
    add_colorous!(palettes, colorous::PURPLE_BLUE_GREEN);
    add_colorous!(palettes, colorous::PURPLE_GREEN);
    add_colorous!(palettes, colorous::PURPLE_ORANGE);
    add_colorous!(palettes, colorous::PURPLE_RED);
    add_colorous!(palettes, colorous::RAINBOW);
    add_colorous!(palettes, colorous::REDS);
    add_colorous!(palettes, colorous::RED_BLUE);
    add_colorous!(palettes, colorous::RED_GREY);
    add_colorous!(palettes, colorous::RED_PURPLE);
    add_colorous!(palettes, colorous::RED_YELLOW_BLUE);
    add_colorous!(palettes, colorous::RED_YELLOW_GREEN);
    add_colorous!(palettes, colorous::SINEBOW);
    add_colorous!(palettes, colorous::SPECTRAL);
    add_colorous!(palettes, colorous::TURBO);
    add_colorous!(palettes, colorous::VIRIDIS);
    add_colorous!(palettes, colorous::WARM);
    add_colorous!(palettes, colorous::YELLOW_GREEN);
    add_colorous!(palettes, colorous::YELLOW_GREEN_BLUE);
    add_colorous!(palettes, colorous::YELLOW_ORANGE_BROWN);
    add_colorous!(palettes, colorous::YELLOW_ORANGE_RED);

    palettes
}
