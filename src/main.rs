mod sortable_data;
mod sorting_algorithms;
mod visualizations;

use crate::{sortable_data::SortableData, visualizations::ConsoleVisualization};

fn main() {
    let mut console_visualization = ConsoleVisualization::new();

    SortableData::new(16)
        .add_visualization(&mut console_visualization)
        .sort(sorting_algorithms::insertion_sort);

    SortableData::new(16)
        .add_visualization(&mut console_visualization)
        .sort(sorting_algorithms::insertion_sort);
}
