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
  let mut i = 100;
  
  loop {
    let mut buffer = String::new();
    // read a line into buffer
    match reader.read_line(&mut buffer) {
      Ok(_) => {},
      Err(e) => return format!("grammar::read_line error: {:?}", e),
    }
    
    grammar = grammar + &*buffer;
    if buffer == "\n" {
      return grammar;
    }
    
    if i > 0 {
      i -= 1;
    } else {
      break;
    }
  }
  
  grammar
}


#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn print_grammar_string() {
    let g :String = get_grammar_string();
    info!("alles: {:?}", g);
    // TODO real Test
    assert!(g.is_empty() == false);
    //assert!("\n\n".is_suffix_of(g));
  }
}
