use clap::Parser;

#[derive(Parser)]
#[command(name = "weathr")]
#[command(about = "a simple cli weather app")]
pub struct Args {
    /// the city to fetch weather for
    pub city: String,
}
