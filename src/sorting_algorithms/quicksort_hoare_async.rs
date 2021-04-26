use async_recursion::async_recursion;

use crate::sortable_data::AsyncSortableData;

#[async_recursion]
async fn quicksort(data: AsyncSortableData, lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(&data, lo, hi).await;
        if lo + 1 < p && p + 1 < hi {
            tokio::join!(
                quicksort(data.clone(), lo, p - 1),
                quicksort(data, p + 1, hi)
            );
        } else if lo + 1 < p {
            quicksort(data.clone(), lo, p - 1).await;
        } else if p + 1 < hi {
            quicksort(data, p + 1, hi).await;
        }
    }
}

async fn partition(data: &AsyncSortableData, lo: usize, hi: usize) -> usize {
    let pivot = data.read().unwrap()[(hi + lo) / 2];
    let mut i = lo;
    let mut j = hi;
    loop {
        while i < j && data.read().unwrap()[i] < pivot {
            i = i + 1;
        }
        while i < j && data.read().unwrap()[j] > pivot {
            j = j - 1;
        }
        if i >= j {
            return j;
        }
        data.write().unwrap().swap(i, j);
        tokio::task::yield_now().await;
    }
}

pub async fn sort(data: AsyncSortableData) {
    // Source: https://en.wikipedia.org/wiki/Quicksort
    let len = data.read().unwrap().len();
    quicksort(data, 0, len - 1).await;
}

crate::test_async_algorithm!();
