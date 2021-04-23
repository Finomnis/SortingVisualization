mod options;
mod sortable_data;
mod sorting_algorithms;
mod visualizations;

use std::fs::create_dir_all;

use sortable_data::SortableData;
use sorting_algorithms::get_algorithms;
use visualizations::console::ConsoleVisualization;
use visualizations::image::color_palettes::get_palettes;
use visualizations::image::ImageVisualization;

fn main() {
    let options = options::parse_command_line_options();

    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(
        match options.verbose {
            0 => "info",
            1 => "debug",
            _ => "trace",
        },
    ))
    .init();

    for (name, algorithm) in get_algorithms() {
        // If an algorithm is specified on the command line, only run that algorithm
        if let Some(wanted_algorithm) = &options.algorithm {
            if wanted_algorithm != name {
                continue;
            }
        }

        log::info!("Running {} ...", name);

        // Two passes required:
        // First: figure out the number of steps required
        // Second: write steps to image
        let num_steps = {
            let result = SortableData::new(options.width).sort(algorithm);
            let num_steps = result.num_steps();

            log::info!("First iteration done. Steps: {}", num_steps);
            log::debug!("Result if first run: {}", result);
            num_steps
        };

        let mut console_visualization = ConsoleVisualization::new();
        let mut image_visualization =
            ImageVisualization::new(options.width, options.height, num_steps)
                .use_color_palette(get_palettes()[options.palette.as_str()]);

        SortableData::new(options.width)
            .add_visualization(&mut console_visualization)
            .add_visualization(&mut image_visualization)
            .sort(algorithm);

        create_dir_all("images/").unwrap();
        image_visualization.save(&format!("images/{}.png", name));
    }
}
