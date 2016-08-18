//! Processes the grammar

/// Returns everything from "src/user_input_compiler.rs" util it passes an empty Line.
pub fn get_grammar_string() -> String {
  let file = include_str!("../src/user_input_compiler.rs");
  let mut grammar = String::new();
  let mut max_lines_to_read = 100;
  let mut skipping_header = true;
  
  for line in file.split('\n') {
    if max_lines_to_read > 0 {
      max_lines_to_read -= 1;
    } else {
      break;
    }
    
    if line == "//! ```" {
      skipping_header = false;
    }
    if skipping_header {
      continue;
    }

    if line == "" { // Block Change
      return grammar;
    }

    let (_, line) = line.split_at(4);
    grammar = grammar + line + "\n";
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
