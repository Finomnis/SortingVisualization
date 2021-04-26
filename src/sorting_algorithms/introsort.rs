use crate::sortable_data::SortableData;

use super::{heapsort::heapsort, insertion_sort_fast::insertion_sort, quicksort_hoare::partition};

fn introsort(data: &mut SortableData, start: usize, end: usize, maxdepth: usize) {
    let size = end - start;
    if size <= 1 {
        return; // base case
    } else if size <= 16 {
        insertion_sort(data, start, end);
    } else if maxdepth == 0 {
        heapsort(data, start, end);
    } else {
        let p = partition(data, start, end - 1); // assume this function does pivot selection, p is the final position of the pivot
        if p > start {
            introsort(data, start, p, maxdepth - 1);
        }
        if p + 1 < end {
            introsort(data, p + 1, end, maxdepth - 1);
        }
    }
}

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Introsort
    let maxdepth = (data.len() as f32).log(2.0).floor() as usize * 2;
    log::info!("Max depth: {}", maxdepth);
    introsort(data, 0, data.len(), maxdepth);
}

crate::test_algorithm!();
