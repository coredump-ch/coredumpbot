use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

/// Returns everything from "src/user_input_compiler.rs" util it passes an empty Line.
pub fn get_grammer_string() -> String {
  let f = match File::open("src/user_input_compiler.rs") {
    Ok(f) => { println!("grammer::File::open"); f },
    Err(e) => return format!("grammer::open error: {:?}", e),
  };
  let mut reader = BufReader::new(f);
  let mut grammer = String::new();
  let mut i = 100;
  
  loop {
    let mut buffer = String::new();
    // read a line into buffer
    match reader.read_line(&mut buffer) {
      Ok(v) => { println!("grammer::read_line: {:?}", v); },
      Err(e) => return format!("grammer::read_line error: {:?}", e),
    }
    println!("buffer: {:?}", buffer);
    grammer = grammer + &*buffer;
    if buffer == "\n" {
      return grammer;
    }
    
    if i > 0 {
      i -= 1;
    } else {
      break;
    }
  }
  
  grammer
}


#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn print_grammer_string() {
    let g :String = get_grammer_string();
    println!("alles: {:?}", g);
    // TODO real Test
    assert!(g.is_empty() == false);
    //assert!("\n\n".is_suffix_of(g));
  }
}
