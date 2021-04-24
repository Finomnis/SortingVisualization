use std::sync::{Arc, Mutex};

use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::sorting_algorithms::SortingAlgorithm;
use crate::visualizations::SortingVisualization;

pub struct SortableData {
    data: Vec<u32>,
    pub rng: ChaCha20Rng,
    visualizations: Vec<Arc<Mutex<dyn SortingVisualization>>>,
    frame_counter: usize,
}

impl SortableData {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::new();
        for i in 0..size {
            let value = i as u32;
            data.push(value);
        }

        let mut rng = ChaCha20Rng::seed_from_u64(42);

        data.shuffle(&mut rng);

        Self {
            data,
            rng,
            visualizations: vec![],
            frame_counter: 0,
        }
    }

    pub async fn sort(&mut self, algorithm: SortingAlgorithm) {
        for visualization in &mut self.visualizations {
            visualization.lock().unwrap().on_start();
        }
        algorithm(self);
        self.send_frame();
        for visualization in &mut self.visualizations {
            visualization.lock().unwrap().on_finished();
        }
    }

    pub fn add_visualization(
        mut self,
        visualization: Arc<Mutex<dyn SortingVisualization>>,
    ) -> Self {
        self.visualizations.push(visualization);
        self
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.send_frame();
        self.data.swap(a, b);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn num_frames(&self) -> usize {
        self.frame_counter
    }

    fn send_frame(&mut self) {
        self.frame_counter += 1;
        for visualization in &mut self.visualizations {
            visualization.lock().unwrap().on_data_changed(&self.data);
        }
    }

    pub fn is_sorted(&self) -> bool {
        let mut prev = u32::MIN;
        for &next in &self.data {
            if prev > next {
                return false;
            }
            prev = next;
        }
        return true;
    }
}

impl std::ops::Index<usize> for SortableData {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl std::ops::IndexMut<usize> for SortableData {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.send_frame();
        self.data.index_mut(index)
    }
}

impl std::fmt::Display for SortableData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
