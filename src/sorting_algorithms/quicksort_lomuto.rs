use crate::sortable_data::SortableData;

fn quicksort(data: &mut SortableData, lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(data, lo, hi);
        if p > 0 {
            quicksort(data, lo, p - 1);
        }
        quicksort(data, p + 1, hi);
    }
}

fn partition(data: &mut SortableData, lo: usize, hi: usize) -> usize {
    let pivot = data[hi];
    let mut i = lo;
    for j in lo..hi {
        if data[j] < pivot {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, hi);
    i
}

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Quicksort
    quicksort(data, 0, data.len() - 1);
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn sort() {
        let mut result = crate::sortable_data::SortableData::new(10000);
        result.sort(super::sort).await;
        assert!(result.is_sorted());
    }
}
