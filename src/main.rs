mod command_line_interface;
mod weather_integrations;

use crate::custom_config::Configuration;
use crate::weather_integrations::WeatherIntegration;
use command_line_interface::custom_config;
use std::time::Instant;
use weather_integrations::open_weather_map::OpenWeatherMapProcessor;

fn main() {
    let start = Instant::now();
    let matches = command_line_interface::register_args();
    // Matches the commands and performs actions
    match matches.subcommand() {
        Some(("configure", sub_m)) => {
            let packages: Vec<_> = sub_m
                .get_many::<String>("provider")
                .expect("contains_id")
                .map(|s| s.as_str())
                .collect();
            let provider = packages.join(", ");
            let packages: Vec<_> = sub_m
                .get_many::<String>("api_key")
                .expect("contains_id")
                .map(|s| s.as_str())
                .collect();
            let api_key = packages.join(", ");
            println!("Configuring provider: {}, {}", provider, api_key);
            let config = custom_config::Configuration {
                provider: Some(provider),
                api_key: Some(api_key),
            };
            config.wright_configuration_for_weather_provider()
        }
        Some(("get", sub_m)) => {
            let latitude_vec: Vec<_> = sub_m
                .get_many::<String>("latitude")
                .expect("contains_id")
                .map(|s| s.as_str())
                .collect();
            let latitude = latitude_vec.join(",");
            let longitude_vec: Vec<_> = sub_m
                .get_many::<String>("longitude")
                .expect("contains_id")
                .map(|s| s.as_str())
                .collect();
            let longitude = longitude_vec.join(",");
            println!(
                "latitude: {}, longitude: {}, latitude_vec {:?}",
                latitude, longitude, latitude_vec
            );
            if sub_m.contains_id("date") {
                let packages: Vec<_> = sub_m
                    .get_many::<String>("date")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let date = packages.join(", ");
                println!("Configuring provider: {}", date);
            }
            let cofiguration = Configuration {
                provider: None,
                api_key: None,
            };
            let config_values =
                Configuration::get_configuration_for_particular_weather_provider(&cofiguration)
                    .unwrap();
            let open_weather_map_processor = OpenWeatherMapProcessor::new(
                config_values.api_key,
                String::from(latitude),
                String::from(longitude),
            );
            open_weather_map_processor.parse_response().unwrap();
        }
        _ => (),
    }
    let end = start.elapsed();
    println!("Time spent = {:.2?}", end);
}
