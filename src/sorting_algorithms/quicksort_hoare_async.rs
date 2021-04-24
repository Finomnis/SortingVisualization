use async_recursion::async_recursion;
use std::sync::{Arc, RwLock};

use crate::sortable_data::SortableData;

#[async_recursion]
async fn quicksort(data: Arc<RwLock<SortableData>>, lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(&data, lo, hi).await;
        tokio::join!(quicksort(data.clone(), lo, p), quicksort(data, p + 1, hi));
    }
}

async fn partition(data: &Arc<RwLock<SortableData>>, lo: usize, hi: usize) -> usize {
    let pivot = data.read().unwrap()[(hi + lo) / 2];
    let mut i = lo;
    let mut j = hi;
    loop {
        while data.read().unwrap()[i] < pivot && i < j {
            i = i + 1;
        }
        while data.read().unwrap()[j] > pivot && i < j {
            j = j - 1;
        }
        if i >= j {
            return j;
        }
        data.write().unwrap().swap(i, j);
        tokio::task::yield_now().await;
    }
}

pub async fn sort(data: Arc<RwLock<SortableData>>) {
    // Source: https://en.wikipedia.org/wiki/Quicksort
    let len = data.read().unwrap().len();
    quicksort(data, 0, len - 1).await;
}

crate::test_async_algorithm!();
