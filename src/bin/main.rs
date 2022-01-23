use cmd_lib::*;
use std::env;

/* 执行git */
fn main() -> CmdResult {
    // DEBUG
    // cmd_lib::set_debug(true);
    init_builtin_logger();
    let args = env::args().collect::<Vec<String>>();
    let args_opt = args.get(1..);
    if let Some(args) = args_opt {
        let mut args = args.to_vec();

        if args.is_empty() {
            args.push(String::from("--help"));
        }

        if args[0].eq("merge")
            & !args.join(" ").contains("--ff")
            & !args.join(" ").contains("--ff-only")
            & !args.join(" ").contains("--no-ff")
        {
            println!("merge 默认使用 --no-ff");
            args.push(String::from("--no-ff"));
        }

        match run_cmd! {
            git $[args];
        } {
            _ => (),
        }
    }
    Ok(())
}
