use crate::{
  save::save,
  utils::{date::get_date, path::get_home_path},
};

mod hardware;
mod save;
mod utils;

fn main() {
  let date = get_date().unwrap_or("НЕИЗВЕСТНО".to_string());
  let path = get_home_path().unwrap_or("/home/mrnkt".to_string());
  save(format!("{}/out.html", path).to_string(), date);
}
