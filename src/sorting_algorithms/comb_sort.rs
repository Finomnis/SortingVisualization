use crate::sortable_data::SortableData;

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Comb_sort
    let mut gap = data.len(); // Initialize gap size
    let shrink = 1.3f32; // Set the gap shrink factor
    let mut sorted = false;

    while sorted == false {
        // Update the gap value for a next comb
        gap = (gap as f32 / shrink).floor() as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true; // If there are no swaps this pass, we are done
        }

        // A single "comb" over the input list
        let mut i = 0;
        while i + gap < data.len() {
            // See Shell sort for a similar idea
            if data[i] > data[i + gap] {
                data.swap(i, i + gap);
                sorted = false;
                // If this assignment never happens within the loop,
                // then there have been no swaps and the list is sorted.
            }

            i = i + 1;
        }
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
