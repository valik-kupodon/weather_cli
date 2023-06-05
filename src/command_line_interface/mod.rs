pub mod custom_config;

use clap::{Arg, ArgMatches, Command};
use crate::{OPEN_WEATHER_MAP_NAME, WEATHER_API_NAME};

pub fn register_args() -> ArgMatches {
    Command::new("Weather CLI")
        .version("1.0")
        .author("Your Name valentinefilatov2015@gmail.com")
        .about("Shows weather to a user")
        .subcommand(
            Command::new("configure")
                .about("Configures credentials for a provider")
                .arg(
                    Arg::new("provider")
                        .help("Sets the provider to configure")
                        .value_parser(clap::builder::PossibleValuesParser::new([OPEN_WEATHER_MAP_NAME, WEATHER_API_NAME]))
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("api_key")
                        .help("Sets the api_key for provider")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Shows weather for the provided address")
                .arg(
                    Arg::new("latitude")
                        .help("Sets the address in format string of lan and lon to get weather for")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("longitude")
                        .help("Sets the address in format string of lan and lon to get weather for")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("date")
                        .help("Sets the date (optional, defaults to now)")
                        .required(false)
                        .index(3),
                ),
        )
        .get_matches()
}
