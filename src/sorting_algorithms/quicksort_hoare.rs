use crate::sortable_data::SortableData;

fn quicksort(data: &mut SortableData, lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(data, lo, hi);
        if p > lo + 1 {
            quicksort(data, lo, p - 1);
        }
        if p + 1 < hi {
            quicksort(data, p + 1, hi);
        }
    }
}

pub fn partition(data: &mut SortableData, lo: usize, hi: usize) -> usize {
    let pivot = data[(hi + lo) / 2];
    let mut i = lo;
    let mut j = hi;
    loop {
        while i < j && data[i] < pivot {
            i = i + 1;
        }
        while i < j && data[j] > pivot {
            j = j - 1;
        }
        if i >= j {
            return j;
        }
        data.swap(i, j);
    }
}

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Quicksort
    quicksort(data, 0, data.len() - 1);
}

crate::test_algorithm!();
