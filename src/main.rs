mod command_line_interface;
mod weather_integrations;

use crate::custom_config::{Configuration, ConfigurationHandler};
use crate::weather_integrations::WeatherIntegration;
use command_line_interface::custom_config;
use std::time::Instant;
use weather_integrations::open_weather_map::OpenWeatherMapProcessor;
use weather_integrations::weather_api::WeatherApiProcessor;
use command_line_interface::{get_provider_and_api_key_args, get_weather_args};

const OPEN_WEATHER_MAP_NAME: &str = "OpenWeatherMap";
const WEATHER_API_NAME: &str = "WeatherApi";

fn get_current_weather(lat: String, lon: String, config_values: Configuration) {
    match config_values.provider.as_str() {
        OPEN_WEATHER_MAP_NAME => {
            let open_weather_map_processor = OpenWeatherMapProcessor::new(
                config_values.api_key,
                String::from(lat),
                String::from(lon),
            );
            open_weather_map_processor.parse_response();
        }
        WEATHER_API_NAME => {
            let weather_api_processor = WeatherApiProcessor::new(
                config_values.api_key,
                String::from(lat),
                String::from(lon),
            );
            weather_api_processor.parse_response();
        }
        _ => {}
    }
}

fn get_specific_date_weather_forecast(lat: String, lon: String, config_values: Configuration){

}

fn main() {
    let start = Instant::now();
    let matches = command_line_interface::register_args();
    // Matches the commands and performs actions
    match matches.subcommand() {
        Some(("configure", sub_m)) => {
            let (provider, api_key) = get_provider_and_api_key_args(sub_m);
            let config = Configuration { provider, api_key };
            let config_handler = ConfigurationHandler {};
            config_handler.wright_configuration_for_weather_provider(config)
        }
        Some(("get", sub_m)) => {
            let (lat, lon, date) = get_weather_args(sub_m);
            let config_handler = ConfigurationHandler {};
            let config_values =
                config_handler.get_configuration_for_particular_weather_provider()
                    .expect("Please, configure weather provider first");
            if date.is_some(){
                return;
            } else {
                get_current_weather(lat, lon, config_values);
            }
        }
        _ => (),
    }
    let end = start.elapsed();
    println!("Time spent = {:.2?}", end);
}
