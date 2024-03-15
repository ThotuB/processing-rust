use p5::Color;

extern crate processing as p5;

mod examples;

// struct State {
//     frame: i64,
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // examples::trees::app().run()?;
    examples::noise::app().run()?;
    Ok(())
}
