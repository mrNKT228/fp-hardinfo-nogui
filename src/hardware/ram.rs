use std::process::Command;

use crate::utils::size::byte_size_to_string;

pub fn get_ram_info() -> String {
  match Command::new("free").arg("-b").output() {
    Ok(output) => {
      let size = String::from_utf8_lossy(&output.stdout)
        .split('\n')
        .skip(1)
        .map(|line| {
          line
            .split(' ')
            .skip(1)
            .filter(|item| item.len() > 0)
            .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>()
        .get(0)
        .unwrap_or(&Vec::from(["Ошибка получения информации"]))
        .get(0)
        .unwrap_or(&"Ошибка получения информации")
        .to_string()
        .parse::<usize>();
      match size {
        Ok(size) => byte_size_to_string(size),
        Err(error) => format!("Ошибка получения информации: {}", error),
      }
    }
    Err(error) => {
      format!("Ошибка получения информации: {}", error)
    }
  }
}
