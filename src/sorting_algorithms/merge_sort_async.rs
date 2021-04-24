use futures::future::join_all;

use crate::sortable_data::AsyncSortableData;

fn read_partition(data: &AsyncSortableData, start: usize, size: usize) -> Vec<u32> {
    let mut result = vec![];
    for i in start..(start + size) {
        if i >= data.read().unwrap().len() {
            break;
        }
        result.push(data.read().unwrap()[i]);
    }
    result
}

async fn merge(
    data: AsyncSortableData,
    partition_0: usize,
    partition_1: usize,
    partition_size: usize,
) {
    let data_0 = read_partition(&data, partition_0, partition_size);
    let data_1 = read_partition(&data, partition_1, partition_size);
    let mut data_0_slice = &data_0[..];
    let mut data_1_slice = &data_1[..];

    let mut pos = partition_0;
    loop {
        if data_0_slice.is_empty() {
            for &element in data_1_slice {
                data.write().unwrap()[pos] = element;
                tokio::task::yield_now().await;
                pos += 1;
            }
            break;
        }
        if data_1_slice.is_empty() {
            for &element in data_0_slice {
                data.write().unwrap()[pos] = element;
                tokio::task::yield_now().await;
                pos += 1;
            }
            break;
        }
        if data_0_slice[0] < data_1_slice[0] {
            data.write().unwrap()[pos] = data_0_slice[0];
            tokio::task::yield_now().await;
            pos += 1;
            data_0_slice = &data_0_slice[1..];
        } else {
            data.write().unwrap()[pos] = data_1_slice[0];
            tokio::task::yield_now().await;
            pos += 1;
            data_1_slice = &data_1_slice[1..];
        }
    }
}

async fn merge_all(data: &AsyncSortableData, partition_size: usize) {
    let futures = (0..data.read().unwrap().len())
        .step_by(2 * partition_size)
        .map(|i| merge(data.clone(), i, i + partition_size, partition_size));
    join_all(futures).await;
}

pub async fn sort(data: AsyncSortableData) {
    // Source: https://en.wikipedia.org/wiki/Merge_sort
    let mut partition_size = 1;
    let len = data.read().unwrap().len();
    while partition_size < len {
        merge_all(&data, partition_size).await;
        partition_size *= 2;
    }
}

crate::test_async_algorithm!();
