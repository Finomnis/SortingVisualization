mod bubble_sort;
mod cocktail_shaker_sort;
mod comb_sort;
mod cycle_sort;
mod insertion_sort;
mod merge_sort;
mod merge_sort_async;
mod merge_sort_recursive;
mod odd_even_sort;
mod quicksort_hoare;
mod quicksort_hoare_async;
mod quicksort_lomuto;
mod quicksort_lomuto_async;
mod selection_sort;
mod shellsort;

use futures::future::BoxFuture;
use std::collections::HashMap;

use crate::sortable_data::AsyncSortableData;

pub type SortingAlgorithm = fn(data: AsyncSortableData) -> BoxFuture<'static, ()>;

macro_rules! register_async_algorithm {
    ($algorithms:ident, $name:ident) => {
        $algorithms.insert(stringify!($name), |data| Box::pin($name::sort(data)));
    };
}

macro_rules! register_algorithm {
    ($algorithms:ident, $name:ident) => {
        $algorithms.insert(stringify!($name), |data| {
            $name::sort(&mut data.write().unwrap());
            Box::pin(std::future::ready(()))
        });
    };
}

#[macro_export]
macro_rules! test_async_algorithm {
    () => {
        #[cfg(test)]
        mod tests {
            use crate::sortable_data::SortableData;
            use std::sync::{Arc, RwLock};

            #[tokio::test]
            async fn sort() {
                let result = Arc::new(RwLock::new(SortableData::new(10000)));
                SortableData::sort(result.clone(), |data| Box::pin(super::sort(data))).await;
                assert!(result.read().unwrap().is_sorted());
            }
        }
    };
}

#[macro_export]
macro_rules! test_algorithm {
    () => {
        #[cfg(test)]
        mod tests {
            use crate::sortable_data::SortableData;
            use std::sync::{Arc, RwLock};

            #[tokio::test]
            async fn sort() {
                let result = Arc::new(RwLock::new(SortableData::new(10000)));
                SortableData::sort(result.clone(), |data| {
                    super::sort(&mut data.write().unwrap());
                    Box::pin(std::future::ready(()))
                })
                .await;
                assert!(result.read().unwrap().is_sorted());
            }
        }
    };
}

pub fn get_algorithms() -> HashMap<&'static str, SortingAlgorithm> {
    let mut algorithms: HashMap<&'static str, SortingAlgorithm> = HashMap::new();

    register_algorithm!(algorithms, bubble_sort);
    register_algorithm!(algorithms, insertion_sort);
    register_algorithm!(algorithms, selection_sort);
    register_algorithm!(algorithms, cycle_sort);
    register_algorithm!(algorithms, cocktail_shaker_sort);
    register_algorithm!(algorithms, comb_sort);
    register_algorithm!(algorithms, odd_even_sort);
    register_algorithm!(algorithms, shellsort);
    register_algorithm!(algorithms, quicksort_hoare);
    register_async_algorithm!(algorithms, quicksort_hoare_async);
    register_algorithm!(algorithms, quicksort_lomuto);
    register_async_algorithm!(algorithms, quicksort_lomuto_async);
    register_algorithm!(algorithms, merge_sort);
    register_algorithm!(algorithms, merge_sort_recursive);
    register_async_algorithm!(algorithms, merge_sort_async);

    algorithms
}
