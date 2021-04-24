pub mod console;
pub mod image;

pub trait SortingVisualization {
    fn on_start(&mut self);
    fn on_data_changed(&mut self, data: &Vec<u32>);
    fn on_finished(&mut self);
}
