pub fn get_date() -> Result<String, std::io::Error> {
    match std::process::Command::new("date").output() {
        Ok(output) => Ok(String::from_utf8_lossy(&output.stdout).trim().to_string()),
        Err(error) => Err(error),
    }
}
