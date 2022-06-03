use rail::prompt::RailPrompt;
use rail::RailState;

pub const RAIL_VERSION: &str = std::env!("CARGO_PKG_VERSION");

fn main() {
    println!("rail {}", RAIL_VERSION);

    RailPrompt::default().fold(RailState::new(), rail::operate);
}
