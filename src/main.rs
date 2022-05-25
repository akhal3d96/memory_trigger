mod cmd;
mod memory;

use cmd::*;
use memory::check_memory_and_excute;
use std::env;
use std::{thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = cmd_args(args);

    let internal_timer = true;
    let exit_after_excute = true;
    let sleep_duration = 1000;

    if internal_timer {
        loop {
            match check_memory_and_excute(args.memory_threshold, &args.command) {
                Some(code) => {
                    if exit_after_excute {
                        std::process::exit(code);
                    }
                }
                None => thread::sleep(Duration::from_millis(sleep_duration)),
            }
        }
    } else {
        check_memory_and_excute(args.memory_threshold, &args.command);
    }
}
