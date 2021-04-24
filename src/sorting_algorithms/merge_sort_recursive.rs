use crate::sortable_data::SortableData;

fn read_partition(data: &SortableData, start: usize, size: usize) -> Vec<u32> {
    let mut result = vec![];
    for i in start..(start + size) {
        if i >= data.len() {
            break;
        }
        result.push(data[i]);
    }
    result
}

fn merge(
    data: &mut SortableData,
    partition_0: usize,
    partition_0_size: usize,
    partition_1: usize,
    partition_1_size: usize,
) {
    let data_0 = read_partition(data, partition_0, partition_0_size);
    let data_1 = read_partition(data, partition_1, partition_1_size);
    let mut data_0_slice = &data_0[..];
    let mut data_1_slice = &data_1[..];

    let mut pos = partition_0;
    loop {
        if data_0_slice.is_empty() {
            for &element in data_1_slice {
                data[pos] = element;
                pos += 1;
            }
            break;
        }
        if data_1_slice.is_empty() {
            for &element in data_0_slice {
                data[pos] = element;
                pos += 1;
            }
            break;
        }
        if data_0_slice[0] < data_1_slice[0] {
            data[pos] = data_0_slice[0];
            pos += 1;
            data_0_slice = &data_0_slice[1..];
        } else {
            data[pos] = data_1_slice[0];
            pos += 1;
            data_1_slice = &data_1_slice[1..];
        }
    }
}

fn merge_sort(data: &mut SortableData, partition_start: usize, partition_size: usize) {
    if partition_size < 2 {
        return;
    }
    let partition_0_start = partition_start;
    let partition_0_size = partition_size / 2;
    let partition_1_start = partition_start + partition_0_size;
    let partition_1_size = partition_size - partition_0_size;
    merge_sort(data, partition_0_start, partition_0_size);
    merge_sort(data, partition_1_start, partition_1_size);
    merge(
        data,
        partition_0_start,
        partition_0_size,
        partition_1_start,
        partition_1_size,
    );
}

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Merge_sort
    merge_sort(data, 0, data.len());
}

crate::test_algorithm!();
