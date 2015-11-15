//! # user_input_compiler
//!
//! Parses Input with this Grammer after trimming the Input and ignoring Whitespaces:
//! 
//! Command         := "/" CommandWord
//! CommandWord     := Status | Subscribe | Cancel | InvalidSyntax
//! Status          := "status" | "crowd"
//! Subscribe       := "subscribe" SensorSelector Duration
//! SensorSelector  := SensorString OptionalNum
//! SensorString    := "temperature" | "door_locked" | "barometer" | "radiation.alpha" | "radiation.beta" | "radiation.gamma" | "radiation.beta_gamma" | "humidity" | "beverage_supply" | "power_consumption" | "wind" | "network_connections" | "account_balance" | "total_member_count" | "people_now_present"
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
    
    matchCommandWord(s)
  }
}

fn matchCommandWord(s :Chars) -> Input {
  
  InvalidSyntax( format!("") )
}


fn tokenize(s :&str) -> Vec<&str> {
  let mut t :Vec<&str> = Vec::new();
  
  t
}


#[cfg(test)]
mod test {
  use super::*;
  use super::Input::*;
  
  
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
}
