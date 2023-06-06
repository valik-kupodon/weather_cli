use crate::weather_integrations::open_weather_map_schemas::{OpenWeatherCurrentResponse, OpenWeatherForecastResponse};
use crate::weather_integrations::WeatherIntegration;
use reqwest::blocking::Client;

const CURRENT_WEATHER_URL: &str = "https://api.openweathermap.org/data/2.5/weather";
const FORECAST_WEATHER_URL: &str = "https://pro.openweathermap.org/data/2.5/forecast/climate";

pub struct OpenWeatherMapCurrentProcessor {
    pub(crate) api_key: String,
    pub(crate) lat: String,
    pub(crate) lon: String,
}

impl OpenWeatherMapCurrentProcessor {
    pub(crate) fn new(api_key: String, lat: String, lon: String) -> Self {
        OpenWeatherMapCurrentProcessor { api_key, lat, lon}
    }
}

impl WeatherIntegration for OpenWeatherMapCurrentProcessor {
    type Response = Result<OpenWeatherCurrentResponse, String>;
    fn make_request(&self) -> Result<OpenWeatherCurrentResponse, String> {
        let url = self.get_data_for_request();
        println!("url: {}", url);
        let raw_response = Client::new()
            .get(url)
            .timeout(std::time::Duration::new(10, 00))
            .send()
            .expect("Unexpected response");
        if raw_response.status().is_success() {
            let json_response = raw_response
                .json::<OpenWeatherCurrentResponse>()
                .expect("Unexpected body");
            Ok(json_response)
        } else if raw_response.status().is_client_error() {
            println!("status: {}", raw_response.status());
            Err("Please check your credetials".to_string())
        } else if raw_response.status().is_server_error() {
            Err("Sorry, currently weather service is not available.\
                Please, try again later or configure another one".to_string())
        } else {
            Err(format!("Unexpected status"))
        }
    }

    fn parse_response(&self){
        // Implement the logic to parse the response JSON and extract the weather data
        // In this example, we simply return the response as-is as a Vec<u8>
        let response = self.make_request().unwrap();
        let weather = &response.weather[0].description;
        let temperature = &response.main.temp;
        let pressure = &response.main.pressure;
        let humidity = &response.main.humidity;

        let parsed_string = format!(
            "Weather: {}, Temperature: {} F, Pressure: {} hPa, Humidity: {}%",
            weather, temperature, pressure, humidity
        );
        println!("{}", parsed_string);
    }

    fn get_data_for_request(&self) -> String {
        // Implement the logic to retrieve the latitude, longitude, and API key
        // In this example, we use hardcoded values for demonstration purposes
        // let lat = "51.5074";
        // let lon = "48.1278";
        let url = format!(
            "{}?lat={}&lon={}&appid={}",
            CURRENT_WEATHER_URL, self.lat, self.lon, self.api_key
        );
        url
    }
}

pub struct OpenWeatherMapForecastProcessor {
    pub(crate) api_key: String,
    pub(crate) lat: String,
    pub(crate) lon: String,
    pub(crate) date: i64,
}

impl OpenWeatherMapForecastProcessor {
    pub(crate) fn new(api_key: String, lat: String, lon: String, date: i64) -> Self {
        OpenWeatherMapForecastProcessor { api_key, lat, lon, date}
    }
}

impl WeatherIntegration for OpenWeatherMapForecastProcessor {
    type Response = Result<OpenWeatherForecastResponse, String>;
    fn make_request(&self) -> Result<OpenWeatherForecastResponse, String> {
        let url = self.get_data_for_request();
        println!("url: {}", url);
        let raw_response = Client::new()
            .get(url)
            .timeout(std::time::Duration::new(10, 00))
            .send()
            .expect("Unexpected response");
        if raw_response.status().is_success() {
            let json_response = raw_response
                .json::<OpenWeatherForecastResponse>()
                .expect("Unexpected body");
            Ok(json_response)
        } else if raw_response.status().is_client_error() {
            println!("status: {}", raw_response.status());
            Err("Please check your credetials".to_string())
        } else if raw_response.status().is_server_error() {
            Err("Sorry, currently weather service is not available.\
                Please, try again later or configure another one".to_string())
        } else {
            Err(format!("Unexpected status"))
        }
    }

    fn parse_response(&self){
        // Implement the logic to parse the response JSON and extract the weather data
        // In this example, we simply return the response as-is as a Vec<u8>
        let response = self.make_request().unwrap();
        let weather = &response.list[self.date as usize].weather[0].description;
        let temperature = &response.list[self.date as usize].temp.day;
        let pressure = &response.list[self.date as usize].pressure;
        let humidity = &response.list[self.date as usize].humidity;

        let parsed_string = format!(
            "Weather: {}, Temperature: {} F, Pressure: {} hPa, Humidity: {}%",
            weather, temperature, pressure, humidity
        );
        println!("{}", parsed_string);
    }

    fn get_data_for_request(&self) -> String {
        // Implement the logic to retrieve the latitude, longitude, and API key
        // In this example, we use hardcoded values for demonstration purposes
        // let lat = "51.5074";
        // let lon = "48.1278";
        let url = format!(
            "{}?lat={}&lon={}&appid={}",
            FORECAST_WEATHER_URL, self.lat, self.lon, self.api_key
        );
        url
    }
}
