use crate::log_with_color;

pub enum Level {
  // INFO,
  WARN,
  ERROR,
}

pub fn print_level(level: Level) {
  match level {
    // Level::INFO => {
    //   log_with_color("[INFO]\t", "green");
    // }
    Level::WARN => {
      log_with_color("[WARN]\t", "yellow");
    }
    Level::ERROR => {
      log_with_color("[ERROR]\t", "red");
    }
  }
}
