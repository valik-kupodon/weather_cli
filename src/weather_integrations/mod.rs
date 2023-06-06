use crate::{Configuration, OpenWeatherMapCurrentProcessor, OpenWeatherMapForecastProcessor, WeatherApiProcessor};
use crate::{OPEN_WEATHER_MAP_NAME, WEATHER_API_NAME};

pub(crate) mod open_weather_map;
mod open_weather_map_schemas;
pub(crate) mod weather_api;
mod weather_api_schemas;

pub trait WeatherIntegration {
    type Response;
    fn make_request(&self) -> Self::Response;
    fn parse_response(&self);
    fn get_data_for_request(&self) -> String;
}

pub enum WeatherIntegrationTypes{
    OpenWeatherCurrent(OpenWeatherMapCurrentProcessor),
    OpenWeatherForecast(OpenWeatherMapForecastProcessor),
    WeatherApi(WeatherApiProcessor)
}

pub fn weather_integration_router(lat: String, lon: String, config: Configuration, date: i64) -> WeatherIntegrationTypes{
    match config.provider.as_str() {
        OPEN_WEATHER_MAP_NAME => {
            if date > 0 {
                let processing_struct = OpenWeatherMapForecastProcessor::new(
                    config.api_key,
                    lat,
                    lon,
                    date
                    );
                WeatherIntegrationTypes::OpenWeatherForecast(processing_struct)
            } else {
                let processing_struct = OpenWeatherMapCurrentProcessor::new(
                    config.api_key,
                    lat,
                    lon,
                    );
                WeatherIntegrationTypes::OpenWeatherCurrent(processing_struct)
            }
        }
        WEATHER_API_NAME => {
            let processing_struct = WeatherApiProcessor::new(
                config.api_key,
                lat,
                lon,
                date
                );
            WeatherIntegrationTypes::WeatherApi(processing_struct)
        }
        _ => panic!("Invalid processing name"),
    }
}
