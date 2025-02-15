use std::env;

use commands::backup::backup;
use commands::purge::purge;
use commands::restore::restore;
use commands::synctogdrive::synctogdrive;
use utils::usage;

mod commands;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage(&args[0]);
    }

    let command = &args[1];
    match command.as_str() {
        "backup" => backup(&args[2..]),
        "restore" => restore(&args[2..]),
        "purge" => purge(&args[2..]),
        "synctogdrive" => synctogdrive(&args[2..]),
        _ => {
            eprintln!("Unknown command: {}", command);
            usage(&args[0]);
        }
    }
}
