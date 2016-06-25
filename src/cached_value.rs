use std::time::{Instant, Duration};
use std::ops::*;

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
impl<T> DerefMut for CachedValue<T> {
  fn deref_mut(&mut self) -> &mut T {
    self.expires = Instant::now() + self.lifetime;
    &mut self.data
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::time::{Instant, Duration};
  use std::thread;

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

  fn sleep(ms : u64) {
    thread::sleep( Duration::from_millis(ms) );
  }
}
