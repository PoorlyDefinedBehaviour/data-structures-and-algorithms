use std::collections::HashMap;

/**
 * time O(2n) -> O(n)
 * space O(n)
 * */
fn first_non_repeating_character(string: String) -> Option<char> {
  let mut character_count_table = HashMap::new();

  for character in string.chars() {
    if character_count_table.contains_key(&character) {
      let x = character_count_table.get_mut(&character).unwrap();
      *x += 1;
    } else {
      character_count_table.insert(character, 1);
    }
  }

  for (character, count) in character_count_table {
    if count == 1 {
      return Some(character);
    }
  }

  None
}

fn main() {
  println!(
    "{:?}",
    first_non_repeating_character(String::from("aaabcbdc")).expect("All characteres are repeating")
  );
}
