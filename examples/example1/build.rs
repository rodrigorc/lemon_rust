#![feature(fs_time)]

use std::process::Command;

fn main() {
    for f in  std::fs::read_dir("src").unwrap() {
        let name = f.unwrap().path();
        if name.extension() == Some("y".as_ref()) {
            let name_rs = name.with_extension("rs");

            let meta_y = std::fs::metadata(&name).unwrap();
            let need_to_build = match std::fs::metadata(&name_rs) {
                Ok(x) => x.modified() < meta_y.modified(),
                Err(_) => true,
            };
            if need_to_build {
                Command::new("../../lemon_rust").arg(&name)
                    .status().unwrap();
            }
        }
    }
}
