//! Handle information which expires

use std::time::{Instant, Duration};
use std::ops::*;

/// A fully generic container for expireing information
pub struct CachedValue<T> {
  data : T,
  expires : Instant,
  lifetime : Duration,
}

impl<T> CachedValue<T> {
  pub fn new(data: T, lifetime: Duration) -> CachedValue<T> {
    CachedValue {
      data: data,
      expires: Instant::now() + lifetime,
      lifetime: lifetime,
    }
  }

  pub fn has_expired(&self) -> bool {
    self.expires < Instant::now()
  }
}

impl<T> Deref for CachedValue<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.data
  }
}

/// Using this will renew the expiration:
impl<T> DerefMut for CachedValue<T> {
  fn deref_mut(&mut self) -> &mut T {
    self.expires = Instant::now() + self.lifetime;
    &mut self.data
  }
}


pub trait CacheContainer {
  fn remove_expired(&mut self);
}

impl<T> CacheContainer for ::std::vec::Vec<CachedValue<T>> {
  fn remove_expired(&mut self) {
    // Maybe switch to swap_remove()
    self.retain(|&ref e| e.has_expired() == false);
  }
}
impl<T> CacheContainer for ::std::collections::VecDeque<CachedValue<T>> {
  fn remove_expired(&mut self) {
    // Maybe switch to swap_remove()
    self.retain(|&ref e| e.has_expired() == false);
  }
}




#[cfg(test)]
mod test {
  use super::*;
  //use super::CacheContainer;
  use std::time::{Instant, Duration};
  use std::thread;
  use std::collections::VecDeque;

  #[test]
  fn expired_cache() {
    let c = CachedValue::new(42, Duration::new(0,0));

    sleep(1);

    assert_eq!(true, c.has_expired());
  }

  #[test]
  fn deref() {
    let c = CachedValue::new(42, Duration::new(0,0));

    assert_eq!(42, *c);
  }

  #[test]
  fn deref_mut() {
    let mut c = CachedValue::new(42, Duration::new(0,0));

    let expires_0 = c.expires;

    sleep(42);
    *c = 23;

    let expires_1 = c.expires;

    assert_eq!(23, *c);
    assert!(expires_0 < expires_1);
  }

  #[test]
  fn use_with_vec() {
    let mut v = vec![
      CachedValue::new(21, Duration::from_millis(1)),
      CachedValue::new(42, Duration::from_millis(4)),
    ];

    sleep(2);
    v.remove_expired();

    assert_eq!(1, v.len());
    assert_eq!(42, *v[0]);
  }

  #[test]
  fn use_with_vec_deque() {
    let mut v : VecDeque<_> = vec![
      CachedValue::new(21, Duration::from_millis(1)),
      CachedValue::new(42, Duration::from_millis(42)),
    ].into_iter().collect();

    sleep(2);
    v.remove_expired();

    assert_eq!(1, v.len());
    assert_eq!(42, *v[0]);
  }

  fn sleep(ms : u64) {
    thread::sleep( Duration::from_millis(ms) );
  }
}
