mod options;
mod sortable_data;
mod sorting_algorithms;
mod visualizations;

use std::{
    fs::create_dir_all,
    sync::{Arc, Mutex},
};

use sortable_data::SortableData;
use sorting_algorithms::get_algorithms;
use visualizations::console::ConsoleVisualization;
use visualizations::image::color_palettes::get_palettes;
use visualizations::image::ImageVisualization;

#[tokio::main]
async fn main() {
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
        // First: figure out the number of frames required
        // Second: write frames to image
        let num_frames = {
            let mut data = SortableData::new(options.width);
            data.sort(algorithm).await;
            let num_frames = data.num_frames();

            log::info!("First iteration done. Frames: {}", num_frames);
            log::debug!("Result if first run: {}", data);
            log::debug!("Sorted: {}", data.is_sorted());
            num_frames
        };

        let mut console_visualization = Arc::new(Mutex::new(ConsoleVisualization::new()));
        let mut image_visualization = Arc::new(Mutex::new(
            ImageVisualization::new(options.width, options.height, num_frames)
                .use_color_palette(get_palettes()[options.palette.as_str()]),
        ));

        let mut data = SortableData::new(options.width)
            .add_visualization(console_visualization.clone())
            .add_visualization(image_visualization.clone());
        data.sort(algorithm).await;

        create_dir_all("images/").unwrap();
        image_visualization
            .lock()
            .unwrap()
            .save(&format!("images/{}.png", name));
    }
}
