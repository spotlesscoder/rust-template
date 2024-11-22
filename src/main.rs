use rust_template::restaurant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome at {}", restaurant::display_name());
    restaurant::foh::process_visit();
    Ok(())
}
