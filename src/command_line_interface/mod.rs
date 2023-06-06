pub mod custom_config;

use chrono::{Local, NaiveDate};
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

pub fn get_provider_and_api_key_args(sub_m: &ArgMatches) -> (String, String) {
    let packages: Vec<_> = sub_m
        .get_many::<String>("provider")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
    let provider = packages.join(", ");
    let packages: Vec<_> = sub_m
        .get_many::<String>("api_key")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
    let api_key = packages.join(", ");
    (provider, api_key)
}

pub fn get_weather_args(sub_m: &ArgMatches) -> (String, String, Option<String>) {
    let latitude_vec: Vec<_> = sub_m
        .get_many::<String>("latitude")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
    let latitude = latitude_vec.join(",");
    let longitude_vec: Vec<_> = sub_m
        .get_many::<String>("longitude")
        .expect("contains_id")
        .map(|s| s.as_str())
        .collect();
    let longitude = longitude_vec.join(",");
    let date = None;
    if sub_m.contains_id("date") {
        let packages: Vec<_> = sub_m
            .get_many::<String>("date")
            .expect("contains_id")
            .map(|s| s.as_str())
            .collect();
        let date = packages.join(", ");
        return (latitude, longitude, Option::from(date));
    }
    return (latitude, longitude, date);
}

pub fn convert_date_to_amount_of_days(date: &str) -> Option<i64> {
    // Get the current date
    let current_date = Local::now().naive_local();

    // Parse the input date string
    let input_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap_or(current_date.into());

    // Calculate the difference in days
    let days_difference = input_date.signed_duration_since(NaiveDate::from(current_date)).num_days();

    Some(days_difference)
}
