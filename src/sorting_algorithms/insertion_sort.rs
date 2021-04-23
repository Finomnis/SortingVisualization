use crate::sortable_data::SortableData;

pub fn insertion_sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Insertion_sort
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j, j - 1);
            j = j - 1;
        }
    }
}
