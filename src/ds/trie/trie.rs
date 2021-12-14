use std::collections::HashMap;

#[derive(Debug)]
struct Node{
  b: bool,
  w: String,
  n: HashMap<char, Node>
}
impl Node{
  fn new() -> Self {
    Node {
      b: false,
      w: String::from(""),
      n: HashMap::new()
    }
  }
}

#[derive(Debug)]
struct Trie {
  root: Node
}

impl Trie {
  fn moving<T>(t : T) -> T{
    t
  }

  fn new() -> Self {
    Trie{ 
      root: Node::new() 
    }
  }

  fn insert(&mut self, word: String) {
    let mut current = &mut self.root;
    for c in word.chars(){
        current = Trie::moving(current).n.entry(c).or_insert(Node::new());
    }
    if !current.b{
        current.b = true;
        current.w = word;
    }
  }
  fn search(&mut self, word: String) -> bool {
    let mut current = &mut self.root;
    for c in word.chars(){
        if let Some(_) = current.n.get(&c){
            current = Trie::moving(current).n.entry(c).or_insert(Node::new());
        }else{
            return false;
        }
    }
    current.b
  }
  fn starts_with(&mut self, prefix: String) -> bool {
    let mut current = &mut self.root;
    for w in prefix.chars(){
      if let Some(_) = current.n.get(&w){
        current = Trie::moving(current).n.entry(w).or_insert(Node::new());
      }else{
        return false;
      }
    }
    true
  }
}

pub fn build_trie(param: &str) -> String {
  let split: std::str::SplitWhitespace = param.split_whitespace();
  let arr = split.collect::<Vec<&str>>();

  let mut trie = Trie::new();
  for i in 1..arr.len() {
    trie.insert(String::from(arr[i]));
  }

  let mut result = format!("```\n{:#?}\n```", trie);
  if result.len() > 2000 {
    result = String::from("Please use a shorter example. (More than the inputable length in discord.)");
  }
  
  result
}