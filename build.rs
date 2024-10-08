use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = slint_build::CompilerConfiguration::new().with_style("cosmic".into());
    slint_build::compile_with_config("ui/appwindow.slint", config)?;

    Ok(())
}
