use crate::weather_integrations::open_weather_map_serializers::OpenWeatherBody;

pub(crate) mod open_weather_map;
mod open_weather_map_serializers;

pub trait WeatherIntegration {
    fn make_request(&self) -> Result<OpenWeatherBody, String>;
    fn parse_response(&self) -> serde_json::Result<String>;
    fn get_data_for_request(&self) -> String;
}
