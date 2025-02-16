#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::env;

    #[test]
    fn test_help_flag() {
        let output = Command::cargo_bin("pf")
            .unwrap()
            .arg("--help")
            .output()
            .expect("Failed to execute pf");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Usage"));
    }

    #[test]
    fn test_version_flag() {
        let output = Command::cargo_bin("pf")
            .unwrap()
            .arg("--version")
            .output()
            .expect("Failed to execute pf");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains(format!("pf {}", env!("CARGO_PKG_VERSION")).as_str()));
    }
}
