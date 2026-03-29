use clap::{value_parser, Arg, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Darts score calculator.")
        .version("0.1.0")
        .author("Federico Aguirre, federico.aguirre.cardiel@gmail.com")
        .about("Calculates the score for a dart throwing game.")
        .arg(
            Arg::new("x")
                .value_parser(value_parser!(f32))
                .allow_hyphen_values(true)
                .long("x_coord")
                .short('x')
                .required(true)
                .help("Dart's X coordinate."),
        )
        .arg(
            Arg::new("y")
                .value_parser(value_parser!(f32))
                .allow_hyphen_values(true)
                .long("y_coord")
                .short('y')
                .required(true)
                .help("Dart's Y coordinate."),
        )
        .try_get_matches()
        .unwrap_or_else(|e| e.exit());

    let x = matches.get_one::<f32>("x").copied().unwrap();
    let y = matches.get_one::<f32>("y").copied().unwrap();

    let score = darts_lib::calculate_score(x, y)
        .ok_or("Invalid coordinates: x and y must be finite numbers")?;

    println!("The score for ({x}, {y}) is: {score}");
    Ok(())
}
