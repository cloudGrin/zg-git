use std::env;
use cmd_lib::spawn;
extern crate proc_macro;

/* 执行git */
fn main() {
    let args = env::args().collect::<Vec<String>>()[1..].join(" ");
    let mut proc = spawn!(git $args).unwrap();
    proc.wait().unwrap();
}
