use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;

#[derive(Debug)]
pub struct SortableData {
    data: Vec<f32>,
    pub rng: ChaCha20Rng,
}

impl SortableData {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::new();
        for i in 0..size {
            let value = (i as f32) / ((size - 1) as f32);
            data.push(value);
        }

        let mut rng = ChaCha20Rng::seed_from_u64(42);

        data.shuffle(&mut rng);

        Self { data, rng }
    }

    pub fn sort(mut self, algorithm: impl Fn(&mut SortableData)) -> Self {
        algorithm(&mut self);
        self
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.data.swap(a, b);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl std::ops::Index<usize> for SortableData {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl std::fmt::Display for SortableData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
