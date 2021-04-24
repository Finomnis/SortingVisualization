use crate::sortable_data::SortableData;

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Selection_sort
    for i in 0..data.len() {
        let mut j_min = i;
        for j in (i + 1)..data.len() {
            if data[j] < data[j_min] {
                j_min = j;
            }
        }
        if j_min != i {
            data.swap(i, j_min);
        }
    }
}

crate::test_algorithm!();
