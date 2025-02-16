#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_compile_fail_with_no_features() {
        let output = Command::new("cargo")
            .arg("build")
            .arg("--no-default-features")
            .output()
            .expect("Failed to execute cargo build");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Error: At least one feature must be enabled."));
    }

    #[test]
    fn test_compile_success_with_features() {
        let output = Command::new("cargo")
            .arg("build")
            .arg("--features")
            .arg("mp4")
            .output()
            .expect("Failed to execute cargo build");

        assert!(output.status.success());
    }
}
