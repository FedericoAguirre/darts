use clap::{value_parser, Arg, Command};
use std::path::Path;

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
                .required_unless_present("csv")
                .requires("y")
                .conflicts_with("csv")
                .help("Dart's X coordinate."),
        )
        .arg(
            Arg::new("y")
                .value_parser(value_parser!(f32))
                .allow_hyphen_values(true)
                .long("y_coord")
                .short('y')
                .required_unless_present("csv")
                .requires("x")
                .conflicts_with("csv")
                .help("Dart's Y coordinate."),
        )
        .arg(
            Arg::new("csv")
                .long("csv")
                .short('c')
                .required_unless_present_any(["x", "y"])
                .conflicts_with_all(["x", "y"])
                .help("Path to input CSV file with columns x and y."),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .requires("csv")
                .help("Path to output CSV file (only used with --csv)."),
        )
        .try_get_matches()
        .unwrap_or_else(|e| e.exit());

    if matches.contains_id("csv") {
        // CSV mode
        let input_csv = matches.get_one::<String>("csv").unwrap();
        let output_csv = matches.get_one::<String>("output");
        darts_lib::process_csv(input_csv, output_csv.map(|s| s.as_str()))?;
        let actual_output = match output_csv {
            Some(path) => path.clone(),
            None => {
                let path = Path::new(input_csv);
                let stem = path.file_stem().unwrap_or_default();
                let parent = path.parent().unwrap_or_else(|| Path::new(""));
                parent
                    .join(stem)
                    .with_extension("score.csv")
                    .to_string_lossy()
                    .into_owned()
            }
        };
        println!(
            "CSV processing completed. Output written to {}",
            actual_output
        );
    } else {
        // Single coordinate mode
        let x = matches
            .get_one::<f32>("x")
            .copied()
            .ok_or("Missing x coordinate")?;
        let y = matches
            .get_one::<f32>("y")
            .copied()
            .ok_or("Missing y coordinate")?;
        let score = darts_lib::calculate_score(x, y)
            .ok_or("Invalid coordinates: x and y must be finite numbers")?;
        println!("The score for ({x}, {y}) is: {score}");
    }
    Ok(())
}
