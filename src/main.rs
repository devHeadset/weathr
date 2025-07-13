mod setup;

use reqwest::blocking::Client;
use serde::Deserialize;
use std::process;

#[derive(Deserialize)]
struct WeatherResponse {
    main: TempInfo,
    weather: Vec<WeatherInfo>,
    name: String,
}

#[derive(Deserialize)]
struct TempInfo {
    temp: f64,
}

#[derive(Deserialize)]
struct WeatherInfo {
    description: String,
}

fn get_ip_city() -> Option<String> {
    let client = Client::new();
    let res = client
        .get("https://ipinfo.io/json")
        .send()
        .ok()?
        .json::<serde_json::Value>()
        .ok()?;
    res.get("city")?.as_str().map(|s| s.to_string())
}

fn main() {
    let config = setup::load_config().unwrap_or_else(|| {
        println!("⚙️ running first-time setup...");
        setup::run_setup().expect("setup failed");
        setup::load_config().expect("couldn't load config after setup")
    });

    let city = if config.city.is_empty() {
        println!("🌐 detecting location from IP...");
        get_ip_city().unwrap_or_else(|| {
            eprintln!("❌ couldn't detect city from IP");
            process::exit(1);
        })
    } else {
        config.city.clone()
    };

    let unit_param = match config.unit.as_str() {
        "f" => "imperial",
        _ => "metric", // default to celsius
    };

    let client = Client::new();
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city}&appid={}&units={}",
        config.api_key, unit_param
    );

    let res = client.get(&url).send();

    match res {
        Ok(resp) => {
            if resp.status().is_success() {
                let data: WeatherResponse = resp.json().unwrap();
                println!("\n🌍 city: {}", data.name);
                println!("🌡️ temp: {:.2}°{}", data.main.temp, if config.unit == "f" { "F" } else { "C" });
                println!("🌥️ weather: {}", data.weather[0].description);
            } else {
                eprintln!("❌ API error: {}", resp.status());
            }
        }
        Err(e) => {
            eprintln!("❌ request failed: {e}");
        }
    }
}
