use super::SortingVisualization;

pub struct ConsoleVisualization {}

impl ConsoleVisualization {
    pub fn new() -> Self {
        Self {}
    }
}

impl SortingVisualization for ConsoleVisualization {
    fn on_data_changed(&self, data: &Vec<f32>) {
        println!("{:?}", data);
    }
}
