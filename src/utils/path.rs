pub fn get_home_path() -> Result<String, std::io::Error> {
  match std::process::Command::new("echo").arg("$HOME").output() {
    Ok(output) => {
      Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    Err(error) => Err(error),
  }
}
