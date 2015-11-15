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
use self::Input::*;

/// like try! but unwraps the Error
macro_rules! extract {
    ($expr:expr) => (match $expr {
        Result::Ok(val) => val,
        Result::Err(err) => {
            return err
        }
    })
}


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
    
    match_command_word(&mut s)
  }
}

fn match_command_word(s :&mut Chars) -> Input {
  if starts_with(s, "status") || starts_with(s, "crowd") {
    return Status;
  }
  if starts_with(s, "subscribe") {
    let sensor = extract!(match_sensor_selector(s));
    let duration = extract!(match_duration(s));
    return Subscribe{ sensor: sensor, duration: duration };
  }
  
  if starts_with(s, "cancel") {
    return Cancel;
  }
  
  InvalidSyntax( format!("Invalid CommandWord") )
}

fn match_sensor_selector(s :&mut Chars) -> Result<String,Input> {
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

/// Duration        := Real TimeSuffix
fn match_duration(s :&mut Chars) -> Result<Duration,Input> {
  let real :f64 = try!(match_real(s));
  let ts   :i64 = try!(match_timesuffix(s));
  
  let duration = Duration::from_secs((real * (ts as f64)) as u64);
  
  Ok( duration )
}

fn match_real(s: &mut Chars) -> Result<f64, Input> {
  let st :String = try!(collect_real(s));
  match st.parse::<f64>() {
    Ok(val) => Ok(val),
    Err(msg) => Err( InvalidSyntax(format!("Invalid Real: {:?}", msg)) )
  }
}

/// Real            := Integer "." Integer | Integer
fn collect_real(s: &mut Chars) -> Result<String, Input> {
  let mut i1 = try!( collect_integer(s) );
  
  if let Some(punkt) = s.next() {
    if punkt != '.' {
      return Err( InvalidSyntax( format!("expected '.' found '{}'", punkt) ) );
    }
    let i2 = try!(collect_integer(s));
    
    i1 = i1 + "." + &i2;
  }
  
  Ok( i1 )
}

fn match_integer(s :&mut Chars) -> Result<i64, Input> {
  let st :String = try!(collect_integer(s));
  match st.parse::<i64>() {
    Ok(val) => Ok(val),
    Err(msg) => Err( InvalidSyntax(format!("Invalid Integer: {:?}", msg)) ),
  }
}

/// Integer         := [0-9]*
fn collect_integer(s :&mut Chars) -> Result<String, Input> {
  let mut i = format!("");
  let mut it = s.clone();
  
  for c in it {
    println!("collect_integer: {}", c);
    match c {
      ' ' | '\t' | '\r' | '\n' => { /* ignoring */ },
      '0' ... '9' => i.push( c ),
      _ => break,
    }
  }
  
  if i.len() == 0 {
    Err( InvalidSyntax(format!("Invalid Integer")) )
  } else {
    s.skip( i.len() -1 ).next();
    Ok( i )
  }
}

/// TimeSuffix      := "m" | "min" | "h" | "d"
/// Factor to multiply with Seconds
fn match_timesuffix(s :&mut Chars) -> Result<i64, Input> {
  if starts_with(s, "m") || starts_with(s, "min") {
    return Ok(60);
  }
  if starts_with(s, "h") {
    return Ok(60*60);
  }
  if starts_with(s, "d") {
    return Ok(60*60*24);
  }
  
  Err( InvalidSyntax(format!("Invalid TimeSuffix")) )
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


#[cfg(test)]
mod test {
  use super::*;
  use super::Input::*;
  use super::{starts_with,match_duration,match_integer,match_real,match_timesuffix};
  
  
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
    assert!( starts_with(&mut s, "abcd") );
  }
  
  #[test]
  fn starts_with_four() {
    let mut s = "abcdef".chars();
    assert!( starts_with(&mut s, "abcxxx") == false );
    assert!( starts_with(&mut s, "abcxyz") == false );
    assert!( starts_with(&mut s, "abcxyz") == false );
    assert!( starts_with(&mut s, "abcd") );
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
  
  #[test]
  fn integer42() {
    let mut s = "42".chars();
    assert_eq!(42, match_integer(&mut s).unwrap());
  }
  
  #[test]
  fn real6_6() {
    let mut s = "6.6".chars();
    match match_real(&mut s) {
      Ok(v) => assert_eq!(6.6, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
  }
  
  #[test]
  fn real666_666() {
    let mut s = "666.666".chars();
    match match_real(&mut s) {
      Ok(v) => assert_eq!(666.666, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
  }
  
  #[test]
  //#[should_panic(expected = "InvalidSyntax(\"Invalid Integer\")")]
  fn real6__6() {
    let mut s = "6..6".chars();
    match match_real(&mut s) {
      Ok(v) => assert!(false),
      Err(e) => {
        println!("====={:?}", e);
        assert!(true);
      },
    }
  }
}
