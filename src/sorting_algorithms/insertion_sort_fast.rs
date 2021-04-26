use crate::sortable_data::SortableData;

pub fn insertion_sort(data: &mut SortableData, start: usize, end: usize) {
    // Source: https://en.wikipedia.org/wiki/Insertion_sort
    if end < start + 2 {
        return;
    }
    for i in (start + 1)..end {
        let x = data[i];
        for j in (start..i).rev() {
            let el = data[j];
            if el <= x {
                data[j + 1] = x;
                break;
            }
            data[j + 1] = el;
            if j == start {
                data[start] = x;
            }
        }
    }
}

pub fn sort(data: &mut SortableData) {
    insertion_sort(data, 0, data.len())
}

crate::test_algorithm!();
