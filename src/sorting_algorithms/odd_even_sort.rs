use crate::sortable_data::SortableData;

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Odd-even_sort

    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in (1..(data.len() - 1)).step_by(2) {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                sorted = false;
            }
        }
        for i in (0..(data.len() - 1)).step_by(2) {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}

crate::test_algorithm!();
