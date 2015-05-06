use std::process::Command;

fn main() {
    Command::new("make").arg("-flemon.mk").status().unwrap();
}
