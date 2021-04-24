use std::{
    cmp::{max, min},
    collections::HashMap,
};

use super::color_palettes::ColorPalette;

fn from_colormap(value: f32, colormap: &[[f32; 3]; 256]) -> [u8; 3] {
    let num_entries = colormap.len();
    let desired_entry = (num_entries - 1) as f32 * value;
    let entry_0 = min(
        num_entries - 2,
        max(0, desired_entry.floor() as i32) as usize,
    );
    let entry_1 = entry_0 + 1;
    let percent_1 = (desired_entry - entry_0 as f32).clamp(0.0, 1.0);
    let percent_0 = 1.0 - percent_1;

    let [r0, g0, b0] = colormap[entry_0];
    let [r1, g1, b1] = colormap[entry_1];

    let r = ((r0 * percent_0 + r1 * percent_1) * 255.0).clamp(0.0, 255.0) as u8;
    let g = ((g0 * percent_0 + g1 * percent_1) * 255.0).clamp(0.0, 255.0) as u8;
    let b = ((b0 * percent_0 + b1 * percent_1) * 255.0).clamp(0.0, 255.0) as u8;

    [r, g, b]
}

macro_rules! register_colormap {
    ($palettes:ident, $colormap:ident) => {
        $palettes.insert(stringify!($colormap), |value: f32| {
            from_colormap(value, &$colormap::colormap())
        });
    };
}

mod autumn;
mod bone;
mod cool;
mod copper;
mod hot;
mod hsv;
mod jet;
mod parula;
mod pink;
mod spring;
mod summer;
mod turbo;
mod winter;

pub fn register_colormaps(palettes: &mut HashMap<&'static str, ColorPalette>) {
    register_colormap!(palettes, parula);
    register_colormap!(palettes, turbo);
    register_colormap!(palettes, hsv);
    register_colormap!(palettes, hot);
    register_colormap!(palettes, cool);
    register_colormap!(palettes, spring);
    register_colormap!(palettes, summer);
    register_colormap!(palettes, autumn);
    register_colormap!(palettes, winter);
    register_colormap!(palettes, bone);
    register_colormap!(palettes, copper);
    register_colormap!(palettes, pink);
    register_colormap!(palettes, jet);
}
