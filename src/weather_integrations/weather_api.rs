use crate::weather_integrations::WeatherIntegration;
use reqwest::blocking::Client;
use crate::weather_integrations::weather_api_schemas::WeatherApiForecastResponse;

pub struct WeatherApiProcessor {
    pub(crate) api_key: String,
    pub(crate) lat: String,
    pub(crate) lon: String,
    pub(crate) date: i64
}

impl WeatherApiProcessor {
    pub(crate) fn new(api_key: String, lat: String, lon: String, date: i64) -> Self {
        WeatherApiProcessor { api_key, lat, lon, date }
    }
}

const WEATHER_URL: &str = "http://api.weatherapi.com/v1/forecast.json?q=";


impl WeatherIntegration for WeatherApiProcessor {
    type Response = Result<WeatherApiForecastResponse, String>;
    fn make_request(&self) -> Result<WeatherApiForecastResponse, String> {
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
                .json::<WeatherApiForecastResponse>()
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

    fn parse_response(&self) {
        // Implement the logic to parse the response JSON and extract the weather data
        // In this example, we simply return the response as-is as a Vec<u8>
        let response = self.make_request().unwrap();
        if self.date > 0 {
            let weather = &response.forecast.forecastday[(self.date - 1) as usize].day.condition.text;
            let temperature = &response.forecast.forecastday[(self.date - 1) as usize].day.avgtemp_f;
            let humidity = &response.forecast.forecastday[(self.date - 1) as usize].day.avghumidity;
            let parsed_string = format!(
                "Weather: {}, Temperature: {} F, Humidity: {}%",
                weather, temperature, humidity
            );
            println!("{}", parsed_string);
        } else {
            let weather = &response.current.condition.text;
            let temperature = &response.current.temp_f;
            let pressure = &response.current.pressure_mb;
            let humidity = &response.current.humidity;
            let parsed_string = format!(
                "Weather: {}, Temperature: {} F, Pressure: {} hPa, Humidity: {}%",
                weather, temperature, pressure, humidity
            );
            println!("{}", parsed_string);
        }
    }

    fn get_data_for_request(&self) -> String {
        // Implement the logic to retrieve the latitude, longitude, and API key
        // In this example, we use hardcoded values for demonstration purposes
        // let lat = "51.5074";
        // let lon = "48.1278";
        let url = format!(
            "{}{},{}&days={}",
            WEATHER_URL, self.lat, self.lon, self.date
        );
        url
    }
}