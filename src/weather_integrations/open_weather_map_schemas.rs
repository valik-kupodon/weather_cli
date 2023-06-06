use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenWeatherForecastResponse{
    pub list: Vec<List>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List{
    pub weather: Vec<Weather>,
    pub temp: Temperature,
    pub pressure: i32,
    pub humidity: i8,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature{
    pub day: f32,
    pub min: f32,
    pub max: f32,
    pub night: f32,
    pub eve: f32,
    pub morn: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenWeatherCurrentResponse {
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
    pub sea_level: i32,
    pub grnd_level: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}
