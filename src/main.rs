use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;

const USAGE: &str = "./memory_trigger <MEMORY USAGE THRESHOLD %XX> <COMMAND>";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Insufficient arguments.");
        eprintln!("{}", USAGE);
        std::process::exit(22)
    }

    let memory_threshold: f32 = (&args[1]).parse().expect("cannot parse memory threshold");
    let action = &args[2];

    let curr_memory = memory_precentage();
    if curr_memory > memory_threshold {
        let out = Command::new("sh")
            .arg("-c")
            .arg(action)
            .output()
            .expect("failed to excute process");

        io::stdout().write_all(&out.stdout).unwrap();
        io::stderr().write_all(&out.stderr).unwrap();

        match out.status.code() {
            Some(code) => std::process::exit(code),
            None => panic!("exited with a signal"),
        }
    }
}

fn memory_precentage() -> f32 {
    let proc_memory_file = File::open("/proc/meminfo").expect("Unable to open the file");
    let mut reader = BufReader::new(proc_memory_file);
    let mut total_memory_line = String::new();
    let mut free_memory_line = String::new();

    let total_memory_line_size = reader.read_line(&mut total_memory_line);

    // Ignore thi line but read it anyway
    reader.read_line(&mut String::new());

    let free_memory_line_size = reader.read_line(&mut free_memory_line);

    let total_memory_vect: Vec<&str> = total_memory_line.split(":").collect();
    let free_memory_vect: Vec<&str> = free_memory_line.split(":").collect();

    let total_memory_str = total_memory_vect[1];
    let free_memory_str = free_memory_vect[1];

    let total_memory: u64 = total_memory_str[0..total_memory_str.len() - 3]
        .to_string()
        .trim()
        .parse()
        .expect("Can't parse number");
    let free_memory: u64 = free_memory_str[0..free_memory_str.len() - 3]
        .to_string()
        .trim()
        .parse()
        .expect("Can't parse number");

    ((free_memory as f64 / total_memory as f64) * 100.0) as f32
}
