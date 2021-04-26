use crate::sortable_data::SortableData;

fn get_parent(start: usize, pos: usize) -> usize {
    ((pos - start) - 1) / 2 + start
}

fn get_children(start: usize, pos: usize) -> (usize, usize) {
    let pos = pos - start;
    (2 * pos + 1 + start, 2 * pos + 2 + start)
}

fn bubble_up(data: &mut SortableData, start: usize, element: usize) {
    if element == start {
        return;
    }

    let parent = get_parent(start, element);
    if data[parent] < data[element] {
        data.swap(parent, element);
        bubble_up(data, start, parent);
    }
}

fn bubble_down(data: &mut SortableData, element: usize, start: usize, heap_size: usize) {
    let (child_0, child_1) = get_children(start, element);

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
        bubble_down(data, child, start, heap_size);
    }
}

pub fn heapsort(data: &mut SortableData, start: usize, end: usize) {
    for element in start..end {
        bubble_up(data, start, element);
    }

    for element in ((start + 1)..end).rev() {
        data.swap(element, start);
        bubble_down(data, start, start, element)
    }
}

pub fn sort(data: &mut SortableData) {
    heapsort(data, 0, data.len());
}

crate::test_algorithm!();
