extern crate spaceapi;
use std::io::prelude::*;
use rustc_serialize::json;
use hyper::Client;
use spaceapi::Optional::{self, Value, Absent};
use spaceapi::{Status, Sensors, Location};

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
  
  pub fn fetch_people_now_present(&mut self) -> ::std::result::Result<String, String> {
    self.fetch_from_api();
    
    extract_people_now_present(self.status.sensors.clone())
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

fn extract_people_now_present(status : Optional<Sensors>) -> Result<String, String> {
  match status { // FIXME why is this clone needed here?
    Absent => Err(format!("response contains no sensors")),
    Value(sensors) => {
      match sensors.people_now_present {
        Absent => Err(format!("response contains no sensors.people_now_present")),
        Value(sensors) => {
          if sensors.is_empty() {
            Err(format!("response.sensors.people_now_present is empty"))
          } else {
            let mut r = "".into();
            
            for pnp in sensors {
              let value_s = match pnp.value {
                0 => format!("Coredump is closed\nNobody here right now."),
                1 => format!("Coredump is open\nOne person is present!"),
                people_now_present =>  format!("Coredump is open\n{} people are present!", people_now_present),
              };
              r = format!("{}\n{}: {}", r, pnp.location.unwrap_or_else(|| "unknown".into()), value_s);
            }
            
            Ok(r)
          }
        }
      }
    }
  }
}


#[cfg(test)]
mod test {
  use super::{SpaceApiClient, extract_people_now_present};
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
    let n = extract_people_now_present( good_response().sensors.clone() );
    
    assert_eq!( Ok("\nHackerspace: Coredump is closed\nNobody here right now.".into()), n );
  }
  #[test]
  fn extract_people_now_present_6() {
    let n = extract_people_now_present( cam_response().sensors.clone() );
    
    assert_eq!( Ok("\nHackerspace: Coredump is open\n6 people are present!".into()), n );
  }
  #[test]
  fn extract_people_now_present_err() {
    let e = extract_people_now_present( minimal_response().sensors.clone() );
    
    assert_eq!( Err("response contains no sensors.people_now_present".into()), e );
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
