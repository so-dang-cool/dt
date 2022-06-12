use rail::prompt::RailPrompt;

pub const RAIL_VERSION: &str = std::env!("CARGO_PKG_VERSION");

fn main() {
    println!("rail {}", RAIL_VERSION);

    RailPrompt::default().run()
}
