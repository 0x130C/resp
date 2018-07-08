#[macro_use]
extern crate derive_more;
extern crate hotwatch;
extern crate sass_rs;
use hotwatch::{Hotwatch, Event};

use std::io::Write;
use std::fs::File;


#[derive(Debug, From)]
enum SassOutputError {
    Sass(String),
    Io(std::io::Error)
}

fn sass_options() -> sass_rs::Options {
    let mut options = sass_rs::Options::default();
    options.output_style = sass_rs::OutputStyle::Expanded;
    options
}

fn sass_compile() -> Result<(), SassOutputError> {
    let compiled = sass_rs::compile_file("../client/assets/scss/main.scss", sass_options())?;
    let mut file = File::create("../static/bundle.css")?;
    file.write_all(compiled.as_bytes())?;
    Ok(())
}

fn main() {

    let mut hotwatch = Hotwatch::new()
        .expect("Hotwatch failed to initialize.");
    hotwatch.watch("../client/assets/scss/", move |e: Event| {
        match e {
            Event::Create(_) | Event::Write(_) | Event::Remove(_) => {
                println!("⏳Updating CSS...");

                match sass_compile() {
                    Ok(_) => println!("✔️ Updated CSS."),
                    Err(e) => match e {
                        SassOutputError::Sass(e) =>
                            println!("❌ Failed to compile SCSS: {:?}", e),
                        SassOutputError::Io(e) =>
                            println!("❌ Failed to write CSS output to file: {:?}", e)
                    }
                }
            },
            _ => {}
        }
    }).expect("Failed to hotly watch.");
    println!("Time to build!");
    loop {}
}


