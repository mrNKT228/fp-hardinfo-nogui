use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]

struct LsblkOutput {
  blockdevices: Vec<BlockDevice>,
}
#[derive(Serialize, Deserialize)]
struct BlockDevice {
  name: String,
  rm: bool,
  ro: bool,
  size: usize,
}

pub fn get_rom_info() -> String {
  match std::process::Command::new("lsblk")
    .arg("-J")
    .arg("-b")
    .output()
  {
    Ok(output) => {
      let output = String::from_utf8_lossy(&output.stdout);
      match serde_json::from_str::<LsblkOutput>(&output) {
        Ok(parsed) => parsed
          .blockdevices
          .iter()
          .map(|device| {
            format!(
              "Устройство \"{}\", размер: {}",
              device.name,
              crate::utils::size::byte_size_to_string(device.size)
            )
          })
          .collect::<Vec<String>>()
          .join("\n"),
        Err(error) => format!("Ошибка получения информации: {}", error),
      }
    }
    Err(error) => {
      format!("Ошибка получения информации: {}", error)
    }
  }
}
// lsblk -o NAME,SIZE,MOUNTPOINT
