use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello, world!");
    let app = App::new()?;

    app.run().unwrap();
    Ok(())
}
