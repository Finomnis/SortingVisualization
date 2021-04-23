mod options;
mod sortable_data;
mod sorting_algorithms;
mod visualizations;

use sortable_data::SortableData;
use visualizations::console::ConsoleVisualization;
use visualizations::image::{color_palettes, ImageVisualization};

fn main() {
    let options = options::parse_command_line_options();

    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(
        match options.verbose {
            0 => "warn",
            1 => "info",
            2 => "debug",
            _ => "trace",
        },
    ))
    .init();

    // Two passes required:
    // First: figure out the number of steps required
    // Second: write steps to image
    let num_steps = SortableData::new(512)
        .sort(sorting_algorithms::insertion_sort)
        .num_steps();

    let mut console_visualization = ConsoleVisualization::new();
    let mut image_visualization =
        ImageVisualization::new(512, 1024, num_steps).use_color_palette(color_palettes::rainbow);

    SortableData::new(512)
        .add_visualization(&mut console_visualization)
        .add_visualization(&mut image_visualization)
        .sort(sorting_algorithms::bubble_sort);

    image_visualization.save("test.png");
}
