use crate::{save::save, utils::date::get_date};

mod hardware;
mod save;
mod utils;

fn main() {
  let date = get_date().unwrap_or("НЕИЗВЕСТНО".to_string());
  let path = get_save_path();
  save(path, date);
}

fn get_save_path() -> Option<String> {
  let args: Vec<String> = std::env::args().skip(1).collect();
  args.get(0).cloned()
}
