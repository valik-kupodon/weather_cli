use crate::weather_integrations::open_weather_map_schemas::OpenWeatherBody;
use crate::weather_integrations::WeatherIntegration;
use clap::Subcommand;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use tokio::runtime::Runtime;
use crate::weather_integrations::weather_api_schemas::WeatherApiResponse;

pub struct WeatherApiProcessor {
    api_key: String,
    lat: String,
    lon: String,
}

impl WeatherApiProcessor {
    pub(crate) fn new(api_key: String, lat: String, lon: String) -> Self {
        WeatherApiProcessor { api_key, lat, lon }
    }
}

const CURRENT_WEATHER_URL: &str = "http://api.weatherapi.com/v1/current.json?q=";


impl WeatherIntegration for WeatherApiProcessor {
    type Response = Result<WeatherApiResponse, String>;
    fn make_request(&self) -> Result<WeatherApiResponse, String> {
        let url = self.get_data_for_request();
        println!("url: {}", url);
        let raw_response = Client::new()
            .get(url)
            .timeout(std::time::Duration::new(10, 00))
            .header("key", &self.api_key)
            .send()
            .expect("Unexpected response");
        if raw_response.status().is_success() {
            let json_response = raw_response
                .json::<WeatherApiResponse>()
                .expect("Unexpected body");
            Ok(json_response)
        } else if raw_response.status().is_client_error() {
            println!("status: {}", raw_response.status());
            Err(format!("Please check your credetials"))
        } else if raw_response.status().is_server_error() {
            Err(format!(
                "Sorry, currently weather service is not available.\
                Please, try again later or configure another one"
            ))
        } else {
            Err(format!("Unexpected status"))
        }
    }

    fn parse_response(&self) -> serde_json::Result<String> {
        // Implement the logic to parse the response JSON and extract the weather data
        // In this example, we simply return the response as-is as a Vec<u8>
        let response = self.make_request().unwrap();
        let weather = &response.current.condition.text;
        let temperature = &response.current.temp_f;
        let pressure = &response.current.pressure_mb;
        let humidity = &response.current.humidity;

        let parsed_string = format!(
            "Weather: {}, Temperature: {} F, Pressure: {} hPa, Humidity: {}%",
            weather, temperature, pressure, humidity
        );
        println!("{}", parsed_string);
        Ok(parsed_string)
    }

    fn get_data_for_request(&self) -> String {
        // Implement the logic to retrieve the latitude, longitude, and API key
        // In this example, we use hardcoded values for demonstration purposes
        // let lat = "51.5074";
        // let lon = "48.1278";
        let url = format!(
            "{}{},{}",
            CURRENT_WEATHER_URL, self.lat, self.lon
        );
        url
    }
}