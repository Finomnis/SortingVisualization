use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Options {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
}

pub fn parse_command_line_options() -> Options {
    Options::parse()
}
