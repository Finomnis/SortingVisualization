mod sortable_data;
mod sorting_algorithms;
mod visualizations;

use sortable_data::SortableData;
use visualizations::{ConsoleVisualization, ImageVisualization};

fn main() {
    // Two passes required:
    // First: figure out the number of steps required
    // Second: write steps to image
    let num_steps = SortableData::new(128)
        .sort(sorting_algorithms::insertion_sort)
        .num_steps();

    let mut console_visualization = ConsoleVisualization::new();
    let mut image_visualization = ImageVisualization::new(128, 256, num_steps);

    SortableData::new(128)
        .add_visualization(&mut console_visualization)
        .add_visualization(&mut image_visualization)
        .sort(sorting_algorithms::insertion_sort);

    image_visualization.save("test.png");
}
