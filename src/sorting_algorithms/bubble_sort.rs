use crate::sortable_data::SortableData;

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Bubble_sort
    let n = data.len();
    loop {
        let mut swapped = false;
        for i in 1..n {
            if data[i - 1] > data[i] {
                data.swap(i - 1, i);
                swapped = true;
            }
        }
        if !swapped {
            break;
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
