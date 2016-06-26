use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::{spawn, JoinHandle};
use std::fmt::Debug;

use cached_value::CachedValue;

#[derive(Debug)]
pub enum InternalCommunication<T> {
  Message(T), Shutdown,
}
use self::InternalCommunication::{Message, Shutdown};

pub struct UserSettings<T> where T : Sized + Send + Debug + 'static {
  sender : Sender<InternalCommunication<T>>,
  //receiver : Receiver<T>
  th : JoinHandle<()>,
}

impl<T> UserSettings<T> where T : Sized + Send + Debug {
  /// Start a self cleaning instance
  pub fn new() -> UserSettings<T> {
    let (tx, rx) = channel();

    let th = spawn(move|| {
      loop {
        match rx.recv().unwrap() {
          Message(m) => {
            println!("Message: {:?}", m)
          },

          Shutdown => {
            println!("Shutdown...");
            return ();
          }
        }
      }
    });

    UserSettings {
      sender: tx,
      th : th,
    }
  }

  /// You may send your request with this
  pub fn get_sender(&self) -> Sender<InternalCommunication<T>> {
    self.sender.clone()
  }

  /// Maybe refactor to Drop
  pub fn join(self) {
    self.sender.send(Shutdown).unwrap();
    self.th.join().unwrap();
  }
}
//impl<T> Drop for UserSettings<T> where T : Sized + Send + Debug {
//  fn drop(&mut self) {
//    self.sender.send(Shutdown).is_ok();
//  }
//}






#[cfg(test)]
mod test {
  use super::*;
  use super::InternalCommunication::Message;
  use cached_value::CachedValue as CV;
  use std::time::Duration;

  #[test]
  fn process_cancel() {
    let us = UserSettings::new();
    let tx = us.get_sender();

    tx.send( Message( CV::new(42, dur1()) ) ).unwrap();

    us.join();
  }


  fn dur1() -> Duration {
    Duration::from_millis(1)
  }
}
