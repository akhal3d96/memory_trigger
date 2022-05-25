use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;

pub fn check_memory_and_excute(memory_threshold: f32, action: &str) -> Option<i32> {
  let curr_memory = memory_precentage();
  if curr_memory > memory_threshold {
      // TODO: make shell optional
      let out = Command::new("sh")
          .arg("-c")
          .arg(action)
          .output()
          .expect("failed to excute process");

      io::stdout().write_all(&out.stdout).unwrap();
      io::stderr().write_all(&out.stderr).unwrap();

      match out.status.code() {
          Some(code) => return Some(code),
          None => panic!("exited with a signal"),
      }
  }

  None
}

fn memory_precentage() -> f32 {
  let proc_memory_file = File::open("/proc/meminfo").expect("Unable to open the file");
  let mut reader = BufReader::new(proc_memory_file);
  let mut total_memory_line = String::new();
  let mut free_memory_line = String::new();

  let _total_memory_line_size = reader.read_line(&mut total_memory_line);
  // Ignore this line but read it anyway
  reader.read_line(&mut String::new());

  let _free_memory_line_size = reader.read_line(&mut free_memory_line);

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

  100.0 - ((free_memory as f64 / total_memory as f64) * 100.0) as f32
}