use colored::Colorize;

pub enum Level {
  INFO,
  WARN,
  ERROR,
}

pub fn print_level(level: Level) {
  match level {
    Level::INFO => {
      print!("{}", "[INFO]\t".green());
    }
    Level::WARN => {
      print!("{}", "[WARN]\t".yellow());
    }
    Level::ERROR => {
      print!("{}", "[ERROR]\t".red());
    }
  }
}
