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
//! Duration        := Real OptionalTimeSuffix
//! Real            := Integer "." Integer | Integer
//! Cancel          := "cancel"
//! InvalidSyntax   := *

use std::time::Duration;

pub enum Input {
  Status,
  Subscribe{ sensor: String, interval: Duration },
  Cancel,
  InvalidSyntax( String ),
}

impl From<String> for Input {
  /// Start the Parser/Compiler
  fn from(s :String) -> Input {
    let s = s.trim();
    
    Input::InvalidSyntax( format!("") )
  }
}

#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn empty_2_fail() {
    match Input::from( format!("") ) {
      Input::InvalidSyntax(m) => assert_eq!("Empty request", m),
      _ => assert!(false),
    }
  }
  
  #[test]
  fn status() {
    match Input::from( format!("/status") ) {
      Input::Status => assert!(true),
      _ => assert!(false)
    }
  }
}
