pub fn calculate_score(x: f64, y: f64) -> Option<i8> {
    if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
        return None;
    }

    let magnitude: f64 = x.hypot(y);
    if magnitude < 1.0 {
        Some(10)
    } else if magnitude < 5.0 {
        Some(5)
    } else if magnitude < 10.0 {
        Some(1)
    } else {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_score_10() {
        assert_eq!(calculate_score(0.0, 0.0), Some(10));
        assert_eq!(calculate_score(0.7, 0.7), Some(10));
        assert_eq!(calculate_score(-0.7, 0.7), Some(10));
        assert_eq!(calculate_score(0.7, -0.7), Some(10));
        assert_eq!(calculate_score(-0.7, -0.7), Some(10));
    }

    #[test]
    fn test_score_5() {
        assert_eq!(calculate_score(1.4, 1.4), Some(5));
        assert_eq!(calculate_score(-1.4, 1.4), Some(5));
        assert_eq!(calculate_score(1.4, -1.4), Some(5));
        assert_eq!(calculate_score(-1.4, -1.4), Some(5));
    }

    #[test]
    fn test_score_1() {
        assert_eq!(calculate_score(4.0, 4.0), Some(1));
        assert_eq!(calculate_score(-4.0, 4.0), Some(1));
        assert_eq!(calculate_score(4.0, -4.0), Some(1));
        assert_eq!(calculate_score(-4.0, -4.0), Some(1));
    }

    #[test]
    fn test_score_0() {
        assert_eq!(calculate_score(11.0, 11.0), Some(0));
        assert_eq!(calculate_score(-11.0, 11.0), Some(0));
        assert_eq!(calculate_score(11.0, -11.0), Some(0));
        assert_eq!(calculate_score(-11.0, -11.0), Some(0));
    }

    #[test]
    fn test_invalid_nan() {
        assert_eq!(calculate_score(f64::NAN, 0.0), None);
        assert_eq!(calculate_score(0.0, f64::NAN), None);
        assert_eq!(calculate_score(f64::NAN, f64::NAN), None);
    }

    #[test]
    fn test_invalid_infinity() {
        assert_eq!(calculate_score(f64::INFINITY, 0.0), None);
        assert_eq!(calculate_score(0.0, f64::NEG_INFINITY), None);
        assert_eq!(calculate_score(f64::INFINITY, f64::NEG_INFINITY), None);
    }

    #[test]
    fn test_main_function_with_valid_input() {
        let output = Command::new("target/debug/darts")
            .args(&["--x_coord", "0.0", "--y_coord", "0.0"])
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());

        let stdout_str = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout_str.contains("The score for (0, 0) is: 10"));

        let output = Command::new("target/debug/darts")
            .args(&["--x_coord", "0.7", "--y_coord", "0.7"])
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());

        let stdout_str = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout_str.contains("The score for (0.7, 0.7) is: 10"));

        let output = Command::new("target/debug/darts")
            .args(&["--x_coord", "-1.4", "--y_coord", "1.4"])
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());

        let stdout_str = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout_str.contains("The score for (-1.4, 1.4) is: 5"));

        let output = Command::new("target/debug/darts")
            .args(&["--x_coord", "4.0", "--y_coord", "-4.0"])
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());

        let stdout_str = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout_str.contains("The score for (4, -4) is: 1"));

        let output = Command::new("target/debug/darts")
            .args(&["--x_coord", "-11.0", "--y_coord", "-11.0"])
            .output()
            .expect("Failed to execute process");

        assert!(output.status.success());

        let stdout_str = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout_str.contains("The score for (-11, -11) is: 0"));
    }
}
