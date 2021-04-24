use async_recursion::async_recursion;

use crate::sortable_data::AsyncSortableData;

#[async_recursion]
async fn quicksort(data: AsyncSortableData, lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(&data, lo, hi).await;

        if p > 0 {
            tokio::join!(
                quicksort(data.clone(), lo, p - 1),
                quicksort(data, p + 1, hi)
            );
        } else {
            quicksort(data, p + 1, hi).await;
        }
    }
}

async fn partition(data: &AsyncSortableData, lo: usize, hi: usize) -> usize {
    let pivot = data.read().unwrap()[hi];
    let mut i = lo;
    for j in lo..hi {
        if data.read().unwrap()[j] < pivot {
            data.write().unwrap().swap(i, j);
            tokio::task::yield_now().await;
            i += 1;
        }
    }
    data.write().unwrap().swap(i, hi);
    tokio::task::yield_now().await;
    i
}

pub async fn sort(data: AsyncSortableData) {
    // Source: https://en.wikipedia.org/wiki/Quicksort
    let len = data.read().unwrap().len();
    quicksort(data, 0, len - 1).await;
}

crate::test_async_algorithm!();
