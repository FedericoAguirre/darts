mod calculate;

use clap::{value_parser, Arg, Command};

fn main() {
    let matches = Command::new("Darts score calculator.")
        .version("0.1.0")
        .author("Federico Aguirre, federico.aguirre.cardiel@gmail.com")
        .about("Calculates the score for a dart throwing game.")
        .arg(
            Arg::new("x")
                .value_parser(value_parser!(f64))
                .allow_hyphen_values(true)
                .long("x_coord")
                .short('x')
                .required(true)
                .help("Dart's X coordinate."),
        )
        .arg(
            Arg::new("y")
                .value_parser(value_parser!(f64))
                .allow_hyphen_values(true)
                .long("y_coord")
                .short('y')
                .required(true)
                .help("Dart's Y coordinate."),
        )
        .try_get_matches()
        .unwrap_or_else(|e| e.exit());


    let x = *matches.get_one::<f64>("x").expect("`x` is required");
    let y = *matches.get_one::<f64>("y").expect("`y` is required");

    match calculate::calculate_score(x, y) {
        Some(score) => println!("The score for ({}, {}) is: {}", x, y, score),
        None => {
            eprintln!("Error: Invalid coordinates (NaN or infinity)");
            std::process::exit(1);
        }
    }
}
