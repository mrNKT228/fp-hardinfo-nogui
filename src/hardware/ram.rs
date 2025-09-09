use std::process::Command;

use crate::utils::size::{byte_size_to_string, gigabytes};

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
        Ok(size) => calc_ram(size),
        Err(error) => format!("Ошибка получения информации: {}", error),
      }
    }
    Err(error) => {
      format!("Ошибка получения информации: {}", error)
    }
  }
}

fn calc_ram(bytes: usize) -> String {
  let mut rounded: Option<&str> = None;
  if bytes <= gigabytes(32) && bytes >= gigabytes(27) {
    rounded = Some("32 GB")
  }
  if bytes <= gigabytes(16) && bytes >= gigabytes(14) {
    rounded = Some("16 GB")
  }
  if bytes <= gigabytes(12) && bytes >= gigabytes(9) {
    rounded = Some("12 GB")
  }
  if bytes <= gigabytes(8) && bytes >= gigabytes(6) {
    rounded = Some("8 GB")
  }
  if bytes <= gigabytes(6) && bytes > gigabytes(4) {
    rounded = Some("6 GB")
  }
  if bytes <= gigabytes(4) && bytes > gigabytes(2) {
    rounded = Some("4 GB")
  }
  if bytes <= gigabytes(2) && bytes > gigabytes(1) {
    rounded = Some("2 GB")
  }
  if bytes <= gigabytes(1) {
    rounded = Some("1 GB")
  }

  if let Some(rounded) = rounded {
    format!("{} ({})", byte_size_to_string(bytes), rounded)
  } else {
    format!("{}", byte_size_to_string(bytes))
  }
}
