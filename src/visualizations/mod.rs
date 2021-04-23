mod console;

pub use console::ConsoleVisualization;
pub trait SortingVisualization {
    fn on_data_changed(&self, data: &Vec<f32>);
}
