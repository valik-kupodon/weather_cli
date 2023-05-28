mod command_line_interface;

use clap::Parser;
use command_line_interface::WeatherClIArgs;

fn main() {
    let args = WeatherClIArgs::parse();
    println!("Hello world!")
}
