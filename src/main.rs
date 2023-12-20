use clap::{value_parser, Arg, Command};
use num_complex::Complex64;

fn main() {
    // Check cmd args https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
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

    let score = calculate_score(x, y);
    println!("The score for ({}, {}) is: {}", x, y, score);
}

pub fn calculate_score(x: f64, y: f64) -> i8 {
    let point = Complex64::new(x, y);
    let magnitude: f64 = point.norm_sqr().sqrt();
    if magnitude < 1.0 {
        return 10;
    } else if magnitude >= 1.0 && magnitude < 5.0 {
        return 5;
    } else if magnitude >= 5.0 && magnitude < 10.0 {
        return 1;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_score_10() {
        assert_eq!(calculate_score(0.0, 0.0), 10);
        assert_eq!(calculate_score(0.7, 0.7), 10);
        assert_eq!(calculate_score(-0.7, 0.7), 10);
        assert_eq!(calculate_score(0.7, -0.7), 10);
        assert_eq!(calculate_score(-0.7, 0.7), 10);
    }

    #[test]
    fn test_score_5() {
        assert_eq!(calculate_score(1.4, 1.4), 5);
        assert_eq!(calculate_score(-1.4, 1.4), 5);
        assert_eq!(calculate_score(1.4, -1.4), 5);
        assert_eq!(calculate_score(-1.4, 1.4), 5);
    }

    #[test]
    fn test_score_1() {
        assert_eq!(calculate_score(4.0, 4.0), 1);
        assert_eq!(calculate_score(-4.0, 4.0), 1);
        assert_eq!(calculate_score(4.0, -4.0), 1);
        assert_eq!(calculate_score(-4.0, -4.0), 1);
    }

    #[test]
    fn test_score_0() {
        assert_eq!(calculate_score(11.0, 11.0), 0);
        assert_eq!(calculate_score(-11.0, 11.0), 0);
        assert_eq!(calculate_score(11.0, -11.0), 0);
        assert_eq!(calculate_score(-11.0, -11.0), 0);
    }

    #[test]
    fn test_main_function_with_valid_input() {
        let output = Command::new("target/debug/darts")
            .args(&["--x_coord", "2.0", "--y_coord", "-2.0"])
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());

        let stdout_str = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout_str.contains("The score for (2, -2) is: 5"));
    }

}
