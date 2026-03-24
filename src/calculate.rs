use num_complex::Complex64;

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
