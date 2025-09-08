use std::process::Command;

pub fn get_cpu_info() -> String {
  match Command::new("neofetch").arg("cpu").output() {
    Ok(output) => String::from_utf8_lossy(&output.stdout)
      .replace("cpu:", "")
      .split('\n')
      .map(|name| name.trim())
      .collect::<Vec<&str>>()
      .join("\n"),
    Err(error) => {
      format!("Ошибка получения информации: {}", error)
    }
  }
}
