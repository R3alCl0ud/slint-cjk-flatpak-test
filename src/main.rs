use std::{
    error::Error,
    fs::{self},
    io,
    path::PathBuf,
};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    println!("Listing contents of /usr/share/fonts");
    let mut buf = PathBuf::from("/usr/share/fonts");
    if buf.exists() {
        let mut entries = fs::read_dir(buf.clone())?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort();

        for entry in entries.iter() {
            println!("{entry:?}");
        }
    }
    println!("Listing contents of /etc/fonts");
    buf.push("/etc/fonts");
    if buf.exists() {
        let mut entries = fs::read_dir(buf)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort();
        for entry in entries.iter() {
            println!("{entry:?}");
        }
    }
    let app = App::new()?;

    app.run().unwrap();
    Ok(())
}
