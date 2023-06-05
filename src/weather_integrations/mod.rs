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


