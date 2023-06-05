use serde::Deserializer;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherApiResponse {
    pub current: Current,
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
