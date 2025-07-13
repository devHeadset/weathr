use dotenv::dotenv;
use std::env;

pub fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    let key = env::var("API_KEY")?;
    Ok(key)
}
