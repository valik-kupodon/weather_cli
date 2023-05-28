use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct WeatherClIArgs{
    #[clap(subcommand)]
    pub weather_integration_type: Option<WeatherIntegrationType>,
}

#[derive(Debug, Subcommand)]
pub enum  WeatherIntegrationType {
    Configure(OpenWeatherMapConfigure),
    GET(LocationAndTime)
}

#[derive(Debug, Args)]
pub struct OpenWeatherMapConfigure{
    /// Coordinates in format <lan,lon>
    coordinates: String,
    /// Api key from https://openweathermap.org
    api_key: String
}

#[derive(Debug, Args)]
pub struct LocationAndTime{
    /// Coordinates in format <lan,lon>
    coordinates: Option<String>,
    /// Date if none was pass it will use current
    date: Option<String>
}
