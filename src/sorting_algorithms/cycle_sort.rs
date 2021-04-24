use crate::sortable_data::SortableData;

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Cycle_sort

    // Loop through the array to find cycles to rotate.
    for cycle_start in 0..data.len() {
        let mut item = data[cycle_start];

        // Find where to put the item.
        let mut pos = cycle_start;
        for i in (cycle_start + 1)..data.len() {
            if data[i] < item {
                pos += 1;
            }
        }

        // If the item is already there, this is not a cycle.
        if pos == cycle_start {
            continue;
        }

        // Otherwise, put the item there or right after any duplicates.
        while item == data[pos] {
            pos += 1
        }
        {
            let tmp = data[pos];
            data[pos] = item;
            item = tmp;
        }

        // Rotate the rest of the cycle.
        while pos != cycle_start {
            // Find where to put the item.
            pos = cycle_start;
            for i in (cycle_start + 1)..data.len() {
                if data[i] < item {
                    pos += 1
                }
            }

            // Put the item there or right after any duplicates.
            while item == data[pos] {
                pos += 1
            }
            {
                let tmp = data[pos];
                data[pos] = item;
                item = tmp;
            }
        }
    }
}

crate::test_algorithm!();
