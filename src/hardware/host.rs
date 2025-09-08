use std::process::Command;

pub fn get_host_info() -> String {
  match Command::new("neofetch").arg("model").output() {
    Ok(output) => String::from_utf8_lossy(&output.stdout)
      .replace("model:", "")
      .trim()
      .to_string(),
    Err(error) => {
      format!("Ошибка получения информации: {}", error)
    }
  }
}
