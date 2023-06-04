use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = ".weather_cli.txt";

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub provider: Option<String>,
    pub api_key: Option<String>,
}

impl Configuration {
    pub fn wright_configuration_for_weather_provider(&self) {
        let config_file = self.get_config_file();
        if !config_file.exists() {
            self.create_config_file();
        }
        let config = self;

        let config_json =
            serde_json::to_string_pretty(&config).expect("Failed to serialize configuration");
        fs::write(config_file, config_json).expect("Failed to write configuration file");

        println!("Configuration saved.");
    }

    pub fn get_configuration_for_particular_weather_provider(
        &self,
    ) -> Result<Configuration, Box<dyn std::error::Error>> {
        let config_file = self.get_config_file();
        if !config_file.exists() {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::NotFound,
                "Please configure provider first",
            )));
        }
        let config_map: HashMap<String, String> = self._read_json_file(config_file)?;
        Ok(Configuration {
            provider: Some(
                config_map
                    .get("provider")
                    .ok_or("Please configure provider first")?
                    .to_owned(),
            ),
            api_key: Some(
                config_map
                    .get("api_key")
                    .ok_or("Please configure provider first")?
                    .to_owned(),
            ),
        })
    }

    fn get_config_dir(&self) -> PathBuf {
        let config_dir = dirs::home_dir().expect("Could not find home directory");
        return config_dir;
    }
    fn get_config_file(&self) -> PathBuf {
        let mut config_file = self.get_config_dir();
        config_file.push(CONFIG_FILE_NAME);
        config_file
    }

    fn create_config_file(&self) {
        let config_file_path = self.get_config_file();
        File::create(config_file_path).expect("Could not create configuration file");
    }

    fn _read_json_file(&self, file_path: PathBuf) -> serde_json::Result<HashMap<String, String>> {
        let mut file = File::open(file_path).expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("File damaged");

        let parsed_json: serde_json::Result<HashMap<String, Value>> =
            serde_json::from_str(&contents);
        let parsed_json = parsed_json?;

        let mut string_map: HashMap<String, String> = HashMap::new();
        for (key, value) in parsed_json {
            if let Value::String(string_value) = value {
                string_map.insert(key, string_value);
            }
        }

        Ok(string_map)
    }
}
