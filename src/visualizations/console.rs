use super::SortingVisualization;

pub struct ConsoleVisualization {
    iterations: usize,
}

impl ConsoleVisualization {
    pub fn new() -> Self {
        Self { iterations: 0 }
    }
}

impl SortingVisualization for ConsoleVisualization {
    fn on_start(&mut self, data: &Vec<f32>) {
        log::info!("Started ...");
        log::debug!("{:?}", data);
    }

    fn on_data_changed(&mut self, data: &Vec<f32>) {
        log::debug!("{:?}", data);
        self.iterations += 1;
    }

    fn on_finished(&mut self) {
        log::info!("Finished. ({} iterations)", self.iterations)
    }
}
