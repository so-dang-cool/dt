use rail::prompt::RailPrompt;
use colored::*;

pub const RAIL_VERSION: &str = std::env!("CARGO_PKG_VERSION");

fn main() {
    println!("{} {}", "rail".red(), RAIL_VERSION.red());

    RailPrompt::default().run()
}
