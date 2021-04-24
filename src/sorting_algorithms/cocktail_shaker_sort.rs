use crate::sortable_data::SortableData;

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Cocktail_shaker_sort
    loop {
        let mut swapped = false;

        for i in 0..(data.len() - 1) {
            // test whether the two elements are in the wrong order
            if data[i] > data[i + 1] {
                data.swap(i, i + 1); // let the two elements change places
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
        swapped = false;

        for i in (0..(data.len() - 1)).rev() {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

crate::test_algorithm!();
