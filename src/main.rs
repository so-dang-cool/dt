use rail::prompt::RailPrompt;
use rail:: RAIL_VERSION;

fn main() {
    println!("rail {}", RAIL_VERSION);

    RailPrompt::default().run()
}
