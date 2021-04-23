mod console;
mod image;

pub use self::image::ImageVisualization;
pub use console::ConsoleVisualization;

pub trait SortingVisualization {
    fn on_start(&mut self, data: &Vec<f32>);
    fn on_data_changed(&mut self, data: &Vec<f32>);
    fn on_finished(&mut self);
}
