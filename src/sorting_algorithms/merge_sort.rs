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

fn merge(data: &mut SortableData, partition_0: usize, partition_1: usize, partition_size: usize) {
    let data_0 = read_partition(data, partition_0, partition_size);
    let data_1 = read_partition(data, partition_1, partition_size);
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

fn merge_all(data: &mut SortableData, partition_size: usize) {
    for i in (0..data.len()).step_by(2 * partition_size) {
        merge(data, i, i + partition_size, partition_size);
    }
}

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Merge_sort
    let mut partition_size = 1;
    while partition_size < data.len() {
        merge_all(data, partition_size);
        partition_size *= 2;
    }
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
