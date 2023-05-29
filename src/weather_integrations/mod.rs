mod open_weather_map;

trait WeatherIntegration {
    fn make_request(&self) -> Result<serde_json::Value, String>;
    fn parse_response(&self) -> serde_json::Result<String>;
    fn get_data_for_request(&self) -> String;
}
