use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeatherResponse {
    pub main: Main,
    pub weather: Vec<Weather>,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Main {
    pub temp: f64,
}

#[derive(Deserialize)]
pub struct Weather {
    pub description: String,
}

pub async fn fetch_weather(city: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let res = reqwest::get(&url).await?.json::<WeatherResponse>().await?;
    Ok(res)
}
