// mod bubble_sort;
// mod cocktail_shaker_sort;
// mod comb_sort;
// mod cycle_sort;
// mod insertion_sort;
// mod merge_sort;
// mod odd_even_sort;
// mod quicksort_hoare;
mod quicksort_hoare_async;
// mod quicksort_lomuto;
// mod selection_sort;
// mod shellsort;

use futures::future::BoxFuture;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::sortable_data::SortableData;

pub type SortingAlgorithm = fn(data: Arc<RwLock<SortableData>>) -> BoxFuture<'static, ()>;

pub fn get_algorithms() -> HashMap<&'static str, SortingAlgorithm> {
    let mut algorithms: HashMap<&'static str, SortingAlgorithm> = HashMap::new();

    // algorithms.insert("bubble_sort", bubble_sort::sort);
    // algorithms.insert("insertion_sort", insertion_sort::sort);
    // algorithms.insert("selection_sort", selection_sort::sort);
    // algorithms.insert("cycle_sort", cycle_sort::sort);
    // algorithms.insert("cocktail_shaker_sort", cocktail_shaker_sort::sort);
    // algorithms.insert("comb_sort", comb_sort::sort);
    // algorithms.insert("odd_even_sort", odd_even_sort::sort);
    // algorithms.insert("shellsort", shellsort::sort);
    // algorithms.insert("quicksort_hoare", quicksort_hoare::sort);
    algorithms.insert("quicksort_hoare_async", |data| {
        Box::pin(quicksort_hoare_async::sort(data))
    });
    // algorithms.insert("quicksort_lomuto", quicksort_lomuto::sort);
    // algorithms.insert("merge_sort", merge_sort::sort);

    algorithms
}
