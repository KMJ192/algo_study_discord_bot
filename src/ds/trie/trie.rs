use std::collections::HashMap;

use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

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
struct TrieDs {
  root: Node
}

impl TrieDs {
  fn moving<T>(t : T) -> T{
    t
  }

  fn new() -> Self {
    TrieDs{ 
      root: Node::new() 
    }
  }

  fn insert(&mut self, word: String) {
    let mut current = &mut self.root;
    for c in word.chars(){
        current = TrieDs::moving(current).n.entry(c).or_insert(Node::new());
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
            current = TrieDs::moving(current).n.entry(c).or_insert(Node::new());
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
        current = TrieDs::moving(current).n.entry(w).or_insert(Node::new());
      }else{
        return false;
      }
    }
    true
  }
}

fn build_trie(param: &str) -> String {
  let split: std::str::SplitWhitespace = param.split_whitespace();
  let arr = split.collect::<Vec<&str>>();

  let mut trie = TrieDs::new();
  for i in 1..arr.len() {
    trie.insert(String::from(arr[i]));
  }

  let mut result = format!("```\n{:#?}\n```", trie);
  if result.len() > 2000 {
    result = String::from("Please use a shorter example. (More than the inputable length in discord.)");
  }
  
  result
}

#[command]
async fn trie_run(ctx: &Context, msg: &Message) -> CommandResult {
  let input_msg = msg.content.to_string();
  if input_msg.len() > 9 {
    let result = build_trie(&input_msg);
    msg.channel_id.say(&ctx.http, result).await?;
  } else {
    let trie_info = "
How to input
!trie_run InputString(Divide into space)
ex) !trie_run app apple as sc
  ";
    msg.channel_id.say(&ctx.http, trie_info).await?;
  }
  Ok(())
}

#[command]
async fn trie_code(ctx: &Context, msg: &Message) -> CommandResult {
  let code = "
  ```ts
  // ts 코드 (언어별 코드는 추후 업데이트)
  export class TrieNode {
    public isWord: boolean;
  
    public word: string;
  
    public next: {
      [key: string]: TrieNode;
    };
  
    constructor() {
      this.isWord = false;
      this.next = {};
      this.word = '';
    }
  }
  
  export class TrieDataStructure {
    private root: TrieNode;
  
    constructor() {
      this.root = new TrieNode();
    }
  
    public insert(word: string) {
      let curNode: TrieNode = this.root;
      for (let i: number = 0; i < word.length; i++) {
        const c: string = word[i];
        if (!curNode.next[c]) {
          curNode.next[c] = new TrieNode();
        }
        curNode = curNode.next[c];
      }
      curNode.word = word;
      curNode.isWord = true;
    }
  
    public search(word: string): boolean {
      let curNode: TrieNode = this.root;
      for (let i: number = 0; i < word.length; i++) {
        curNode = curNode.next[word.charAt(i)];
        if (!curNode) return false;
      }
      return curNode.isWord;
    }
  
    public startsWith(prefix: string): boolean {
      let curNode = this.root;
      for (let i: number = 0; i < prefix.length; i++) {
        curNode = curNode.next[prefix.charAt(i)];
        if (!curNode) return false;
      }
      return true;
    }
  }
  ```
  ";
  msg.channel_id.say(&ctx.http, code).await?;
  Ok(())
}

#[group]
#[commands(trie_run, trie_code)]
pub struct Trie;