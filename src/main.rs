mod command_line_interface;
mod weather_integrations;

use command_line_interface::custom_config;

fn main() {
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
            let config = custom_config::Configuration { provider, api_key };
            config.wright_configuration_for_weather_provider()
        }
        Some(("get", sub_m)) => {
            let packages: Vec<_> = sub_m
                .get_many::<String>("address")
                .expect("contains_id")
                .map(|s| s.as_str())
                .collect();
            let address = packages.join(", ");
                println!("Configuring provider: {}", address);
            let packages: Vec<_> = sub_m
                .get_many::<String>("date")
                .expect("contains_id")
                .map(|s| s.as_str())
                .collect();
            let date = packages.join(", ");
            println!("Configuring provider: {}", date);
        }
        _ => (),
    }
}
