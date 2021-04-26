use crate::sortable_data::SortableData;

fn get_parent(pos: usize) -> usize {
    (pos - 1) / 2
}

fn get_children(pos: usize) -> (usize, usize) {
    (2 * pos + 1, 2 * pos + 2)
}

fn bubble_up(data: &mut SortableData, element: usize) {
    if element == 0 {
        return;
    }

    let parent = get_parent(element);
    if data[parent] < data[element] {
        data.swap(parent, element);
        bubble_up(data, parent);
    }
}

fn bubble_down(data: &mut SortableData, element: usize, heap_size: usize) {
    let (child_0, child_1) = get_children(element);

    if child_0 >= heap_size && child_1 >= heap_size {
        return;
    }

    let child = if child_1 >= heap_size {
        assert!(child_0 < heap_size);
        child_0
    } else if data[child_0] > data[child_1] {
        child_0
    } else {
        child_1
    };

    if data[child] > data[element] {
        data.swap(child, element);
        bubble_down(data, child, heap_size);
    }
}

pub fn sort(data: &mut SortableData) {
    for element in 0..data.len() {
        bubble_up(data, element);
    }

    for element in (1..data.len()).rev() {
        data.swap(element, 0);
        bubble_down(data, 0, element)
    }
}

crate::test_algorithm!();
