//extern crate spaceapi;
use rustc_serialize::json;
use hyper::Client;
use std::io::Read;
use spaceapi::Optional::{Value,Absent};
use spaceapi::{Status};
use spaceapi::sensors::{PeopleNowPresentSensor};

struct SpaceApiClient {
  last_fetch: i64,
}

pub fn fetch_people_now_present() -> ::std::result::Result<u64, String> {
  let client = Client::new();

  match client.get("https://status.coredump.ch/").send() {
    Err(e) => Err(format!("client.get() error:\n{}", e)),
    Ok(mut res) => {
      
      let mut body = String::new();
      match res.read_to_string(&mut body) {
        Err(e) => { Err(format!("unable to connect to server, try again later:\n{}\n{}", e, body)) },
        Ok(_/*len*/) => {
          
          
          match json::decode( &*body ) {
            Err(e) => Err(format!("unable to parse server response: {:?}", e)),
            Ok(status) => {
              let status :Status = status;
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
          }
          
        }
      }
    }
  }
}




#[cfg(test)]
mod test {
  use super::*;
  use spaceapi::{Status, Location, Contact};
  use spaceapi::optional::Optional;
  use spaceapi::sensors::{TemperatureSensor, PeopleNowPresentSensor};
  use rustc_serialize::json::{self, Json};
  
  fn good_response_string() -> &'static str {
    "{\"api\":\"0.13\",\"contact\":{\"email\":\"vorstand@lists.coredump.ch\",\"foursquare\":\"525c20e5498e875d8231b1e5\",\"irc\":\"irc://freenode.net/#coredump\",\"twitter\":\"@coredump_ch\"},\"feeds\":{\"blog\":{\"type\":\"rss\",\"url\":\"https://www.coredump.ch/feed/\"}},\"issue_report_channels\":[\"email\",\"twitter\"],\"location\":{\"address\":\"Spinnereistrasse 2, 8640 Rapperswil, Switzerland\",\"lat\":47.22936,\"lon\":8.82949},\"logo\":\"https://www.coredump.ch/logo.png\",\"projects\":[\"https://www.coredump.ch/projekte/\",\"https://discourse.coredump.ch/c/projects\",\"https://github.com/coredump-ch/\"],\"sensors\":{\"people_now_present\":[{\"location\":\"Hackerspace\",\"value\":0}],\"temperature\":[{\"location\":\"Hackerspace\",\"name\":\"Raspberry CPU\",\"unit\":\"°C\",\"value\":55.7}]},\"space\":\"coredump\",\"spacefed\":{\"spacenet\":false,\"spacephone\":false,\"spacesaml\":false},\"state\":{\"message\":\"Open every Monday from 20:00\",\"open\":false},\"url\":\"https://www.coredump.ch/\"}"
  }
  
  fn minimal_response_string() -> &'static str {
    "{\"api\":\"0.13\",\"contact\":{\"email\":\"vorstand@lists.coredump.ch\",\"foursquare\":\"525c20e5498e875d8231b1e5\",\"irc\":\"irc://freenode.net/#coredump\",\"twitter\":\"@coredump_ch\"},\"feeds\":{\"blog\":{\"type\":\"rss\",\"url\":\"https://www.coredump.ch/feed/\"}},\"issue_report_channels\":[\"email\",\"twitter\"],\"location\":{\"lat\":47.22936,\"lon\":8.82949},\"logo\":\"https://www.coredump.ch/logo.png\",\"projects\":[\"https://www.coredump.ch/projekte/\",\"https://discourse.coredump.ch/c/projects\",\"https://github.com/coredump-ch/\"],\"sensors\":{\"people_now_present\":[{\"location\":\"Hackerspace\",\"value\":0}],\"temperature\":[{\"location\":\"Hackerspace\",\"name\":\"Raspberry CPU\",\"unit\":\"°C\",\"value\":55.7}]},\"space\":\"coredump\",\"spacefed\":{\"spacenet\":false,\"spacephone\":false,\"spacesaml\":false},\"state\":{\"message\":\"Open every Monday from 20:00\",\"open\":false},\"url\":\"https://www.coredump.ch/\"}"
  }
  
  #[test]
  fn api_response() {
    let resp :Status = json::decode(&good_response_string()).unwrap();
  }
  
  #[test]
  fn api_response_minimal() {
    let resp :Status = json::decode(&minimal_response_string()).unwrap();
  }
}