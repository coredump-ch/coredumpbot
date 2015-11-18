//! # user_input_compiler
//!
//! Parses Input with this Grammer after trimming the Input and ignoring Whitespaces:
//! 
//! Command         := "/" CommandWord
//! CommandWord     := Status | Subscribe | Cancel | Version | Help | WebCam | Start | InvalidSyntax
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
//! InvalidSyntax   := *

// ===========================================================================

use std::time::Duration;
use std::str::Chars;

#[derive(Debug)]
pub enum Input {
  Status,
  Subscribe{ sensor :SensorSelector, duration :Duration },
  Cancel,
  Version,
  Help,
  WebCam{ nth :Option<u64> },
  Start,
  InvalidSyntax( String ),
}
#[derive(Debug)]
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
  if starts_with(s, "subscribe") {
    s.skip(9 -1).next();
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
  
  if starts_with(s, "webcam") {
    s.skip(6 -1).next();
    let nth = match match_integer(s) {
      Ok(n) if n >= 0 => Some(n as u64),
      Ok(_) => None,
      Err(e) => None,
    };
    return WebCam{ nth: nth };
  } else
  
  if starts_with(s, "start") {
    return Start;
  } else {
  
    return InvalidSyntax( format!("Invalid CommandWord") );
  }
}

fn match_sensor_selector(s :&mut Chars) -> Result<SensorSelector,Input> {
  let mut sensor;
  
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
  
  println!("sensor: {}", sensor);
  s.skip(sensor.len() -1 +w).next();
  
  // OptionalInteger
  let mut it = s.clone();
  let mut nth = None;
  if let Ok(n) = match_integer(&mut it) {
    println!("potential OptionalInteger: {}", n);
    match it.next() {
      Some(ws) => {
        println!("ws: '{}'", ws);
        if ws == ' ' || ws == '\t' || ws == '\r' || ws == '\n' {
          nth = match match_integer(s) {
            Ok(n) if n >= 0 => {
              println!("next Integer: {}", n);
              Some(n as u64)
            },
            Ok(n) => return Err( InvalidSyntax( format!("Index {} must be positive", n) ) ),
            Err(e) => None,
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
  println!("match_real: '{}'", st);
  match st.parse::<f64>() {
    Ok(val) => {
      println!("match_real: {}", val);
      Ok(val)
    },
    Err(msg) => Err( InvalidSyntax(format!("Invalid Real: {:?}", msg)) )
  }
}

/// Real            := Integer "." Integer | Integer
fn collect_real(s: &mut Chars) -> Result<String, Input> {
  let mut i1 = try!( collect_integer(s) );
  println!("collect_real.l1: '{}'", i1);
  
  let mut it = s.clone();
  
  if let Some(punkt) = it.next() {
    if punkt != '.' {
      println!("collect_real.punkt '{}'", punkt);
      return Ok( i1 );
      //return Err( InvalidSyntax( format!("expected '.' found '{}'", punkt) ) );
    }
    s.next(); // consume '.'
    let i2 = try!(collect_integer(s));
    println!("collect_real.l2: '{}'", i2);
    
    i1 = i1 + "." + &i2;
    println!("collect_real.l1: '{}'", i1);
  }
  
  Ok( i1 )
}

fn match_integer(s :&mut Chars) -> Result<i64, Input> {
  let st :String = try!(collect_integer(s));
  match st.parse::<i64>() {
    Ok(val) => {
      //s.skip(st.len() -1).next();
      println!("match_integer: {}", st);
      Ok(val)
    },
    Err(msg) => Err( InvalidSyntax(format!("Invalid Integer: {:?}", msg)) ),
  }
}

/// Integer         := [0-9]*
fn collect_integer(s :&mut Chars) -> Result<String, Input> {
  let mut i = format!("");
  let mut it = s.clone();
  
  
  let w = consume_whitespaces(&mut it);
  println!("collect_integer.s: '{}', w: '{}'", s.clone().collect::<String>(), w);
  
  for c in it {
    println!("collect_integer: '{}'", c);
    match c {
      //' ' | '\t' | '\r' | '\n' => { /* ignoring */ },
      '0' ... '9' => i.push( c ),
      _ => break,
    }
  }
  
  if i.len() == 0 {
    Err( InvalidSyntax(format!("Invalid Integer")) )
  } else {
    s.skip( i.len() -1 +w).next();
    println!("collect_integer.skip: i.len({}) w({}) skip({}) collect({})", i.len(), w, i.len() -1 +w, s.clone().collect::<String>());
    Ok( i )
  }
}

/// TimeSuffix      := "m" | "min" | "h" | "d"
/// Factor to multiply with Seconds
fn match_timesuffix(s :&mut Chars) -> Result<i64, Input> {
  let w = consume_whitespaces(s);
  println!("match_timesuffix: '{}'", s.clone().collect::<String>());
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

fn collect_iterator(it :&mut Chars) -> String {
  let mut s = String::new();
  
  for c in it {
    s.push(c);
  }
  
  s
}

fn consume_whitespaces(it :&mut Chars) -> usize {
  let mut dry_run = it.clone();
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
  use super::*;
  use super::Input::*;
  use super::{starts_with,match_duration,match_integer,match_real,match_timesuffix};
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
        assert_eq!("people_now_present", sensor.sensor_selector);
        assert_eq!(None, sensor.nth);
        assert_eq!(10 * 60, duration.as_secs());
      },
      InvalidSyntax(msg) => {
        println!("{}", msg);
        assert!(false);
      },
      _ => assert!(false)
    }
  }
  
  #[test]
  fn subscribe_pnp_13_10min() {
    match Input::from( format!("/subscribe people_now_present 13 10min") ) {
      Subscribe{ sensor, duration } => {
        assert_eq!("people_now_present", sensor.sensor_selector);
        assert_eq!(Some(13), sensor.nth);
        assert_eq!(10 * 60, duration.as_secs());
      },
      InvalidSyntax(msg) => {
        println!("{}", msg);
        assert!(false);
      },
      _ => assert!(false)
    }
  }
  
  #[test]
  fn subscribe_pnp_2h() {
    match Input::from( format!("/subscribe people_now_present 2h") ) {
      Subscribe{ sensor, duration } => {
        assert_eq!("people_now_present", sensor.sensor_selector);
        assert_eq!(None, sensor.nth);
        assert_eq!(2 * 60 * 60, duration.as_secs());
      },
      InvalidSyntax(msg) => {
        println!("{}", msg);
        assert!(false);
      },
      _ => assert!(false)
    }
  }
  
  #[test]
  fn subscribe_pnp_7d() {
    match Input::from( format!("/subscribe people_now_present 7d") ) {
      Subscribe{ sensor, duration } => {
        assert_eq!("people_now_present", sensor.sensor_selector);
        assert_eq!(None, sensor.nth);
        assert_eq!(7 * 60 * 60 * 24, duration.as_secs());
      },
      InvalidSyntax(msg) => {
        println!("{}", msg);
        assert!(false);
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
  fn real_123_456_() {
    let mut s = " 123.456 ".chars();
    match match_real(&mut s) {
      Ok(v) => assert_eq!(123.456, v),
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
  fn real10min() {
    let mut s = "10min".chars();
    match match_real(&mut s) {
      Ok(v) => assert_eq!(10 as f64, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
  }
  
  #[test]
  fn real_duration_10min() {
    let mut s = " 10min".chars();
    match match_real(&mut s) {
      Ok(v) => assert_eq!(10 as f64, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
    match match_timesuffix(&mut s) {
      Ok(v) => assert_eq!(60, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
  }
  
  #[test]
  fn real_duration_10__min() {
    let mut s = " 10  min".chars();
    match match_real(&mut s) {
      Ok(v) => assert_eq!(10 as f64, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
    match match_timesuffix(&mut s) {
      Ok(v) => assert_eq!(60, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
  }
  
  #[test]
  fn real_duration__10_5min__() {
    let mut s = "  10.5min  ".chars();
    match match_real(&mut s) {
      Ok(v) => assert_eq!(10.5 as f64, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
    match match_timesuffix(&mut s) {
      Ok(v) => assert_eq!(60, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
  }
  
  #[test]
  fn real_duration__10_5__min__() {
    let mut s = "  10.5  min  ".chars();
    match match_real(&mut s) {
      Ok(v) => assert_eq!(10.5 as f64, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
    match match_timesuffix(&mut s) {
      Ok(v) => assert_eq!(60, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
  }
  
  #[test]
  fn real12_3__45_6() {
    let mut s = "12.3  45.6".chars();
    match match_real(&mut s) {
      Ok(v) => assert_eq!(12.3, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
    match match_real(&mut s) {
      Ok(v) => assert_eq!(45.6, v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
  }
  
  #[test]
  //#[should_panic(expected = "InvalidSyntax(\"Invalid Integer\")")]
  fn real6_punkt_6() {
    let mut s = "6..6".chars();
    match match_real(&mut s) {
      Ok(v) => assert!(false),
      Err(e) => {
        println!("====={:?}", e);
        assert!(true);
      },
    }
  }
  
  #[test]
  fn help() {
    match Input::from( format!("/help") ) {
      Help => assert!(true),
      _ => assert!(false),
    }
  }
  
  #[test]
  fn version() {
    match Input::from( format!("/version") ) {
      Version => assert!(true),
      _ => assert!(false),
    }
  }
  
  #[test]
  fn start() {
    match Input::from( format!("/start") ) {
      Start => assert!(true),
      _ => assert!(false),
    }
  }
  
  
  
  #[test]
  fn duration_10min() {
    let mut s = "10min".chars();
    match match_duration(&mut s) {
      Ok(v) => assert_eq!(Duration::from_secs(10*60), v),
      Err(e) => {
        println!("{:?}", e);
        assert!(false);
      },
    }
  }
  
  #[test]
  fn match_integer_position() {
    let mut s = "  10  22".chars();
    match_integer(&mut s).unwrap_or(0); // I do not care here
    
    assert_eq!("  22", s.collect::<String>());
  }
  
  #[test]
  fn match_integer_position_spaces() {
    let mut s = "  10  ".chars();
    match_integer(&mut s).unwrap_or(0); // I do not care here
    
    assert_eq!("  ", s.collect::<String>());
  }
  
  #[test]
  fn match_integer_fail_position_spaces() {
    let mut s = "  bla  ".chars();
    match_integer(&mut s).unwrap_or(0); // I do not care here
    
    assert_eq!("  bla  ", s.collect::<String>());
  }
  
  
  
  #[test]
  fn webcam() {
    match Input::from( format!("/webcam") ) {
      WebCam{ nth } => if let None = nth { assert!(true) } else { assert!(false) },
      _ => assert!(false),
    }
  }
  
  #[test]
  fn webcam_42() {
    match Input::from( format!("/webcam 42") ) {
      WebCam{ nth } => {
        if let Some(nth) = nth { 
          assert!( if nth == 42 { true } else { println!("wrong Value: {}", nth); false } ) 
        } else {
          println!("expected OptionalInteger");
          assert!(false)
        }
      },
      _ => assert!(false),
    }
  }
}
