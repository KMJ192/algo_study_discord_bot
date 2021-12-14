use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

mod trie;

use trie::build_trie;

const RES_TRIE_RUN: &str = "
  How to input
  !trie_run InputString(Divide into space)
  ex) !trie_run app apple as sc
";

#[command]
async fn trie_run(ctx: &Context, msg: &Message) -> CommandResult {
  let input_msg = msg.content.to_string();
  if input_msg.len() > 9 {
    let result = build_trie(&input_msg);
    msg.channel_id.say(&ctx.http, result).await?;
  } else {
    msg.channel_id.say(&ctx.http, RES_TRIE_RUN).await?;
  }
  Ok(())
}

const RES_TRIE_CODE: &str = "
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

#[command]
async fn trie_code(ctx: &Context, msg: &Message) -> CommandResult {
  msg.channel_id.say(&ctx.http, RES_TRIE_CODE).await?;
  Ok(())
}


#[group]
#[commands(trie_run, trie_code)]
pub struct Trie;