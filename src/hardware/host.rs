use std::process::Command;

pub fn get_host_info() -> Result<String, String> {
  match Command::new("neofetch").arg("model").output() {
    Ok(output) => Ok(
      String::from_utf8_lossy(&output.stdout)
        .replace("model:", "")
        .trim()
        .to_string(),
    ),
    Err(error) => Err(format!("Ошибка получения информации: {}", error)),
  }
}
