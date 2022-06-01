use getopts;
use getopts::Options;

const ERRNO_INVALID_ARGUMENT: i32 = 22;

pub struct Args {
  pub memory_threshold: f32,
  pub command: String,
}

pub fn cmd_args(args: Vec<String>) -> Args {
  let mut opts = Options::new();

  opts.optflag("h", "help", "print this help menu");
  opts.optopt("m", "memory", "memory threshold", "THRESHOLD");
  opts.optopt("c", "command", "shell command", "COMMAND");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(f) => {
      panic!("{}", f.to_string())
    }
  };

  if matches.opt_present("h") {
    print_usage(opts, &args[0]);
    std::process::exit(0)
  }

  let memory_threshold = match matches.opt_get::<f32>("m").unwrap() {
    Some(memory_threshold) => memory_threshold,
    None => {
      eprintln!("{}", "memory threshold wasn't set");
      print_usage(opts, &args[0]);
      std::process::exit(ERRNO_INVALID_ARGUMENT)
    }
  };

  let command = match matches.opt_str("c") {
    Some(command) => command,
    None => {
      eprintln!("Command wasn't set");
      print_usage(opts, &args[0]);
      std::process::exit(ERRNO_INVALID_ARGUMENT)
    }
  };

  Args {
    memory_threshold,
    command,
  }
}

fn print_usage(opts: Options, program_name: &str) {
  let brief = format!("Usage: {} [options]", program_name);
  println!("{}", opts.usage(&brief));
}
