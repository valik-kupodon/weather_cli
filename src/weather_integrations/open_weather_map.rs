use crate::weather_integrations::WeatherIntegration;
use reqwest::Client;
use tokio::runtime::Runtime;

struct OpenWeatherMapProcessor {
    api_key: String,
    lat: String,
    lon: String,
    client: Client,
}

impl OpenWeatherMapProcessor {
    fn new(api_key: String, lat: String, lon: String) -> Self {
        OpenWeatherMapProcessor {
            api_key,
            lat,
            lon,
            client: Client::new(),
        }
    }
}

impl WeatherIntegration for OpenWeatherMapProcessor {
    fn make_request(&self) -> Result<serde_json::Value, String> {
        let url = self.get_data_for_request();
        let rt = Runtime::new().expect("Failed to create Tokio runtime");

        let client = Client::new();
        let response = rt
            .block_on(client.get(url).send())
            .expect("Service is not available now");

        if response.status().is_success() {
            let body = rt.block_on(response.text()).expect("Expect json");
            let json_response: serde_json::Value =
                serde_json::from_str(&body.as_str()).expect("Unexpected error");
            Ok(json_response)
        } else {
            let err_msg = format!("Request failed with status: {}", response.status());
            println!("Error: {}", err_msg.as_str());
            Err(err_msg)
        }
    }

    fn parse_response(&self) -> serde_json::Result<String> {
        // Implement the logic to parse the response JSON and extract the weather data
        // In this example, we simply return the response as-is as a Vec<u8>
        let response = self.make_request().expect("Failed to parse json");
        let weather = &response["weather"][0]["description"];
        let temperature = &response["main"]["temp"];
        let wind_speed = &response["wind"]["speed"];
        let pressure = &response["main"]["pressure"];
        let humidity = &response["main"]["humidity"];

        let parsed_string = format!(
            "Weather: {}, Temperature: {} K, Wind Speed: {} m/s, Pressure: {} hPa, Humidity: {}%",
            weather, temperature, wind_speed, pressure, humidity
        );
        Ok(parsed_string)
    }

    fn get_data_for_request(&self) -> String {
        // Implement the logic to retrieve the latitude, longitude, and API key
        // In this example, we use hardcoded values for demonstration purposes
        // let lat = "51.5074";
        // let lon = "48.1278";
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
            self.lat.as_str(), self.lon.as_str(), self.api_key.as_str()
        );
        url
    }
}
