mod bubble_sort;
mod insertion_sort;

pub use bubble_sort::bubble_sort;
pub use insertion_sort::insertion_sort;

use crate::sortable_data::SortableData;
use std::collections::HashMap;

pub type SortingAlgorithm = fn(data: &mut SortableData) -> ();

pub fn get_algorithms() -> HashMap<&'static str, SortingAlgorithm> {
    let mut algorithms: HashMap<&'static str, SortingAlgorithm> = HashMap::new();

    algorithms.insert("bubble_sort", bubble_sort);
    algorithms.insert("insertion_sort", insertion_sort);

    algorithms
}
