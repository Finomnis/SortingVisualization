use crate::sortable_data::SortableData;

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Insertion_sort
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j, j - 1);
            j = j - 1;
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
