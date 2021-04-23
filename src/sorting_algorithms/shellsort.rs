use crate::sortable_data::SortableData;

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Shellsort
    let gaps = vec![701, 301, 132, 57, 23, 10, 4, 1];

    // Start with the largest gap and work down to a gap of 1
    for gap in gaps {
        // Do a gapped insertion sort for this gap size.
        // The first gap elements a[0..gap-1] are already in gapped order
        // keep adding one more element until the entire array is gap sorted
        for i in gap..data.len() {
            // add a[i] to the elements that have been gap sorted
            // save a[i] in temp and make a hole at position i
            let temp = data[i];
            // shift earlier gap-sorted elements up until the correct location for a[i] is found
            let mut j = i;
            while j >= gap && data[j - gap] > temp {
                data[j] = data[j - gap];
                j -= gap;
            }
            // put temp (the original a[i]) in its correct location
            data[j] = temp;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort() {
        let result = crate::sortable_data::SortableData::new(10000).sort(super::sort);
        assert!(result.is_sorted());
    }
}

/*# Sort an array a[0...n-1].
gaps = [701, 301, 132, 57, 23, 10, 4, 1]

# Start with the largest gap and work down to a gap of 1
foreach (gap in gaps)
{
    # Do a gapped insertion sort for this gap size.
    # The first gap elements a[0..gap-1] are already in gapped order
    # keep adding one more element until the entire array is gap sorted
    for (i = gap; i < n; i += 1)
    {
        # add a[i] to the elements that have been gap sorted
        # save a[i] in temp and make a hole at position i
        temp = a[i]
        # shift earlier gap-sorted elements up until the correct location for a[i] is found
        for (j = i; j >= gap and a[j - gap] > temp; j -= gap)
        {
            a[j] = a[j - gap]
        }
        # put temp (the original a[i]) in its correct location
        a[j] = temp
    }
}*/
