//extern crate spaceapi;
use std::io::prelude::*;
use rustc_serialize::json;
use hyper::Client;
use spaceapi::Optional::{self, Value, Absent};
use spaceapi::{Status};
use spaceapi::sensors::{PeopleNowPresentSensor};

use std::time::Duration;
use time::SteadyTime;
use std::env;
use std::fs::{self, File};
use std::io;

pub struct SpaceApiClient {
  last_fetch: i64,
  webcams: Vec<String>,
}

impl SpaceApiClient {
  pub fn new() -> SpaceApiClient {
    let mut s = SpaceApiClient{
      last_fetch: 0,
      webcams: Vec::new(),
    };
    
    s.fetch_webcams();
    
    s
  }
  
  pub fn fetch_people_now_present(&self) -> ::std::result::Result<u64, String> {
    let status = try!(fetch_status());
    
    extract_people_now_present(status)
  }

  pub fn get_webcams(&self) -> Vec<String> {
    self.webcams.clone()
  }

  fn fetch_webcams(&mut self) {
    if let Ok(status) = fetch_status() {
      
      if let Value(cams) = status.cam {
        self.webcams = cams;
      }
    }
  }
  
  pub fn get_tmp_path_for_webcam(&self, url :&String) -> Result<String,io::Error> {
    let dir = env::temp_dir().join("coredump_bot").join("get_tmp_path_for_webcam");
    let path = env::temp_dir().join("coredump_bot").join("get_tmp_path_for_webcam").join( self.basename(url) );
    let path = path.as_path();
    
    
    fs::create_dir_all(dir);
    
    let mut f = try!(File::create(&path));
    
    
    let bin = try!(fetch_binary(url));
    
    
    f.write_all(&bin);
    f.sync_all();
    
    Ok::<String,io::Error>(format!("{}", path.to_str().unwrap()))
  }
  
  pub fn basename(&self, path :&String) -> String {
    let mut r = String::new();
    
    for c in path.chars() {
      match c {
        '/' => r.clear(),
        _ => r.push( c ),
      }
    }
    
    r
  }
}

fn extract_people_now_present(status :Status) -> Result<u64, String> {
  match status.sensors {
    Absent => Err(format!("response contains no sensors")),
    Value(sensors) => {
      match sensors.people_now_present {
        Absent => Err(format!("response contains no sensors.people_now_present")),
        Value(v) => {
          
          if v.is_empty() {
            Err(format!("response.sensors.people_now_present is empty"))
          } else {
              Ok( v[0].value )
          }
          
        }
      }
    }
  }
}

/// Fetch the Status from https://status.crdmp.ch/
fn fetch_status() -> Result<Status,String> {
  let client = Client::new();

  match client.get("https://status.crdmp.ch/").send() {
    Err(e) => Err(format!("client.get() error:\nError: {}", e)),
    Ok(mut res) => {
      
      let mut body = String::new();
      match res.read_to_string(&mut body) {
        Err(e) => { Err(format!("unable to connect to server, try again later:\nError: {}\nBody: {}", e, body)) },
        Ok(_/*len*/) => {
          
          match json::decode( &*body ) {
            Err(e) => Err(format!("unable to parse server response: {:?}", e)),
            Ok(status) => Ok(status),
          }
        }
      }
    }
  }
}

/// Fetch a Binary from url and save it to a temporary Location.
/// returns the temp Path
fn fetch_binary(url :&String) -> Result<Vec<u8>,io::Error> {
  let client = Client::new();
  
  let mut res = match client.get(url).send() {
    Ok(v) => v,
    Err(e) => return Err(io::Error::new(io::ErrorKind::BrokenPipe, format!("{:?}", e))),
  };
  
  let mut v = vec![];
  let size = try!(res.read_to_end(&mut v));
  
  Ok(v)

  /*match client.get(url).send() {
    Err(e) => Err(format!("client.get({}) error:\nError: {}", url, e)),
    Ok(mut res) => {
      
      match res.read_to_end(&mut v) {
        Err(e) => Err(format!("")),
        Ok(size) => {
          Ok( v )
        }
      }
    }
  }*/
}



#[cfg(test)]
mod test {
  use super::{extract_people_now_present};
  use spaceapi::{Status, Location, Contact};
  use spaceapi::optional::Optional;
  use spaceapi::sensors::{TemperatureSensor, PeopleNowPresentSensor};
  use rustc_serialize::json::{self};
  
  fn good_response() -> Status {
    let s :String = "{\"api\":\"0.13\",\"contact\":{\"email\":\"vorstand@lists.coredump.ch\",\"foursquare\":\"525c20e5498e875d8231b1e5\",\"irc\":\"irc://freenode.net/#coredump\",\"twitter\":\"@coredump_ch\"},\"feeds\":{\"blog\":{\"type\":\"rss\",\"url\":\"https://www.coredump.ch/feed/\"}},\"issue_report_channels\":[\"email\",\"twitter\"],\"location\":{\"address\":\"Spinnereistrasse 2, 8640 Rapperswil, Switzerland\",\"lat\":47.22936,\"lon\":8.82949},\"logo\":\"https://www.coredump.ch/logo.png\",\"projects\":[\"https://www.coredump.ch/projekte/\",\"https://discourse.coredump.ch/c/projects\",\"https://github.com/coredump-ch/\"],\"sensors\":{\"people_now_present\":[{\"location\":\"Hackerspace\",\"value\":0}],\"temperature\":[{\"location\":\"Hackerspace\",\"name\":\"Raspberry CPU\",\"unit\":\"°C\",\"value\":55.7}]},\"space\":\"coredump\",\"spacefed\":{\"spacenet\":false,\"spacephone\":false,\"spacesaml\":false},\"state\":{\"message\":\"Open every Monday from 20:00\",\"open\":false},\"url\":\"https://www.coredump.ch/\"}".into();
    
    json::decode( &s ).unwrap()
  }
  
  fn minimal_response() -> Status {
    let s :String = "{\"api\":\"0.13\",\"contact\":{\"email\":\"vorstand@lists.coredump.ch\",\"foursquare\":\"525c20e5498e875d8231b1e5\",\"irc\":\"irc://freenode.net/#coredump\",\"twitter\":\"@coredump_ch\"},\"feeds\":{\"blog\":{\"type\":\"rss\",\"url\":\"https://www.coredump.ch/feed/\"}},\"issue_report_channels\":[\"email\",\"twitter\"],\"location\":{\"lat\":47.22936,\"lon\":8.82949},\"logo\":\"https://www.coredump.ch/logo.png\",\"projects\":[\"https://www.coredump.ch/projekte/\",\"https://discourse.coredump.ch/c/projects\",\"https://github.com/coredump-ch/\"],\"sensors\":{\"temperature\":[{\"location\":\"Hackerspace\",\"name\":\"Raspberry CPU\",\"unit\":\"°C\",\"value\":55.7}]},\"space\":\"coredump\",\"spacefed\":{\"spacenet\":false,\"spacephone\":false,\"spacesaml\":false},\"state\":{\"message\":\"Open every Monday from 20:00\",\"open\":false},\"url\":\"https://www.coredump.ch/\"}".into();
    
    json::decode( &s ).unwrap()
  }
  
  fn cam_response() -> Status {
    let s :String = "{\"api\":\"0.13\",\"cam\":[\"https://webcam.coredump.ch/cams/ultimaker.jpg\"],\"contact\":{\"email\":\"vorstand@lists.coredump.ch\",\"foursquare\":\"525c20e5498e875d8231b1e5\",\"irc\":\"irc://freenode.net/#coredump\",\"twitter\":\"@coredump_ch\"},\"feeds\":{\"blog\":{\"type\":\"rss\",\"url\":\"https://www.coredump.ch/feed/\"}},\"issue_report_channels\":[\"email\",\"twitter\"],\"location\":{\"address\":\"Spinnereistrasse 2, 8640 Rapperswil, Switzerland\",\"lat\":47.22936,\"lon\":8.82949},\"logo\":\"https://www.coredump.ch/logo.png\",\"projects\":[\"https://www.coredump.ch/projekte/\",\"https://discourse.coredump.ch/c/projects\",\"https://github.com/coredump-ch/\"],\"sensors\":{\"people_now_present\":[{\"location\":\"Hackerspace\",\"value\":6}],\"temperature\":[{\"location\":\"Hackerspace\",\"name\":\"Raspberry CPU\",\"unit\":\"°C\",\"value\":48.7}]},\"space\":\"coredump\",\"spacefed\":{\"spacenet\":false,\"spacephone\":false,\"spacesaml\":false},\"state\":{\"message\":\"6 people here right now\",\"open\":true},\"url\":\"https://www.coredump.ch/\"}".into();
    
    json::decode( &s ).unwrap()
  }
  
  #[test]
  fn extract_people_now_present_0() {
    let n = extract_people_now_present( good_response() ).unwrap();
    
    assert_eq!(0, n);
  }
  #[test]
  fn extract_people_now_present_6() {
    let n = extract_people_now_present( cam_response() ).unwrap();
    
    assert_eq!(6, n);
  }
  #[test]
  fn extract_people_now_present_Err() {
    let e = extract_people_now_present( minimal_response() ).unwrap_err();
    
    assert_eq!("response contains no sensors.people_now_present", e);
  }
}
