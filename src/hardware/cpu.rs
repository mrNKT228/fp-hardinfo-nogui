use std::process::Command;

pub fn get_cpu_info() -> Result<String, String> {
  match Command::new("neofetch").arg("cpu").output() {
    Ok(output) => Ok(
      String::from_utf8_lossy(&output.stdout)
        .replace("cpu:", "")
        .split('\n')
        .map(|name| name.trim())
        .collect::<Vec<&str>>()
        .join("\n"),
    ),
    Err(error) => Err(format!("Ошибка получения информации: {}", error)),
  }
}
