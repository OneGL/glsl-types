use std::io::Write;
use std::process::{Command, Stdio};

pub enum CheckError {
  UnsupportedPlatform,
}

#[derive(Debug)]
pub struct ErrorData {
  pub message: String,
  pub line: u32,
}

fn get_error_data(output: &str) -> Vec<ErrorData> {
  let mut errors = Vec::new();

  for line in output.lines() {
    let mut line = String::from(line);
    if line.starts_with("ERROR: 0:") {
      line = line[9..].to_string();
    } else if line.starts_with("WARNING: 0:") {
      line = line[11..].to_string();
    } else {
      continue;
    }

    let colon_index = line.find(':').unwrap();
    let line_number = line[0..colon_index].parse::<u32>().unwrap();
    let mut message = line[colon_index + 1..].trim().to_string();

    if message.starts_with("'' :  ") {
      message = message[6..].to_string();
    }

    errors.push(ErrorData {
      message,
      line: line_number,
    });
  }

  return errors;
}

pub fn error_check(source: &str) -> Result<Vec<ErrorData>, CheckError> {
  let base = std::env::current_dir().unwrap();

  let platform_name = if cfg!(target_os = "windows") {
    "Windows"
  } else if cfg!(target_os = "macos") {
    "Mac"
  } else if cfg!(target_os = "linux") {
    "Linux"
  } else {
    return Err(CheckError::UnsupportedPlatform);
  };

  let validator_path = base
    .join("bin")
    .join(format!("glslangValidator{}", platform_name));

  let stage = "vert";

  let mut child = Command::new(validator_path)
    .arg("--stdin")
    .arg("-C")
    .arg("-S")
    .arg(stage)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to start validator");

  if let Some(ref mut stdin) = child.stdin {
    stdin
      .write_all(source.as_bytes())
      .expect("Failed to write to stdin");
  } else {
    panic!("Failed to open stdin");
  }

  let output = child.wait_with_output().expect("Failed to read stdout");
  Ok(get_error_data(&String::from_utf8_lossy(&output.stdout)))
}
