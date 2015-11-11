extern crate telegram_bot;

use telegram_bot::*;

fn main() {
    // Create bot, test simple API call and print bot information
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    println!("getMe: {:?}", api.get_me());
    let mut listener = api.listener(ListeningMethod::LongPoll(None));
    
    let path_to_picture :String = "rust-logo-blk.png".to_string();
    let caption_to_picture :String = "Rust Logo".to_string();

    // Fetch new updates via long poll method
    let res = listener.listen(|u| {
        // If the received update contains a message...
        if let Some(m) = u.message {
            let name = m.from.first_name;

            // Match message type
            match m.msg {
                MessageType::Text(t) => {
                    // Print received text message to stdout
                    println!("<{}> {}", name, t);
                    let t = t.replace("@CoreDumpBot", "");
                    let ts:&str = t.trim();
                    
                    match ts {
                      "/getPicture" => { 
                      try!(api.send_photo(
                                m.chat.id(),
                                path_to_picture.clone(), // Path
                                Some(caption_to_picture.clone()), // caption
                                None, // reply_to_message_id
                                None  // reply_markup
                        ));
                      },
                      "/help" => {
                        try!(api.send_message(
                                  m.chat.id(),
                                  format!("No such help 😜\nuse /getPicture"),
                                  None, None, None
                        ));
                      },
                      "/start" => {
                        try!(api.send_message(
                                  m.chat.id(),
                                  format!("Welcome to CoredumpBot\nuse /getPicture for a snapshot of the 3d printer."),
                                  None, None, None
                        ));
                      },
                      _ => { /* ignore */ }, 
                    }
                },
                _ => {
                    try!(
                        api.send_message(
                            m.chat.id(),
                            format!("Unknown Command ..."),
                            None, None, None)
                    );
                }
            }
        }

        // If none of the "try!" statements returned an error: It's Ok!
        Ok(ListeningAction::Continue)
    });

    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}
