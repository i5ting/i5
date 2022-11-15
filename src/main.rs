use clap::{arg, Command};

mod registry;

pub use self::registry::*;

fn main() {
    let matches = Command::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(arg!(--two <VALUE>).required(false))
        .arg(arg!(--one <VALUE>).required(true))
        .get_matches();

    println!("dsd {:?}", matches);

    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );

    if let Some(name) = matches.get_one::<String>("two") {
        println!("Value for name: {}", name);
    }
}