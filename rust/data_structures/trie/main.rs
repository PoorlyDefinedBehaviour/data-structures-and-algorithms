use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
  pub is_end_of_word: bool,
  pub children: HashMap<char, TrieNode>,
}

impl TrieNode {
  pub fn new() -> Self {
    TrieNode {
      is_end_of_word: false,
      children: HashMap::new(),
    }
  }
}

#[derive(Debug)]
pub struct Trie {
  root: TrieNode,
}

impl Trie {
  pub fn new() -> Self {
    Trie {
      root: TrieNode::new(),
    }
  }

  pub fn insert(&mut self, word: &String) {
    let mut current_node = &mut self.root;

    for character in word.chars() {
      if !current_node.children.contains_key(&character) {
        current_node.children.insert(character, TrieNode::new());
      }
      current_node = current_node.children.get_mut(&character).unwrap();
    }

    current_node.is_end_of_word = true;
  }

  pub fn contains(&self, word: &String) -> bool {
    if word.is_empty() {
      return false;
    }

    let mut current_node = &self.root;

    for character in word.chars() {
      match current_node.children.get(&character) {
        None => return false,
        Some(child) => current_node = child,
      }
    }

    current_node.is_end_of_word
  }
}

fn main() {
  println!("hello")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn contains() {
    let tests = vec![
      (vec![], "hello", false),
      (vec![], "", false),
      (vec!["hello", "world"], "one", false),
      (vec!["hello", "world"], "hel", false),
      (vec!["hello", "world"], "w", false),
      (vec!["hello", "world"], "hello", true),
      (vec!["hello", "world", "one", "a"], "a", true),
      (vec!["hello", "world", "one", "a"], "one", true),
      (vec!["hello", "hello", "one", "a"], "h", false),
    ];

    for (words_in_trie, target, expected) in tests {
      let mut trie = Trie::new();

      for word in words_in_trie {
        trie.insert(&word.to_owned());
      }

      let actual = trie.contains(&target.to_owned());

      assert_eq!(expected, actual);
    }
  }
}
