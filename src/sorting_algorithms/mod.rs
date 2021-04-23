mod bubble_sort;
mod cocktail_shaker_sort;
mod comb_sort;
mod cycle_sort;
mod gnome_sort;
mod insertion_sort;
mod odd_even_sort;
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
    algorithms.insert("cocktail_shaker_sort", cocktail_shaker_sort::sort);
    algorithms.insert("comb_sort", comb_sort::sort);
    algorithms.insert("gnome_sort", gnome_sort::sort);
    algorithms.insert("odd_even_sort", odd_even_sort::sort);

    algorithms
}
