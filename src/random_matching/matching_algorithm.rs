use std::collections::{HashMap, HashSet};
use rand::Rng;

use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

fn matching_algorithm(members_array: Vec<String>, pair_num: usize) -> HashMap<String, HashSet<String>> {
  if members_array.len() < pair_num { return HashMap::new(); }
  let mut dictionary: HashMap<String, HashSet<String>> = HashMap::new();
  let members_len = members_array.len();  
  for i in 0..members_len {
    let mem: String = members_array[i].clone();
    dictionary.insert(mem, HashSet::new());
  }
  let mut complete = false;  
  let (mut idx, limit) = (0, members_len * 10000);  
  while complete == false {
    idx += 1;
    let ran_num1 = rand::thread_rng().gen_range(0..members_len);
    let ran_num2 = rand::thread_rng().gen_range(0..members_len);
    if ran_num1 == ran_num2 { continue; }  
    let mem1 = members_array[ran_num1].clone();
    let mem2 = members_array[ran_num2].clone();  
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();  
    if let Some(p1) = dictionary.get(&*mem1) {
      if p1.len() >= pair_num {
        continue;
      }
      set1 = p1.clone();
    }
      if let Some(p2) = dictionary.get(&*mem2) {
        if p2.len() >= pair_num {
          continue;
        }
        set2 = p2.clone();
      }  
      set1.insert(mem2.clone());
      set2.insert(mem1.clone());
      dictionary.insert(mem1, set1);
      dictionary.insert(mem2, set2);  
      let mut cnt = 0;
      for d in &dictionary {
        if d.1.len() < pair_num {
          cnt += 1;
        }
      }
      if cnt <= 1 {
        complete = true;
      }
      if idx > limit {
        break;
      }
  }
  let tmp = dictionary.clone();
  for d in tmp {
    if d.1.len() < pair_num {
      let mut l = d.1.len();
      let (mut idx, limit) = (0, 10000);
      while l < pair_num {
        idx += 1;
        if let Some(p) = dictionary.get(&*d.0) {
          let mut set1 = p.clone();
          let mut set2 = HashSet::new();  
          let ran_num = rand::thread_rng().gen_range(0..members_len);
          let mem = members_array[ran_num].clone();
          if mem != *d.0 {
            l += 1;
            if let Some(pp) = dictionary.get(&*mem) {
              set2 = pp.clone();
            }  
            if let Some(_) = set1.get(&*mem){}
            else {
              set1.insert(mem.clone());
              set2.insert(d.0.clone());
              dictionary.insert(d.0.clone(), set1);
              dictionary.insert(mem, set2);
            }
          }
        }
        if idx > limit {
          break;
        }
      }
    }
  }
  dictionary
}

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

fn run(members: String) -> String {
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

#[command]
async fn matching(ctx: &Context, msg: &Message) -> CommandResult {
  let input_msg = msg.content.to_string();
  if input_msg.len() > 9 {
    let members = String::from(&input_msg[10..]);
    let result = run(members);
    msg.channel_id.say(&ctx.http, result).await?;
  } else {
    let matching_expression = "
How to input
!matching MemberName(Divide into space) matching count
ex) !matching name1 name2 name3 name4 name5 name6 2
  ";
    msg.channel_id.say(&ctx.http, matching_expression).await?;
  }
  
  Ok(())
}

#[group]
#[commands(matching)]
pub struct Matching;
