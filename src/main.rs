extern crate chase;
extern crate clap;

use clap::{App, Arg};

use std::error::Error;
use std::process::exit;

use chase::*;

const FILE_KEY: &'static str = "f";
const LINE_KEY: &'static str = "l";

fn main() {
    match inner_main() {
        Ok(_) => exit(0),
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    }
}

fn inner_main() -> Result<(), Box<Error>> {
    let version = version();
    let app = App::new("chase")
        .version(version.as_str())
        .author("Lloyd (github.com/lloydmeta)")
        .about("Chases a file through thick and thin.")
        .arg(
            Arg::with_name(FILE_KEY)
                .takes_value(true)
                .number_of_values(1)
                .required(true)
                .help("The file you want to chase"),
        )
        .arg(
            Arg::with_name(LINE_KEY)
                .long("line")
                .short("L")
                .takes_value(true)
                .number_of_values(1)
                .validator(|s| s.parse::<usize>().map(|_| ()).map_err(|e| format!("{}", e)))
                .required(false)
                .default_value("0")
                .help("The line you want to start chasing your file from"),
        );

    // in case we need to print help
    let mut app_clone = app.clone();
    let matches = app.get_matches();
    match (matches.value_of(FILE_KEY), matches.value_of(LINE_KEY)) {
        (Some(file), maybe_line) => {
            let mut chaser = Chaser::new(&file);
            if let Some(start_line) = maybe_line {
                chaser.set_line(Line(start_line.parse()?))
            }
            chaser.run(|l, _, _| {
                println!("{}", l);
                Ok(Control::Continue)
            })?;
            Ok(())
        }
        _ => Ok(app_clone.print_help()?),
    }
}

/// Return the current crate version
fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}
