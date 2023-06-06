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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_integration_router_open_weather_map_forecast() {
        let lat = "40.7128".to_string();
        let lon = "-74.0060".to_string();
        let config = Configuration {
            provider: OPEN_WEATHER_MAP_NAME.to_string(),
            api_key: "api_key_123".to_string(),
        };
        let date = 7;

        let result = weather_integration_router(lat, lon, config, date);

        match result {
            WeatherIntegrationTypes::OpenWeatherForecast(processor) => {
                assert_eq!(processor.api_key, "api_key_123");
                assert_eq!(processor.lat, "40.7128");
                assert_eq!(processor.lon, "-74.0060");
                assert_eq!(processor.date, 7);
            },
            _ => panic!("Unexpected WeatherIntegrationTypes variant"),
        }
    }

    #[test]
    fn test_weather_integration_router_open_weather_map_current() {
        let lat = "40.7128".to_string();
        let lon = "-74.0060".to_string();
        let config = Configuration {
            provider: OPEN_WEATHER_MAP_NAME.to_string(),
            api_key: "api_key_123".to_string(),
        };
        let date = 0;

        let result = weather_integration_router(lat, lon, config, date);

        match result {
            WeatherIntegrationTypes::OpenWeatherCurrent(processor) => {
                assert_eq!(processor.api_key, "api_key_123");
                assert_eq!(processor.lat, "40.7128");
                assert_eq!(processor.lon, "-74.0060");
            },
            _ => panic!("Unexpected WeatherIntegrationTypes variant"),
        }
    }

    #[test]
    fn test_weather_integration_router_weather_api() {
        let lat = "40.7128".to_string();
        let lon = "-74.0060".to_string();
        let config = Configuration {
            provider: WEATHER_API_NAME.to_string(),
            api_key: "api_key_123".to_string(),
        };
        let date = 7;

        let result = weather_integration_router(lat, lon, config, date);

        match result {
            WeatherIntegrationTypes::WeatherApi(processor) => {
                assert_eq!(processor.api_key, "api_key_123");
                assert_eq!(processor.lat, "40.7128");
                assert_eq!(processor.lon, "-74.0060");
                assert_eq!(processor.date, 7);
            },
            _ => panic!("Unexpected WeatherIntegrationTypes variant"),
        }
    }

    #[test]
    #[should_panic(expected = "Invalid processing name")]
    fn test_weather_integration_router_invalid_provider() {
        let lat = "40.7128".to_string();
        let lon = "-74.0060".to_string();
        let config = Configuration {
            provider: "invalid_provider".to_string(),
            api_key: "api_key_123".to_string(),
        };
        let date = 7;

        weather_integration_router(lat, lon, config, date);
    }
}
