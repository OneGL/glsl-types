#![deny(clippy::all)]

mod cli;
mod debounce;
mod generator;
mod log;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn start_cli(args: Vec<String>) -> () {
  cli::start(args);
}
