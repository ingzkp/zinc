//!
//! The Zinc server arguments.
//!

use structopt::StructOpt;

///
/// The Zinc server arguments.
///
#[derive(StructOpt)]
#[structopt(
    name = zinc_const::app_name::ZINC_SERVER,
    about = "The Zinc server"
)]
pub struct Arguments {
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    /// The HTTP server port.
    #[structopt(
        short = "p",
        long = "port",
        help = "The HTTP server port",
        default_value = "80"
    )]
    pub port: u16,
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }
}
