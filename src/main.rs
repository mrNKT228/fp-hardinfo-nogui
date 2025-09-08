use crate::{save::save, utils::date::get_date};

mod hardware;
mod save;
mod utils;

fn main() {
    let date = get_date().unwrap_or("НЕИЗВЕСТНО".to_string());
    save(
        "/home/mrnkt/DEV/hardinfo/fp-hardinfo-nogui/out.html".to_string(),
        date,
    );
}
