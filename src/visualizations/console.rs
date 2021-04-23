use super::SortingVisualization;

pub struct ConsoleVisualization {
    iterations: usize,
    is_verbose: bool,
}

impl ConsoleVisualization {
    pub fn new() -> Self {
        Self {
            iterations: 0,
            is_verbose: false,
        }
    }
    pub fn verbose(mut self) -> Self {
        self.is_verbose = true;
        self
    }
}

impl SortingVisualization for ConsoleVisualization {
    fn on_start(&mut self, data: &Vec<f32>) {
        println!("Started ...");
        if self.is_verbose {
            println!("{:?}", data);
        }
    }

    fn on_data_changed(&mut self, data: &Vec<f32>) {
        if self.is_verbose {
            println!("{:?}", data);
        }
        self.iterations += 1;
    }

    fn on_finished(&mut self) {
        println!("Finished. ({} iterations)", self.iterations)
    }
}
