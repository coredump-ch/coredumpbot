//! # user_input_compiler
//!
//! Parses Input with this Grammar after trimming the Input and ignoring Whitespaces:
//! 
//! Command         := "/" CommandWord
//! CommandWord     := Status | Subscribe | Cancel | Version | Help | WebCam | Start | Grammar | InvalidSyntax
//! Status          := "status" | "crowd"
//! Subscribe       := "subscribe" SensorSelector Duration
//! SensorSelector  := SensorString OptionalInteger
//! SensorString    := "account_balance" | "barometer" | "beverage_supply" | "door_locked" | "humidity" | "network_connections" | "power_consumption" | "temperature" | "total_member_count" | "radiation.alpha" | "radiation.beta_gamma" | "radiation.beta" | "radiation.gamma" | "people_now_present" | "wind"
//! OptionalInteger := Integer | ɛ
//! Integer         := [0-9]*
//! Duration        := Real TimeSuffix
//! TimeSuffix      := "m" | "min" | "h" | "d"
//! Real            := Integer "." Integer | Integer
//! Cancel          := "cancel"
//! Version         := "version"
//! Help            := "help"
//! WebCam          := "webcam" OptionalInteger
//! Start           := "start"
//! Grammar         := "grammar"
//! InvalidSyntax   := *

// ===========================================================================

use std::time::Duration;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Input {
  Status,
  Subscribe{ sensor :SensorSelector, duration :Duration },
  Cancel,
  Version,
  Help,
  WebCam{ nth :Option<usize> },
  Start,
  Grammar,
  InvalidSyntax( String ),
}
#[derive(Debug, PartialEq)]
pub struct SensorSelector {
  sensor_selector :String,
  nth :Option<u64>,
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
  } else
  if matches_with(s, "subscribe") {
    let sensor = extract!(match_sensor_selector(s));
    let duration = extract!(match_duration(s));
    return Subscribe{ sensor: sensor, duration: duration };
  } else
  
  if starts_with(s, "cancel") {
    return Cancel;
  } else
  
  if starts_with(s, "version") {
    return Version;
  } else
  
  if starts_with(s, "help") {
    return Help;
  } else
  
  if matches_with(s, "webcam") {
    let nth = match match_full_integer(s) {
      Ok(n) if n >= 0 => Some(n as usize),
      Ok(_) => return InvalidSyntax("Expected positive Integer".into()),
      Err(_) => None,
    };
    return WebCam{ nth: nth };
  } else
  
  if starts_with(s, "grammar") {
    return Grammar;
  } else 
  
  if starts_with(s, "start") {
    return Start;
  } else {
  
    return InvalidSyntax( "Invalid CommandWord".into() );
  }
}

fn match_sensor_selector(s :&mut Chars) -> Result<SensorSelector,Input> {
  let sensor;
  
  let w = consume_whitespaces(s);
  
  if starts_with(s, "account_balance") {
    sensor = "account_balance";
  } else
  if starts_with(s, "barometer") {
    sensor = "barometer";
  } else
  if starts_with(s, "beverage_supply") {
    sensor = "beverage_supply";
  } else
  if starts_with(s, "door_locked") {
    sensor = "door_locked";
  } else
  if starts_with(s, "humidity") {
    sensor = "humidity";
  } else
  if starts_with(s, "network_connections") {
    sensor = "network_connections";
  } else
  if starts_with(s, "power_consumption") {
    sensor = "power_consumption";
  } else
  if starts_with(s, "temperature") {
    sensor = "temperature";
  } else
  if starts_with(s, "total_member_count") {
    sensor = "total_member_count";
  } else
  if starts_with(s, "radiation.alpha") {
    sensor = "radiation.alpha";
  } else
  if starts_with(s, "radiation.beta_gamma") {
    sensor = "radiation.beta_gamma";
  } else
  if starts_with(s, "radiation.beta") {
    sensor = "radiation.beta";
  } else
  if starts_with(s, "radiation.gamma") {
    sensor = "radiation.gamma";
  } else
  if starts_with(s, "people_now_present") {
    sensor = "people_now_present";
  } else
  if starts_with(s, "wind") {
    sensor = "wind";
  } else {
    return Err( InvalidSyntax( format!("Invalid SensorSelector: {}", collect_iterator(s)) ) );
  }
  
  debug!("sensor: {}", sensor);
  s.skip(sensor.len() -1 +w).next();
  
  // OptionalInteger
  let mut it = s.clone();
  let mut nth = None;
  if let Ok(n) = match_integer(&mut it) {
    debug!("potential OptionalInteger: {}", n);
    match it.next() {
      Some(ws) => {
        debug!("ws: '{}'", ws);
        if ws == ' ' || ws == '\t' || ws == '\r' || ws == '\n' {
          nth = match match_integer(s) {
            Ok(n) if n >= 0 => {
              debug!("next Integer: {}", n);
              Some(n as u64)
            },
            Ok(n) => return Err( InvalidSyntax( format!("Index {} must be positive", n) ) ),
            Err(_) => None,
          };
        }
      },
      None => {
        nth = None;
      },
    }
  }
  
  
  
  Ok( SensorSelector{ sensor_selector: format!("{}", sensor), nth: nth } )
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
  st.parse::<f64>().map_err(|msg| InvalidSyntax(format!("Invalid Real: {:?}", msg)) )
}

/// Real            := Integer "." Integer | Integer
fn collect_real(s: &mut Chars) -> Result<String, Input> {
  let mut i1 = try!( collect_integer(s) );
  
  let mut it = s.clone();
  
  if let Some(punkt) = it.next() {
    if punkt != '.' {
      return Ok( i1 );
    }
    s.next(); // consume '.'
    let i2 = try!(collect_integer(s));
    
    i1 = i1 + "." + &i2;
  }
  
  Ok( i1 )
}

fn match_full_integer(s :&mut Chars) -> Result<i64, Input> {
  let _ = consume_whitespaces(s);
  let sign = if let Some(c) = s.clone().next() {
    if c == '-' { -1 } else { 1 }
  } else { 1 };
  
  // Skip '-'
  if sign < 0 {
    s.next();
  }
  
  
  
  let st :String = try!(collect_integer(s));
  match st.parse::<i64>() {
    Ok(val)  => Ok(sign * val),
    Err(msg) => Err( InvalidSyntax(format!("Invalid Integer: {:?}", msg)) ),
  }
}

fn match_integer(s :&mut Chars) -> Result<i64, Input> {
  let st :String = try!(collect_integer(s));
  st.parse::<i64>().map_err( |msg| InvalidSyntax(format!("Invalid Integer: {:?}", msg)) )
}

/// Integer         := [0-9]*
fn collect_integer(s :&mut Chars) -> Result<String, Input> {
  let mut i = format!("");
  let mut it = s.clone();
  
  
  let w = consume_whitespaces(&mut it);
  
  for c in it {
    info!("collect_integer: '{}'", c);
    match c {
      '0' ... '9' => i.push( c ),
      _ => break,
    }
  }
  
  if i.len() == 0 {
    Err( InvalidSyntax(format!("Invalid Integer")) )
  } else {
    s.skip( i.len() -1 +w).next();
    Ok( i )
  }
}

/// TimeSuffix      := "m" | "min" | "h" | "d"
/// Factor to multiply with Seconds
fn match_timesuffix(s :&mut Chars) -> Result<i64, Input> {
  consume_whitespaces(s);
  
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


/// Search without modifing the Iterator
fn starts_with(haystack_iter :&Chars, needle :&str) -> bool {
  let mut iter = haystack_iter.clone();
  
  for c in needle.chars() {
    if c != iter.next().unwrap_or('/') {
      return false;
    }
  }
  
  true
}

/// Search and advance the Iterator
fn matches_with(haystack_iter :&mut Chars, needle :&str) -> bool {
  let mut steps_taken = 0;
  let mut iter = haystack_iter.clone();
  
  for c in needle.chars() {
    steps_taken += 1;
    if c != iter.next().unwrap_or('/') {
      return false;
    }
  }
  
  haystack_iter.skip(steps_taken -1).next();
  
  true
}

fn collect_iterator(it :&mut Chars) -> String {
  let mut s = String::new();
  
  for c in it {
    s.push(c);
  }
  
  s
}

fn consume_whitespaces(it :&mut Chars) -> usize {
  let dry_run = it.clone();
  let mut skip :usize = 0;
  
  for c in dry_run {
    match c {
      ' ' | '\t' | '\r' | '\n' => { skip += 1 },
      _ => break,
    }
  }
  
  if skip > 0 {
    it.skip(skip -1).next();
  }
  skip
}




#[cfg(test)]
mod test {
  #![allow(non_snake_case)] // may change to non_snake_case_functions
  use super::*;
  use super::Input::*;
  use super::{starts_with, matches_with, consume_whitespaces, match_duration, match_integer, match_full_integer, match_real, match_timesuffix};
  use std::time::Duration;
  
  
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
  
  #[test]
  fn starts_with_final_iterator_state() {
    let mut s = "abcdef".chars();
    assert!( starts_with(&mut s, "abcxxx") == false );
    assert!( starts_with(&mut s, "abcd") );
    assert_eq!( Some('a'), s.next() );
  }
  
  #[test]
  fn matches_with_final_iterator_state() {
    let mut s = "abcdef".chars();
    assert!( matches_with(&mut s, "abcxxx") == false );
    assert!( matches_with(&mut s, "abcd") );
    assert_eq!( Some('e'), s.next() );
  }
  
  #[test]
  fn consume_whitespaces_iter_position() {
    let mut s = "    -".chars();
    let w = consume_whitespaces(&mut s);
    
    assert_eq!( 4, w );
    assert_eq!( Some('-'), s.next() );
  }
  
  // =================== Parser Tests ===================
  
  #[test]
  fn empty_2_fail() {
    assert_eq!(InvalidSyntax("Empty Request".into()), Input::from(format!("")))
  }
  
  #[test]
  fn status() {
    assert_eq!(Status, Input::from( format!("/status") ) )
  }
  
  #[test]
  fn crowd() {
    assert_eq!(Status, Input::from( format!("/crowd") ) )
  }
  
  #[test]
  fn subscribe_pnp_10min() {
    assert_eq!(Subscribe{ sensor: SensorSelector{ sensor_selector: "people_now_present".into(), nth: None }, duration: Duration::from_secs(10*60) } 
        , Input::from( format!("/subscribe people_now_present 10min") ) )
  }
  
  #[test]
  fn subscribe_pnp_13_10min() {
    assert_eq!(Subscribe{ sensor: SensorSelector{ sensor_selector: "people_now_present".into(), nth: Some(13) }, duration: Duration::from_secs(10*60) } 
        , Input::from( format!("/subscribe people_now_present 13 10min") ) )
  }
  
  #[test]
  fn subscribe_pnp_2h() {
    assert_eq!(Subscribe{ sensor: SensorSelector{ sensor_selector: "people_now_present".into(), nth: None }, duration: Duration::from_secs(2*60*60) } 
        , Input::from( format!("/subscribe people_now_present 2h") ) )
  }
  
  #[test]
  fn subscribe_pnp_7d() {
    assert_eq!(Subscribe{ sensor: SensorSelector{ sensor_selector: "people_now_present".into(), nth: None }, duration: Duration::from_secs(7*60*60*24) } 
        , Input::from( format!("/subscribe people_now_present 7d") ) )
  }
  
  #[test]
  fn cancel() {
    assert_eq!(Cancel, Input::from( format!("/cancel") ) )
  }
  
  
  
  #[test]
  fn integer42() {
    let mut s = "42".chars();
    assert_eq!(Ok(42), match_integer(&mut s));
  }
  
  #[test]
  fn integer_neg42() {
    let mut s = "-42".chars();
    assert_eq!(Err(InvalidSyntax("Invalid Integer".into())), match_integer(&mut s))
  }
  
  #[test]
  fn integer__neg42() {
    assert_eq!( Err(InvalidSyntax("Invalid Integer".into())), match_integer(&mut " -42 ".chars()) );
  }
  
  #[test]
  fn real6_6() {
    assert_eq!( Ok(6.6), match_real(&mut "6.6".chars()) )
  }
  
  #[test]
  fn real_123_456_() {
    assert_eq!( Ok(123.456), match_real(&mut " 123.456 ".chars()) )
  }
  
  #[test]
  fn real666_666() {
    assert_eq!( Ok(666.666), match_real(&mut "666.666".chars()) )
  }
  
  #[test]
  fn real10min() {
    assert_eq!( Ok(10 as f64), match_real(&mut "10min".chars()) )
  }
  
  #[test]
  fn real_duration_10min() {
    let mut s = " 10min".chars();
    
    assert_eq!( Ok(10 as f64), match_real(&mut s) );
    assert_eq!( Ok(60), match_timesuffix(&mut s) );
  }
  
  #[test]
  fn real_duration_10__min() {
    let mut s = " 10  min".chars();
    
    assert_eq!( Ok(10 as f64), match_real(&mut s) );
    assert_eq!( Ok(60), match_timesuffix(&mut s) );
  }
  
  #[test]
  fn real_duration__10_5min__() {
    let mut s = "  10.5min  ".chars();
    
    assert_eq!( Ok(10.5), match_real(&mut s) );
    assert_eq!( Ok(60), match_timesuffix(&mut s) );
  }
  
  #[test]
  fn real_duration__10_5__min__() {
    let mut s = "  10.5  min  ".chars();
    
    assert_eq!( Ok(10.5), match_real(&mut s) );
    assert_eq!( Ok(60), match_timesuffix(&mut s) );
  }
  
  #[test]
  fn real12_3__45_6() {
    let mut s = "12.3  45.6".chars();
    
    assert_eq!( Ok(12.3), match_real(&mut s) );
    assert_eq!( Ok(45.6), match_real(&mut s) );
  }
  
  #[test]
  //#[should_panic(expected = "InvalidSyntax(\"Invalid Integer\")")]
  fn real6_punkt_6() {
    assert_eq!( Err(InvalidSyntax("Invalid Integer".into())), match_real(&mut "6..6".chars()) )
  }
  
  
  #[test]
  fn help() {
    assert_eq!( Help, Input::from( format!("/help") ) )
  }
  
  #[test]
  fn version() {
    assert_eq!( Version, Input::from( format!("/version") ) )
  }
  
  #[test]
  fn start() {
    assert_eq!( Start, Input::from( format!("/start") ) )
  }
  
  
  
  #[test]
  fn duration_10min() {
    assert_eq!( Ok(Duration::from_secs(10*60)), match_duration(&mut "10min".chars()) )
  }
  
  
  
  #[test]
  fn match_integer_position() {
    let mut s = "  10  22".chars();
    assert_eq!( Ok(10), match_integer(&mut s) ); // I do not care here
    
    assert_eq!( "  22", s.collect::<String>() );
  }
  
  #[test]
  fn match_integer_position_spaces() {
    let mut s = "  10  ".chars();
    assert_eq!( Ok(10), match_integer(&mut s) ); // I do not care here
    
    assert_eq!( "  ", s.collect::<String>() );
  }
  
  #[test]
  fn match_integer_fail_position_spaces() {
    let mut s = "  bla  ".chars();
    assert_eq!( Err(InvalidSyntax("Invalid Integer".into())), match_integer(&mut s) ); // I do not care here
    
    assert_eq!("  bla  ", s.collect::<String>());
  }
  
  #[test]
  fn match_integer_fail_neg42() {
    assert_eq!( Err( InvalidSyntax("Invalid Integer".into()) ), match_integer(&mut "  -42  ".chars()) )
  }
  
  #[test]
  fn match_full_integer_neg42() {
    assert_eq!( Ok(-42), match_full_integer(&mut "  -42  ".chars()) )
  }
  
  
  
  #[test]
  fn webcam() {
    assert_eq!( WebCam{ nth: None }, Input::from( format!("/webcam") ) )
  }
  
  #[test]
  fn webcam_negative_1() {
    assert_eq!( InvalidSyntax("Expected positive Integer".into()), Input::from( format!("/webcam -1") ))
  }
  
  #[test]
  fn webcam_negative_13() {
    assert_eq!( InvalidSyntax("Expected positive Integer".into()), Input::from( format!("/webcam -13") ))
  }
  
  #[test]
  fn webcam_42() {
    assert_eq!( WebCam{ nth: Some(42) }, Input::from( format!("/webcam 42") ) )
  }
  
  #[test]
  fn webcam_23() {
    assert_eq!( WebCam{ nth: Some(23) }, Input::from( format!("/webcam 23") ) )
  }
  
  
  
  #[test]
  fn grammar() {
    assert_eq!( Grammar, Input::from( format!("/grammar") ) )
  }
}
