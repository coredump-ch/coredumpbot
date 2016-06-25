use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

use cached_value::CachedValue;


pub struct UserSettings<T> {
  sender : Sender<T>,
  //receiver : Receiver<T>
}

impl<T> UserSettings<T> {
  pub fn new() -> UserSettings<T> {
    let (tx, rx) = channel();

    UserSettings {
      sender: tx,
    }
  }

  pub fn get_sender(&self) -> Sender<T> {
    self.sender.clone()
  }
}
