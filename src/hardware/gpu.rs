pub fn get_gpu_info() -> String {
  match std::process::Command::new("neofetch").arg("gpu").output() {
    Ok(output) => String::from_utf8_lossy(&output.stdout)
      .replace("gpu:", "")
      .split('\n')
      .map(|name| name.trim())
      .collect::<Vec<&str>>()
      .join("\n"),
    Err(error) => {
      format!("Ошибка получения информации: {}", error)
    }
  }
}
