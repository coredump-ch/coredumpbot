//! # CoredumpBot
//! 
//! Works with status.coredump.ch and api.telegram.org

extern crate telegram_bot;
extern crate hyper;
extern crate rustc_serialize;
extern crate spaceapi;
extern crate env_logger;
#[macro_use] extern crate log;

use telegram_bot::{Api, ListeningMethod, MessageType, ListeningAction};
use rustc_serialize::json::Json;

mod user_input_compiler;
use user_input_compiler::Input;

mod spaceapi_client;
use spaceapi_client::fetch_people_now_present;

mod grammar;

fn main() {
    env_logger::init().unwrap();
    let max_backoff_seconds = 128;
    let min_backoff_seconds = 1;
    let mut backoff_seconds = min_backoff_seconds;
    let mut last_processed_message_id = 0;
    
    loop {
        // Create bot, test simple API call and print bot information
        let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
        info!("getMe: {:?}", api.get_me());
        let mut listener = api.listener(ListeningMethod::LongPoll(None));
        
        let path_to_picture :String = "rust-logo-blk.png".to_string();
        let caption_to_picture :String = "Rust Logo".to_string();

        // Fetch new updates via long poll method
        let res = listener.listen(|u| {
            // Restore backoff_seconds, since it works agan
            backoff_seconds = min_backoff_seconds;
            
            
            // If the received update contains a message...
            if let Some(m) = u.message {
                // Discard Messages from Groups the Bot is no longer part of
                if m.chat.id() == last_processed_message_id {
                    warn!("Dropped Message: {:?}", m);
                    return Ok(ListeningAction::Continue);
                } else {
                    last_processed_message_id = m.chat.id();
                }
                
                let name = m.from.first_name;

                // Match message type
                match m.msg {
                    MessageType::Text(t) => {
                        // Print received text message to stdout
                        info!("<{}> {}", name, t);
                        let t = t.replace("@CoreDumpBot", "");
                        let ts:String = format!("{}", t.trim() );
                        
                        match Input::from(ts) {
                        Input::WebCam{ nth } => { 
                            try!(api.send_photo(
                                    m.chat.id(),
                                    path_to_picture.clone(), // Path
                                    Some(caption_to_picture.clone()), // caption
                                    None, // reply_to_message_id
                                    None  // reply_markup
                            ));
                        },
                        Input::Help => {
                            try!(api.send_message(
                                    m.chat.id(),
                                    format!("No such help 😜\nuse /webcam for a snapshot of the 3d printer.\nuse /crowd or /status for an update on people now present\nuse /grammar to receive the spec"),
                                    None, None, None
                            ));
                        },
                        Input::Status => {
                            let s = match fetch_people_now_present() {
                                Ok(people_now_present) if people_now_present > 1 =>  format!("Coredump is open\n{} people are present!", people_now_present),
                                Ok(people_now_present) if people_now_present == 1 => format!("Coredump is open\nOne person is present!"),
                                Ok(_) => format!("Coredump is closed\nNobody here right now."),
                                Err(e) => format!("An error occured 😕\n{}", e),
                            };
                            try!(api.send_message(
                                    m.chat.id(),
                                    s,
                                    None, None, None
                            ));
                        },
                        Input::Start => {
                            try!(api.send_message(
                                    m.chat.id(),
                                    format!("Welcome to CoredumpBot\nuse /help for a some commands."),
                                    None, None, None
                            ));
                        },
                        Input::Version => {
                            try!(api.send_message(
                                    m.chat.id(),
                                    format!("Version: {}", env!("CARGO_PKG_VERSION")),
                                    None, None, None
                            ));
                        },
                        Input::Grammar => {
                            try!(api.send_message(
                                    m.chat.id(),
                                    grammar::get_grammar_string(),
                                    None, None, None
                            ));
                        },
                        Input::InvalidSyntax( msg ) => {
                            try!(api.send_message(
                                    m.chat.id(),
                                    format!("InvalidSyntax: {}\ntry /grammar", msg),
                                    None, None, None
                            ));
                        },
                        _ => {
                            try!(
                                api.send_message(
                                    m.chat.id(),
                                    format!("Unknown Command ... try /help"),
                                    None, None, None)
                            );
                        }, 
                        }
                    },
                    _ => {
                        try!(
                            api.send_message(
                                m.chat.id(),
                                format!("Unknown Command ... try /help"),
                                None, None, None)
                        );
                    }
                }
            }

            // If none of the "try!" statements returned an error: It's Ok!
            Ok(ListeningAction::Continue)
        });

        if let Err(e) = res {
            warn!("An error occured: {}\nSleeping for {} Seconds", e, backoff_seconds);
            // Rest for 10 Seconds
            std::thread::sleep_ms(backoff_seconds * 1000);
            
            if backoff_seconds < max_backoff_seconds {
                backoff_seconds *= 2;
            }
        }
    }
}


