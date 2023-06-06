mod command_line_interface;
mod weather_integrations;

use crate::custom_config::{Configuration, ConfigurationHandler};
use crate::weather_integrations::{weather_integration_router, WeatherIntegration, WeatherIntegrationTypes};
use command_line_interface::custom_config;
use std::time::Instant;
use weather_integrations::open_weather_map::OpenWeatherMapCurrentProcessor;
use weather_integrations::weather_api::WeatherApiProcessor;
use command_line_interface::{get_provider_and_api_key_args, get_weather_args};
use crate::command_line_interface::convert_date_to_amount_of_days;
use crate::weather_integrations::open_weather_map::OpenWeatherMapForecastProcessor;

const OPEN_WEATHER_MAP_NAME: &str = "OpenWeatherMap";
const WEATHER_API_NAME: &str = "WeatherApi";

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
            let days = get_amount_of_days(date).unwrap_or(0_i64);
            let processing_struct = weather_integration_router(
                lat, lon, config_values, days
            );
            match processing_struct {
                WeatherIntegrationTypes::WeatherApi(weather_api_processor) => {
                    weather_api_processor.parse_response()
                }
                WeatherIntegrationTypes::OpenWeatherCurrent(open_weather_current_processor) => {
                    open_weather_current_processor.parse_response()
                }
                WeatherIntegrationTypes::OpenWeatherForecast(open_weather_forecast_processor) => {
                    open_weather_forecast_processor.parse_response()
                }
            }

        }
        _ => (),
    }
    let end = start.elapsed();
    println!("Time spent = {:.2?}", end);
}

fn get_amount_of_days(date: Option<String>) -> Result<i64, ()> {
    if let Some(date_str) = date {
        if let Some(days) = convert_date_to_amount_of_days(date_str.as_str()) {
            if !(0..=14).contains(&days) {
                println!("Forecast available for 14 days only. The current weather:");
                Err(())
            } else {
                Ok(days)
            }
        } else {
            println!("Invalid date format");
            Err(())
        }
    } else {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_amount_of_days_valid_date() {
        let date = Some("2023-06-01".to_string());
        let result = get_amount_of_days(date);

        assert_eq!(result, Err(()));
    }

    #[test]
    fn test_get_amount_of_days_invalid_date() {
        let date = Some("2023-06-abc".to_string());
        let result = get_amount_of_days(date);

        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_get_amount_of_days_no_date() {
        let date = None;
        let result = get_amount_of_days(date);

        assert_eq!(result, Ok(0));
    }
}
