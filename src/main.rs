//! # CoredumpBot
//! 
//! Works with status.coredump.ch and api.telegram.org

extern crate telegram_bot;
extern crate hyper;
extern crate rustc_serialize;
extern crate spaceapi;
extern crate time;

use telegram_bot::{Api, ListeningMethod, Message, MessageType, ListeningAction};
use rustc_serialize::json::Json;

mod user_input_compiler;
use user_input_compiler::Input;

mod spaceapi_client;

mod grammar;

fn main() {
    let max_backoff_seconds = 128;
    let min_backoff_seconds = 1;
    let mut backoff_seconds = min_backoff_seconds;
    let mut sac = spaceapi_client::SpaceApiClient::new();
    
    loop {
        // Create bot, test simple API call and print bot information
        let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
        println!("getMe: {:?}", api.get_me());
        let mut listener = api.listener(ListeningMethod::LongPoll(None));
        
        let path_to_picture :String = "rust-logo-blk.png".to_string();
        let caption_to_picture :String = "Rust Logo".to_string();

        // Fetch new updates via long poll method
        let res = listener.listen(|u| {
            // Restore backoff_seconds, since it works agan
            backoff_seconds = min_backoff_seconds;
            
            
            // If the received update contains a message...
            if let Some(m) = u.message {
                let name = m.from.first_name;

                // Match message type
                match m.msg {
                    MessageType::Text(t) => {
                        // Print received text message to stdout
                        println!("<{}> {}", name, t);
                        let t = t.replace("@CoreDumpBot", "");
                        let ts:String = format!("{}", t.trim() );
                        
                        match Input::from(ts) {
                        Input::WebCam{ nth } => {
                            let mut w = sac.get_webcams();
                            match nth {
                                /*Some(nth) if nth < w.len() as u64 => {
                                    let e = w.get(nth as usize).unwrap();
                                    w.clear();
                                    w.push(format!("{}",*e));
                                },
                                Some(nth) => { try!(send(&api, m, format!("invalid OptionalInteger: {}", nth))); return Ok(ListeningAction::Continue); },*/
                                Some (nth) => match w.get(nth as usize) {
                                    Some(e) => {},
                                    None => {},
                                },
                                None => {},
                            };
                            
                            for pic_path in w {
                                let caption = sac.basename(&pic_path);
                                match sac.get_tmp_path_for_webcam(&pic_path) {
                                    Ok(pic_tmp_path) => {
                                        try!(api.send_photo(
                                                m.chat.id(),
                                                pic_tmp_path, // Path
                                                Some(caption), // caption
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
                            try!(api.send_message(
                                    m.chat.id(),
                                    format!("No such help 😜\nuse /webcam for a snapshot of the 3d printer.\nuse /crowd or /status for an update on people now present\nuse /grammar to receive the spec"),
                                    None, None, None
                            ));
                        },
                        Input::Status => {
                            let s = match sac.fetch_people_now_present() {
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
            println!("An error occured: {}\nSleeping for {} Seconds", e, backoff_seconds);
            // Rest for 10 Seconds
            std::thread::sleep_ms(backoff_seconds * 1000);
            
            if backoff_seconds < max_backoff_seconds {
                backoff_seconds *= 2;
            }
        }
    }
}

fn send(api:&Api, m: Message, message :String) -> Result<Message,telegram_bot::Error> {
    api.send_message(
        m.chat.id(), // chat_id                  : Integer
        message,     // text                     : String
        None,        // disable_web_page_preview : Option<bool>
        None,        // reply_to_message_id      : Option<Integer>
        None)        // reply_markup             : Option<ReplyMakrup>
}

