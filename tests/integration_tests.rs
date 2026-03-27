use std::process::Command;

#[cfg(test)]
mod integration_tests {
    use super::*;
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
