//! Processes the grammar

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

/// Returns everything from "src/user_input_compiler.rs" util it passes an empty Line.
pub fn get_grammar_string() -> String {
  let f = match File::open("src/user_input_compiler.rs") {
    Ok(f) => f,
    Err(e) => return format!("grammar::open error: {:?}", e),
  };
  let mut reader = BufReader::new(f);
  let mut grammar = String::new();
  let mut max_lines_to_read = 100;
  let mut skipping_header = true;
  
  loop {
    let mut buffer = String::new();
    // read a line into buffer
    match reader.read_line(&mut buffer) {
      Ok(_) => {},
      Err(e) => return format!("grammar::read_line error: {:?}", e),
    }


    if buffer == "//! ```\n" {
      skipping_header = false;
    }

    if max_lines_to_read > 0 {
      max_lines_to_read -= 1;
    } else {
      break;
    }

    if skipping_header {
      continue;
    }

    if buffer == "\n" { // Block Change
      return grammar;
    }

    let (_, line) = buffer.split_at(4);
    grammar = grammar + line;
  }
  
  grammar
}


#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn print_grammar_string() {
    let g :String = get_grammar_string();
    println!("alles: {:?}", g);
    // TODO real Test
    assert!(g.is_empty() == false);
    assert_eq!(format!(":= *\n```\n"), g[(g.len()-9)..]);
  }
}
