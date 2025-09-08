use std::{
  fs::{self, File},
  io::Write,
  path::{Path, PathBuf},
};

use crate::hardware::{
  cpu::get_cpu_info, gpu::get_gpu_info, host::get_host_info, ram::get_ram_info,
  rom::get_rom_info,
};

pub fn save(path: Option<String>, date: String) {
  let cpu = get_cpu_info();
  let gpu = get_gpu_info();
  let ram = get_ram_info();
  let rom = get_rom_info();
  let host = get_host_info();

  let cpu = cpu
    .split('\n')
    .filter(|line| line.len() > 0)
    .map(|line| format!("<li>{}</li>", line))
    .collect::<Vec<String>>()
    .join("");

  let gpu = gpu
    .split('\n')
    .filter(|line| line.len() > 0)
    .map(|line| format!("<li>{}</li>", line))
    .collect::<Vec<String>>()
    .join("");

  let rom = rom
    .split('\n')
    .filter(|line| line.len() > 0)
    .map(|line| format!("<li>{}</li>", line))
    .collect::<Vec<String>>()
    .join("");

  let html = format!(
    "<!DOCTYPE html>
<html>
  <head>
    <title>Отчёт от {date}</title>
  </head>
  <body>
    <h1>Отчёт об аппаратном обеспечении ПК</h1>
    <div class=\"group\">
      <h2>Процессор:</h2>
      <ul>
        {cpu}
      </ul>
    </div>
    <div class=\"group\">
      <h2>Видеокарта:</h2>
      <ul>
        {gpu}
      </ul>
    </div>
    <div class=\"group\">
      <h2>ОЗУ:</h2>
      <ul>
        <li>{ram}</li>
      </ul>
    </div>
    <div class=\"group\">
      <h2>ПЗУ:</h2>
      <ul>
        {rom}
      </ul>
    </div>
    <div class=\"group\">
      <h2>Система:</h2>
      <ul>
        <li>{host}</li>
      </ul>
    </div>
    <div class=\"meta-info\">
      Отчёт сгенерирован {date}
    </div>
  </body>
</html>"
  );

  if let Some(path) = path {
    let path = make_file_name(path, host, date);

    if path.exists() {
      if let Err(error) = fs::remove_file(&path) {
        println!("Ошибка сохранения файла: {}", error);
        return;
      }
    }

    let mut file = match File::create(&path) {
      Ok(file) => file,
      Err(error) => {
        println!("Ошибка сохранения файла: {}", error);
        return;
      }
    };

    if let Err(error) = file.write_all(html.as_bytes()) {
      println!("Ошибка сохранения файла: {}", error);
      return;
    }

    println!(
      "{}",
      path.to_str().unwrap_or("Ошибка преобразования имени файла")
    );
  } else {
    println!("Ошибка сохранения отчёта: не указан путь к папке")
  }
}

fn make_file_name(base: String, host: String, date: String) -> PathBuf {
  let base = PathBuf::from(&base);
  if base.is_file() {
    return base;
  }

  let mut name_candidate = build_path(
    &base,
    format!("{}_{}.html", escape_string(&host), escape_string(&date)),
  );

  let mut i = 0;
  while Path::new(&name_candidate).exists() {
    name_candidate = build_path(
      &base,
      format!(
        "{}_{}_{}.html",
        escape_string(&host),
        escape_string(&date),
        i
      ),
    );
    i = i + 1;
  }

  name_candidate
}

fn escape_string(host: &String) -> String {
  host
    .chars()
    .map(|char| {
      if char == ' ' {
        return '+';
      }
      if !char.is_alphanumeric() {
        return '-';
      }
      return char;
    })
    .collect::<String>()
}

fn build_path(base: &PathBuf, name: String) -> PathBuf {
  let mut path = base.clone();
  path.push(name);
  path
}
