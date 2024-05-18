use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct Debouncer<T> {
  tx: mpsc::Sender<T>,
}

impl<T> Debouncer<T>
where
  T: Send + 'static,
{
  pub fn new<F>(delay: Duration, mut callback: F) -> Self
  where
    F: FnMut(T) + Send + 'static,
  {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
      let mut value = None;

      loop {
        let recv_result = rx.recv_timeout(delay);

        match recv_result {
          Ok(v) => {
            value = Some(v);
          }
          Err(mpsc::RecvTimeoutError::Timeout) => {
            if let Some(v) = value.take() {
              callback(v);
            }
          }
          Err(_) => break, // Handle potential disconnection
        }
      }
    });

    Self { tx }
  }

  pub fn call(&self, value: T) {
    let _ = self.tx.send(value);
  }
}
