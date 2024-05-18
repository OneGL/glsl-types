#![deny(clippy::all)]

mod cli;
mod uniforms;
mod generator;
mod debounce;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn start_cli(args: Vec<String>) -> napi::Result<()> {
  cli::start(args);
  Ok(())
}
