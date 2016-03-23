extern crate spaceapi;
use std::io::prelude::*;
use rustc_serialize::json;
use hyper::Client;
use spaceapi::Optional::{self, Value, Absent};
use spaceapi::{Status, Location};

use std::env;
use std::fs::{self, File};
use std::io;
use chrono::{DateTime, UTC};

pub struct SpaceApiClient {
  last_fetch: DateTime<UTC>,
  status: spaceapi::Status,
}

impl SpaceApiClient {
  pub fn new() -> SpaceApiClient {
    let empty_location = json::decode(r#"{ "lon": 0.0, "lat": 0.0 }"#).unwrap();
    let emtpy_contact = json::decode("{}").unwrap();
    SpaceApiClient{
      last_fetch: UTC::now(),
      status: Status::new("no space", "no logo", "no url", empty_location, emtpy_contact, vec![]),
    }
  }
  
  pub fn init() -> SpaceApiClient {
    let mut s = SpaceApiClient::new();
    
    s.fetch_from_api();
    
    s
  }
  
  pub fn fetch_aggregated_status(&mut self) -> ::std::result::Result<String, String> {
    self.fetch_from_api();
    
    aggregate_status(self.status.clone())
  }

  fn fetch_from_api(&mut self) {
    if let Ok(status) = fetch_status() {
      self.status = status;
      self.last_fetch = UTC::now();
    }
  }
  
  pub fn get_tmp_path_for_webcam(&self, url :&String) -> Result<String,io::Error> {
    let dir = env::temp_dir().join("coredump_bot").join("get_tmp_path_for_webcam");
    let path = env::temp_dir().join("coredump_bot").join("get_tmp_path_for_webcam").join( self.basename(url) );
    let path = path.as_path();
    
    
    try!(fs::create_dir_all(dir));
    
    let mut f = try!(File::create(&path));
    
    
    let bin = try!(fetch_binary(url));
    
    
    try!(f.write_all(&bin));
    try!(f.sync_all());
    
    Ok::<String,io::Error>(format!("{}", path.to_str().unwrap()))
  }
  
  pub fn basename<'a>(&self, path :&'a String) -> &'a str {
    match path.rfind('/') {
      Some(p) => &path[p+1..],
      None => path,
    }
  }
  
  pub fn get_webcams(&self) -> Vec<String> {
    match self.status.cam.clone() { // FIXME this clone is very ugly, because it should not be needed here.
      Value(webcams) => webcams, // FIXME the clone should be here.
      Absent => vec![],
    }
  }
  
  pub fn get_location(&self) -> Location {
    self.status.location.clone()
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
        Ok(len) => {
          
          match json::decode( &*body ) {
            Err(e) => Err(format!("unable to parse server response of size {}: {:?}", len, e)),
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
  
  if size > 0 {
    Ok(v)
  } else {
    Err(io::Error::new(io::ErrorKind::Interrupted, format!("fetch_binary({}) empty response", url)))
  }
}


fn aggregate_status(status : Status) -> Result<String, String> {
  let msg : Option<String> = status.state.message.into();
  let mut r = format!("{}\n\n", msg.unwrap_or( status.space ));
  
  if let Value(sensors) = status.sensors {
    let pnp = match extract_sensors(sensors.people_now_present, "people_now_present") {
      Ok(o) => {
        o.into_iter().map(|e| {
  format!("In {} are {} people.\n", e.location.unwrap_or_else(|| "unknown".into()), e.value)
        }).collect()
      },
      Err(e) => e,
    };
    r = r + &pnp + "\n";
    
    let temp = match extract_sensors(sensors.temperature, "temperature") {
      Ok(o) => {
        o.into_iter().map(|e| {
          let name : Option<String> = e.name.into();
          format!("{} ({}): {}{}\n", name.unwrap_or("Unidentified Sensor".into()), e.location, e.value, e.unit)
        }).collect()
      },
      Err(e) => e,
    };
    r = r + &temp;
    
  } else {
    r = r + "SpaceAPI response contains no sensors";
    return Err(r);
  }
  
  
  Ok(r)
}

fn extract_sensors<T>(sensors : Optional<Vec<T>>, name : &str) -> Result<Vec<T>, String> {
  match sensors {
    Absent => Err(format!("SpaceAPI response contains no {} sensors.", name)),
    Value(sensors) => {
      if sensors.is_empty() {
        Err(format!("SpaceAPI response has an empty list of {} sensors.", name))
      } else {
        Ok(sensors)
      }
    }
  }
}























#[cfg(test)]
mod test {
  use super::{SpaceApiClient, aggregate_status};
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
  fn aggregate_status_closed() {
    let n = aggregate_status( good_response() );
    
    assert_eq!( Ok("Open every Monday from 20:00\n\nIn Hackerspace are 0 people.\n\nRaspberry CPU (Hackerspace): 55.7\u{b0}C\n".into()), n );
  }
  
  #[test]
  fn aggregate_status_6() {
    let n = aggregate_status( minimal_response() );
    
    assert_eq!( Ok("Open every Monday from 20:00\n\nSpaceAPI response contains no people_now_present sensors.\nRaspberry CPU (Hackerspace): 55.7\u{b0}C\n".into()), n );
  }
  
  #[test]
  fn aggregate_status_err() {
    let n = aggregate_status( cam_response() );
    
    assert_eq!( Ok("6 people here right now\n\nIn Hackerspace are 6 people.\n\nRaspberry CPU (Hackerspace): 48.7\u{b0}C\n".into()), n );
  }
}


#[cfg(test)]
mod test_basename {
  use super::SpaceApiClient;

  #[test]
  fn path() {
    let sac = SpaceApiClient::new();
    assert_eq!(sac.basename(&"/usr/bin/bash".into()), format!("bash"));
  }

  #[test]
  fn url() {
    let sac = SpaceApiClient::new();
    assert_eq!(sac.basename(&"http://coredump.ch/f/index.html".into()), "index.html");
  }

  #[test]
  fn plain() {
    let sac = SpaceApiClient::new();
    assert_eq!(sac.basename(&"foobar".into()), "foobar");
  }

  #[test]
  fn none() {
    let sac = SpaceApiClient::new();
    assert_eq!(sac.basename(&"/bin/".into()), "");
  }
}
