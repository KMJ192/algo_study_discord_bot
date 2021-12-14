use std::collections::{HashMap, HashSet};
mod matching_algorithm;

use matching_algorithm::matching_algorithm;

fn make_array(members: String) -> (Vec<String>, usize) {
  let split: std::str::SplitWhitespace = members.split_whitespace();
  let arr = split.collect::<Vec<&str>>();
  let mut re = vec![];
  for s in arr {
    re.push(String::from(s));
  }
  let cast = re[re.len() - 1].parse::<usize>();

  // default size = 2;
  let mut size = 2;

  match cast {
    Ok(r) => {
      size = r;
      re.pop();
    },
    Err(_) => size = 2,
  };

  (re, size)
}

fn output_str(result: HashMap<String, HashSet<String>>) -> String {
  let mut make = String::new();
  let ar = String::from(" => ");

  for d in result {
    make.push_str(&*d.0);
    make.push_str(&ar);
    for e in d.1 {
      make.push_str(&*e);
      make.push_str(&*String::from(" "));
    }
    make.push('\n');
  }
  make
}

pub fn run(members: String) -> String {
  let (arr, pair_num) = make_array(members);
  if arr.len() < pair_num { return String::from("잘못된 입력입니다. (입력 된 숫자가 매칭 유저수보다 큽니다.)"); }

  let mut result = HashMap::new();
  
  // retry
  for _ in 0..100 {
    let tmp = arr.clone();
    result = matching_algorithm(tmp, pair_num);
    let mut s = true;

    for r in &result {
      if r.1.len() < pair_num {
        s = false;
        break;
      }
    }

    if s == true { break; }
  }
  let r = output_str(result);
  if r == "" {
    return String::from("ERROR-S000");
  }
  r
}

use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

const RES_MATCHING: &str = "
  How to input
  !matching MemberName(Divide into space) matching count
  ex) !matching name1 name2 name3 name4 name5 name6 2
";

#[command]
async fn matching(ctx: &Context, msg: &Message) -> CommandResult {
  let input_msg = msg.content.to_string();
  if input_msg.len() > 9 {
    let members = String::from(&input_msg[10..]);
    let result = run(members);
    msg.channel_id.say(&ctx.http, result).await?;
  } else {
    msg.channel_id.say(&ctx.http, RES_MATCHING).await?;
  }
  
  Ok(())
}

#[group]
#[commands(matching)]
pub struct Matching;