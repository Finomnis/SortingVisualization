use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::visualizations::SortingVisualization;

pub struct SortableData<'a> {
    data: Vec<f32>,
    pub rng: ChaCha20Rng,
    visualizations: Vec<&'a mut dyn SortingVisualization>,
    step_counter: usize,
}

impl<'a> SortableData<'a> {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::new();
        for i in 0..size {
            let value = (i as f32) / ((size - 1) as f32);
            data.push(value);
        }

        let mut rng = ChaCha20Rng::seed_from_u64(42);

        data.shuffle(&mut rng);

        Self {
            data,
            rng,
            visualizations: vec![],
            step_counter: 0,
        }
    }

    pub fn sort(mut self, algorithm: impl Fn(&mut SortableData)) -> Self {
        for visualization in &mut self.visualizations {
            visualization.on_start(&self.data);
        }
        algorithm(&mut self);
        for visualization in &mut self.visualizations {
            visualization.on_finished();
        }
        self
    }

    pub fn add_visualization(mut self, visualization: &'a mut impl SortingVisualization) -> Self {
        self.visualizations.push(visualization);
        self
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.data.swap(a, b);
        self.step_counter += 1;
        for visualization in &mut self.visualizations {
            visualization.on_data_changed(&self.data);
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn num_steps(&self) -> usize {
        self.step_counter
    }
}

impl<'a> std::ops::Index<usize> for SortableData<'a> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<'a> std::fmt::Display for SortableData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
