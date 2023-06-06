use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherApiForecastResponse{
    pub forecast: Forecast,
    pub current: Current,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast{
    pub forecastday: Vec<ForecastDay>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastDay{
    pub day: Day
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Day{
    pub avgtemp_f: f32,
    pub avghumidity: f32,
    pub condition: Condition
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    pub condition: Condition,
    pub temp_f: f32,
    pub pressure_mb: f32,
    pub humidity: i8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct  Condition {
    pub text: String,
    pub icon: String,
    pub code: i32
}
