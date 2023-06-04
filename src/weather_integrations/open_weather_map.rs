use crate::weather_integrations::open_weather_map_serializers::OpenWeatherBody;
use crate::weather_integrations::WeatherIntegration;
use clap::Subcommand;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use tokio::runtime::Runtime;

pub struct OpenWeatherMapProcessor {
    api_key: Option<String>,
    lat: String,
    lon: String,
}

impl OpenWeatherMapProcessor {
    pub(crate) fn new(api_key: Option<String>, lat: String, lon: String) -> Self {
        OpenWeatherMapProcessor { api_key, lat, lon }
    }
}

const CURRENT_WEATHER_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

impl WeatherIntegration for OpenWeatherMapProcessor {
    fn make_request(&self) -> Result<OpenWeatherBody, String> {
        let url = self.get_data_for_request();
        println!("url: {}", url);
        let raw_response = Client::new()
            .get(url)
            .timeout(std::time::Duration::new(10, 00))
            .send()
            .expect("Unexpected response");
        if raw_response.status().is_success() {
            let json_response = raw_response
                .json::<OpenWeatherBody>()
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
        let weather = &response.weather[0].description;
        let temperature = &response.main.temp;
        let pressure = &response.main.pressure;
        let humidity = &response.main.humidity;

        let parsed_string = format!(
            "Weather: {}, Temperature: {} K, Pressure: {} hPa, Humidity: {}%",
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
        let api_key = self.api_key.clone().unwrap();
        let url = format!(
            "{}?lat={}&lon={}&appid={}",
            CURRENT_WEATHER_URL, self.lat, self.lon, api_key
        );
        url
    }
}
