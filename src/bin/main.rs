use cmd_lib::*;
use std::env;

/* 执行git */
fn main() -> CmdResult {
    // DEBUG
    // cmd_lib::set_debug(true);
    // init_builtin_logger();
    let mut args = env::args().collect::<Vec<String>>();

    if args[0].eq("merge")
        & !args.join(" ").contains("--ff")
        & !args.join(" ").contains("--ff-only")
        & !args.join(" ").contains("--no-ff")
    {
        println!("merge 默认使用 --no-ff");
        args.push(String::from("--no-ff"));
    }

    let mut proc = spawn!(git $[args]).unwrap();
    proc.wait().unwrap();
    Ok(())
}
