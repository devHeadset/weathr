use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use dirs::config_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub unit: String,
    pub city: String,
    pub api_key: String,
}

pub fn run_setup() -> io::Result<()> {
    let mut unit = String::new();
    print!("🌡️ use Celsius or Fahrenheit? (c/f): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut unit)?;
    let unit = unit.trim().to_lowercase();

    let mut city = String::new();
    print!("🏙️ default city (leave blank to use IP location): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut city)?;
    let city = city.trim().to_string();

    let mut api_key = String::new();
    print!("🔑 your OpenWeatherMap API key: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut api_key)?;
    let api_key = api_key.trim().to_string();

    let config = Config { unit, city, api_key };

    let config_path = get_config_path();
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let toml = toml::to_string_pretty(&config).unwrap();
    fs::write(config_path, toml)?;

    println!("\n✅ config saved to ~/.config/weathr/config.toml");
    Ok(())
}

pub fn get_config_path() -> PathBuf {
    config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("weathr")
        .join("config.toml")
}

pub fn load_config() -> Option<Config> {
    let path = get_config_path();
    let data = fs::read_to_string(path).ok()?;
    toml::from_str(&data).ok()
}
