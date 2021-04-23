mod sortable_data;
mod sorting_algorithms;

use crate::sortable_data::SortableData;

fn main() {
    let data = SortableData::new(8).sort(sorting_algorithms::insertion_sort);
    println!("{}", data);
}
