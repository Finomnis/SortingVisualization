use clap::{crate_version, AppSettings, Clap};

use crate::visualizations::image::color_palettes::get_palettes;
#[derive(Clap)]
#[clap(version = crate_version!())]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Options {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
    /// The color palette to use for the final image
    #[clap(long, default_value="rainbow", possible_values = &get_palettes().keys().cloned().collect::<Vec<_>>()[..])]
    pub palette: String,
}

pub fn parse_command_line_options() -> Options {
    Options::parse()
}
