pub mod custom_config;

use clap::{Arg, ArgMatches, Command};

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
                    Arg::new("address")
                        .help("Sets the address to get weather for")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("date")
                        .help("Sets the date (optional, defaults to now)")
                        .required(false)
                        .index(2),
                ),
        )
        .get_matches()
}
