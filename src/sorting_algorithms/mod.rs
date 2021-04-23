mod bubble_sort;
mod cycle_sort;
mod insertion_sort;
mod selection_sort;

use crate::sortable_data::SortableData;
use std::collections::HashMap;

pub type SortingAlgorithm = fn(data: &mut SortableData) -> ();

pub fn get_algorithms() -> HashMap<&'static str, SortingAlgorithm> {
    let mut algorithms: HashMap<&'static str, SortingAlgorithm> = HashMap::new();

    algorithms.insert("bubble_sort", bubble_sort::sort);
    algorithms.insert("insertion_sort", insertion_sort::sort);
    algorithms.insert("selection_sort", selection_sort::sort);
    algorithms.insert("cycle_sort", cycle_sort::sort);

    algorithms
}
