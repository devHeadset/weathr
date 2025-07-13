mod env;
mod cli;
mod api;

use cli::Args;
use api::fetch_weather;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let api_key = match env::get_api_key() {
        Ok(k) => k,
        Err(e) => {
            eprintln!("❌ error loading API key: {}", e);
            return;
        }
    };

    match fetch_weather(&args.city, &api_key).await {
        Ok(weather) => {
            println!("🌍 city: {}", weather.name);
            println!("🌡️ temp: {}°C", weather.main.temp);
            println!("🌥️ weather: {}", weather.weather[0].description);
        }
        Err(e) => {
            eprintln!("❌ failed to fetch weather: {}", e);
        }
    }
}
