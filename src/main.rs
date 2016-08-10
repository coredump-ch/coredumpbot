//! # CoredumpBot
//! 
//! Checks https://status.crdmp.ch/ and communicates via https://api.telegram.org/ to the user.

extern crate telegram_bot;
extern crate hyper;
extern crate rustc_serialize;
extern crate spaceapi;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate chrono;

use telegram_bot::{Api, ListeningMethod, Message, MessageType, ListeningAction};

pub mod user_input_compiler;
use user_input_compiler::Input;

pub mod spaceapi_client;

pub mod grammar;

pub mod cached_value;
pub mod user_settings;

use std::time::Duration;

fn main() {
  env_logger::init().unwrap();
  let max_backoff_seconds = Duration::from_secs(128);
  let min_backoff_seconds = Duration::from_secs(1);
  let mut backoff_seconds = min_backoff_seconds;
  let mut sac = spaceapi_client::SpaceApiClient::init();
  let mut last_processed_message_id = 0;

  loop {
    // Create bot, test simple API call and print bot information
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    let me = match api.get_me() {
      Ok(me) => me,
      Err(e) => {
        error!("Unable to get_me(): {:?}", e);
        continue;
      }
    };
    info!("getMe: {:?}", me);
    let mut listener = api.listener(ListeningMethod::LongPoll(None));


    // Fetch new updates via long poll method
    let res = listener.listen(|u| {
      // Restore backoff_seconds, since it works agan
      backoff_seconds = min_backoff_seconds;


      // If the received update contains a message...
      if let Some(m) = u.message {
        // Discard Messages from Groups the Bot is no longer part of
        if m.message_id == last_processed_message_id {
          warn!("Dropped Message: {:?}", m);
          return Ok(ListeningAction::Continue);
        } else {
          last_processed_message_id = m.message_id;
        }

        let name = m.from.first_name;

        // Match message type
        match m.msg {
          MessageType::Text(t) => {
              // Print received text message to stdout
              info!("<{}> {}", name, t);
              let t = t.replace(&* format!("@{}", me.first_name), "");
              let ts:String = format!("{}", t.trim() );

              match Input::from(ts) {
                Input::WebCam{ nth } => {
                  let cams = sac.get_webcams();

                  let no_filter = if let Some(nth) = nth {
                    if nth >= cams.len() {
                      try!(send_message(&api, m.chat.id(),
                        format!("You requested the webcam #{}, but there are just {}", nth, cams.len())
                      ));
                      return Ok(ListeningAction::Continue);
                    }

                    false
                  } else {
                    true
                  };

                  let mut n : usize = 0;
                  let w = cams.iter().filter(|_| {
                      let b = no_filter || Some(n) == nth;
                      n += 1;
                      b
                    });

                  for pic_path in w {
                    let caption = sac.basename(&pic_path);
                    match sac.get_tmp_path_for_webcam(&pic_path) {
                      Ok(pic_tmp_path) => {
                        try!(api.send_photo(
                                m.chat.id(),
                                pic_tmp_path, // Path
                                Some(caption.into()), // caption
                                None, // reply_to_message_id
                                None  // reply_markup
                        ));
                      },
                      Err(e) => {
                        println!("WebCam({:?}) Error: {:?}",nth, e);
                        // TODO send Error
                      },
                    }
                  }
                },


                Input::Help => {
                  try!(send_message(&api, m.chat.id(),
                          "No such help 😜\nuse /webcam for a snapshot of the 3d printer.\nuse /crowd or /status for an update on people now present\nuse /grammar to receive the spec".into())
                  );
                },


                Input::Status => {
                  let s = match sac.fetch_aggregated_status() {
                    Ok(people_now_present) => people_now_present,
                    Err(e) => format!("An error occured 😕\n{}", e),
                  };
                  try!(send_message(&api, m.chat.id(),s));
                },


                Input::Start => {
                  try!(send_message(&api, m.chat.id(),
                          "Welcome to CoredumpBot\nuse /help for a some commands.".into())
                  );
                },


                Input::Version => {
                  try!(send_message(&api, m.chat.id(),
                          format!("Version: {}", env!("CARGO_PKG_VERSION")))
                  );
                },


                Input::Grammar => {
                  try!(
                    api.send_message(
                      m.chat.id(),                                    // chat_id                  : Integer
                      grammar::get_grammar_string(),                  // text                     : String
                      Some(telegram_bot::types::ParseMode::Markdown), // parse_mode               : Option<ParseMode>
                      Some(true),                                     // disable_web_page_preview : Option<bool>
                      None,                                           // reply_to_message_id      : Option<Integer>
                      None)                                           // reply_markup             : Option<ReplyMakrup>
                  );
                },


                Input::Location => {
                  let loc = sac.get_location();
                  try!(api.send_location(
                          m.chat.id(),
                          loc.lat as f32, loc.lon as f32,
                          None, None
                  ));
                },


                Input::InvalidSyntax( msg ) => {
                  if m.chat.is_user() {
                      try!(send_message(&api, m.chat.id(),
                              format!("InvalidSyntax: {}\ntry /grammar", msg)
                      ));
                  }
                },


                Input::Subscribe{ sensor, duration  } => {
                  try!(send_message(&api, m.chat.id(),
                          format!("Sensor: {:?}; Duration: {:?}", sensor, duration)
                  ));
                }


                Input::Cancel => {

                }


                /*_ => {
                  if m.chat.is_user() {
                      try!(
                          send_message(&api, m.chat.id(),
                              "Unknown Command ... try /help".into())
                      );
                  }
                },*/
              }
          },
          _ => {
            if m.chat.is_user() {
              try!(
                send_message(&api, m.chat.id(),
                    "Unknown Command ... try /help".into())
              );
            }
          }
        }
      }

      // If none of the "try!" statements returned an error: It's Ok!
      Ok(ListeningAction::Continue)
    });

    if let Err(e) = res {
      warn!("An error occured: {}\nSleeping for {} Seconds", e, backoff_seconds.as_secs());
      // Rest for 10 Seconds
      std::thread::sleep(backoff_seconds);

      if backoff_seconds < max_backoff_seconds {
        backoff_seconds = backoff_seconds * 2u32;
      }
    }
  }
}

fn send_message(api: &Api, chat_id: i64, message: String) -> Result<Message,telegram_bot::Error> {
  api.send_message(
      chat_id,     // chat_id                  : Integer
      message,     // text                     : String
      None,        // parse_mode               : Option<ParseMode>
      None,        // disable_web_page_preview : Option<bool>
      None,        // reply_to_message_id      : Option<Integer>
      None)        // reply_markup             : Option<ReplyMakrup>
}

