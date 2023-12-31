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
        10
    } else if magnitude >= 1.0 && magnitude < 5.0 {
        5
    } else if magnitude >= 5.0 && magnitude < 10.0 {
        1
    } else {
        0
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    // Unit tests
    #[test]
    fn test_score_10() {
        assert_eq!(calculate_score(0.0, 0.0), 10);
        assert_eq!(calculate_score(0.7, 0.7), 10);
        assert_eq!(calculate_score(-0.7, 0.7), 10);
        assert_eq!(calculate_score(0.7, -0.7), 10);
        assert_eq!(calculate_score(-0.7, -0.7), 10);
    }

    #[test]
    fn test_score_5() {
        assert_eq!(calculate_score(1.4, 1.4), 5);
        assert_eq!(calculate_score(-1.4, 1.4), 5);
        assert_eq!(calculate_score(1.4, -1.4), 5);
        assert_eq!(calculate_score(-1.4, -1.4), 5);
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

    // Integration tests
    #[test]
    fn test_main_function_with_valid_input() {
        // (0, 0) test, score 10
        let output = Command::new("target/debug/darts")
            .args(&["--x_coord", "0.0", "--y_coord", "0.0"])
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());

        let stdout_str = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout_str.contains("The score for (0, 0) is: 10"));

        // (0.7, 0.7) test, score 10)
        let output = Command::new("target/debug/darts")
            .args(&["--x_coord", "0.7", "--y_coord", "0.7"])
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());

        let stdout_str = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout_str.contains("The score for (0.7, 0.7) is: 10"));

        // (-1.4, 1.4) test, score 5)
        let output = Command::new("target/debug/darts")
            .args(&["--x_coord", "-1.4", "--y_coord", "1.4"])
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());

        let stdout_str = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout_str.contains("The score for (-1.4, 1.4) is: 5"));

        // (4, -4) test, score 1)
        let output = Command::new("target/debug/darts")
            .args(&["--x_coord", "4.0", "--y_coord", "-4.0"])
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());

        let stdout_str = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout_str.contains("The score for (4, -4) is: 1"));

        // (-11, -11) test, score 0)
        let output = Command::new("target/debug/darts")
            .args(&["--x_coord", "-11.0", "--y_coord", "-11.0"])
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());

        let stdout_str = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout_str.contains("The score for (-11, -11) is: 0"));
    }
}
