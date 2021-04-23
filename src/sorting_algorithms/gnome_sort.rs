use crate::sortable_data::SortableData;

pub fn sort(data: &mut SortableData) {
    // Source: https://en.wikipedia.org/wiki/Gnome_sort
    let mut pos = 0;
    while pos < data.len() {
        if pos == 0 || data[pos] >= data[pos - 1] {
            pos = pos + 1;
        } else {
            data.swap(pos, pos - 1);
            pos = pos - 1
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
