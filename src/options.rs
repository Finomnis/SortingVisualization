use itertools::Itertools;

use clap::Parser;

use crate::sorting_algorithms::get_algorithms;
use crate::visualizations::image::color_palettes::get_palettes;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Options {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,

    /// The color palette to use for the final image
    #[clap(long, default_value="gray", possible_values = get_palettes().keys().cloned().sorted())]
    pub palette: String,

    /// Only run specified sorting algorithm
    #[clap(long, possible_values = get_algorithms().keys().cloned().sorted())]
    pub algorithm: Option<String>,

    /// The width of the output image
    #[clap(long, default_value = "400")]
    pub width: usize,

    /// The height of the output image
    #[clap(long, default_value = "400")]
    pub height: usize,
}

pub fn parse_command_line_options() -> Options {
    Options::parse()
}
