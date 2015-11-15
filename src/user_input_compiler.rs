//! # user_input_compiler
//!
//! Parses Input with this Grammer after trimming the Input:
//! 
//! Command      := "/" CommandWord
//! CommandWord  := Status | Subscribe | Cancel | InvalidSyntax
//! Status       := "status" | "crowd"
//! Subscribe    := "subscribe" SensorString Duration
//! SensorString := "... TODO Space API ..."
//! Duration     := ... TODO ...
//! Cancel       := "cancel"
//! InvalidSyntax:= *

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
