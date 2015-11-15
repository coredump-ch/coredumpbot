//! # user_input_compiler
//!
//! Parses Input with this Grammer after trimming the Input and ignoring Whitespaces:
//! 
//! Command         := "/" CommandWord
//! CommandWord     := Status | Subscribe | Cancel | InvalidSyntax
//! Status          := "status" | "crowd"
//! Subscribe       := "subscribe" SensorSelector Duration
//! SensorSelector  := SensorString OptionalNum
//! SensorString    := "account_balance" | "barometer" | "beverage_supply" | "door_locked" | "humidity" | "network_connections" | "power_consumption" | "temperature" | "total_member_count" | "radiation.alpha" | "radiation.beta_gamma" | "radiation.beta" | "radiation.gamma" | "people_now_present" | "wind"
//! OptionalInteger := Integer | ɛ
//! Integer         := [0-9]*
//! Duration        := Real TimeSuffix
//! TimeSuffix      := "m" | "min" | "h" | "d"
//! Real            := Integer "." Integer | Integer
//! Cancel          := "cancel"
//! InvalidSyntax   := *

// ===========================================================================

use std::time::Duration;
use std::str::Chars;

#[derive(Debug)]
pub enum Input {
  Status,
  Subscribe{ sensor: String, duration: Duration },
  Cancel,
  InvalidSyntax( String ),
}

/// like try! but unwraps the Error
macro_rules! extract {
    ($expr:expr) => (match $expr {
        Result::Ok(val) => val,
        Result::Err(err) => {
            return err
        }
    })
}

use self::Input::*;

impl From<String> for Input {
  /// Start the Parser/Compiler
  fn from(s :String) -> Input {
    let s = s.trim();
    
    if s.len() == 0 {
      return InvalidSyntax( format!("Empty Request") );
    }
    
    let mut s = s.chars();
    
    if s.next().unwrap() != '/' {
      return InvalidSyntax( format!("Command must start with /") );
    }
    
    matchCommandWord(&mut s)
  }
}

fn matchCommandWord(s :&mut Chars) -> Input {
  if starts_with(s, "status") || starts_with(s, "crowd") {
    return Status;
  }
  if starts_with(s, "subscribe") {
    let sensor = extract!(matchSensorSelector(s));
    let duration = extract!(collectDuration(s));
    return Subscribe{ sensor: sensor, duration: duration };
  }
  
  if starts_with(s, "cancel") {
    return Cancel;
  }
  
  InvalidSyntax( format!("Invalid CommandWord") )
}

fn matchSensorSelector(s :&mut Chars) -> Result<String,Input> {
  if starts_with(s, "account_balance") {
    return unimplemented();
  }
  if starts_with(s, "barometer") {
    return unimplemented();
  }
  if starts_with(s, "beverage_supply") {
    return unimplemented();
  }
  if starts_with(s, "door_locked") {
    return unimplemented();
  }
  if starts_with(s, "humidity") {
    return unimplemented();
  }
  if starts_with(s, "network_connections") {
    return unimplemented();
  }
  if starts_with(s, "power_consumption") {
    return unimplemented();
  }
  if starts_with(s, "temperature") {
    return unimplemented();
  }
  if starts_with(s, "total_member_count") {
    return unimplemented();
  }
  if starts_with(s, "radiation.alpha") {
    return unimplemented();
  }
  if starts_with(s, "radiation.beta_gamma") {
    return unimplemented();
  }
  if starts_with(s, "radiation.beta") {
    return unimplemented();
  }
  if starts_with(s, "radiation.gamma") {
    return unimplemented();
  }
  if starts_with(s, "people_now_present") {
    return unimplemented();
  }
  if starts_with(s, "wind") {
    return unimplemented();
  }
  
  // TODO OptionalNum
  
  Err( InvalidSyntax( format!("Invalid SensorSelector") ) )
}

fn collectDuration(s :&mut Chars) -> Result<Duration,Input> {
  Err( InvalidSyntax( format!("Invalid Duration") ) )
}

fn starts_with(it :&mut Chars, con :&str) -> bool {
  let mut steps_taken = 0;
  let mut iter = it.clone();
  
  for c in con.chars() {
    steps_taken += 1;
    if c != iter.next().unwrap_or('/') {
      return false;
    }
  }
  
  it.skip(steps_taken);
  
  true
}

fn unimplemented() -> Result<String,Input> {
  Err( InvalidSyntax( format!("CommandWord not implemented yet") ) )
}


fn tokenize(s :&str) -> Vec<&str> {
  let mut t :Vec<&str> = Vec::new();
  
  t
}


#[cfg(test)]
mod test {
  use super::*;
  use super::Input::*;
  use super::starts_with;
  
  
  // =================== Util Tests ===================
  #[test]
  fn starts_with_one() {
    let mut s = "abcdef".chars();
    assert!( starts_with(&mut s, "abcd") );
  }
  #[test]
  fn starts_with_two() {
    let mut s = "abcdef".chars();
    assert!( starts_with(&mut s, "xyz") == false );
    println!("======");
    assert!( starts_with(&mut s, "abcd") );
  }
  
  
  // =================== Lexer Tests ===================
  #[test]
  fn simple() {
    assert_eq!(vec!["/","status"], super::tokenize("/status") )
  }
  
  // =================== Parser Tests ===================
  
  #[test]
  fn empty_2_fail() {
    match Input::from( format!("") ) {
      InvalidSyntax(m) => assert_eq!("Empty Request", m),
      _ => assert!(false),
    }
  }
  
  #[test]
  fn status() {
    match Input::from( format!("/status") ) {
      Status => assert!(true),
      _ => assert!(false)
    }
  }
  
  #[test]
  fn crowd() {
    match Input::from( format!("/crowd") ) {
      Status => assert!(true),
      _ => assert!(false)
    }
  }
  
  #[test]
  fn subscribe_pnp_10min() {
    match Input::from( format!("/subscribe people_now_present 10min") ) {
      Subscribe{ sensor, duration } => {
        assert_eq!("people_now_present", sensor);
        assert_eq!(10 * 60, duration.as_secs());
      },
      _ => assert!(false)
    }
  }
  
  #[test]
  fn subscribe_pnp_2h() {
    match Input::from( format!("/subscribe people_now_present 2h") ) {
      Subscribe{ sensor, duration } => {
        assert_eq!("people_now_present", sensor);
        assert_eq!(2 * 60 * 60, duration.as_secs());
      },
      _ => assert!(false)
    }
  }
  
  #[test]
  fn subscribe_pnp_7d() {
    match Input::from( format!("/subscribe people_now_present 7d") ) {
      Subscribe{ sensor, duration } => {
        assert_eq!("people_now_present", sensor);
        assert_eq!(7 * 60 * 60 * 24 * 7, duration.as_secs());
      },
      _ => assert!(false)
    }
  }
  
  #[test]
  fn cancel() {
    match Input::from( format!("/cancel") ) {
      Cancel => assert!(true),
      _ => assert!(false)
    }
  }
}
