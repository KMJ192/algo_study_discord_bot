use std::collections::VecDeque;

use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct TopologySortDataType {
  prev: usize,
  next: usize
}

#[derive(Debug, Deserialize, Serialize)]
struct TopologySortType {
  data: Vec<TopologySortDataType>,
}

struct TopologySortClass {
  entry_point: VecDeque<usize>,
  topology_table: Vec<(usize, Vec<usize>)>,
  sorted: Vec<usize>
}

impl TopologySortClass {
  pub fn new () -> Self {
    TopologySortClass { 
      entry_point: VecDeque::new(),
      topology_table: vec![],
      sorted: vec![],
    }
  }
  pub fn sorting(&mut self, data: TopologySortType) {
    let graph = data.data;
    self.topology_table = vec![(0, vec![]); graph.len()];

    for node in graph {
      self.topology_table[node.next].0 += 1;
      self.topology_table[node.prev].1.push(node.next);
    }

    for (node, table) in (0..).zip(self.topology_table.iter()) {
      if table.0 == 0 {
        self.entry_point.push_back(node)
      }
    }

    while let Some(point) = self.entry_point.pop_back() {
      let p = point as usize;
      self.sorted.push(p);
      for node in self.topology_table[p].1.clone() {
        self.topology_table[node].0 -= 1;
        if self.topology_table[node].0 == 0 {
          self.entry_point.push_back(node);
        }
      }
    }
  }
}

fn data_parsing(data: String) -> (Option<TopologySortType>, String) {
  let v = serde_json::from_str(&data);
  let v: TopologySortType = match v {
    Ok(r) => r,
    Err(e) => {
      return (None, format!("{:?}", e));
    }
  };

  (Some(v), String::from(""))
}

#[command]
async fn topology_sort(ctx: &Context, msg: &Message) -> CommandResult {
  let input_msg = msg.content.to_string();
  if input_msg.len() > 14 {
    let input_data = String::from(&input_msg[15..]);
    let data = input_data.replace("```\n", "").replace("\n```", "");
    let (data, error) = data_parsing(data);
    if error.len() > 0 {
      msg.channel_id.say(&ctx.http, &error).await?;
    } else {
      if let Some(d) = data {
        let mut topology = TopologySortClass::new();
        topology.sorting(d);
        let result = format!("topology sort table```{:?}```sorted items```{:?}```", topology.topology_table, topology.sorted);
        msg.channel_id.say(&ctx.http, result).await?;
      }
    }
  } else {
let topology_sort_expression = "
topology_sort(위상정렬)를 실행합니다.
선행, 후행조건에 따라 정렬하는 알고리즘입니다.

JSON 형태로 입력합니다.
\"data\"로 데이터를 입력합니다.
```
{
  \"prev\": number,
  \"next\": number    
}
```
타입의 배열 형태로 입력합니다.

\"prev\"에 선행 노드를 입력합니다.
\"next\"에 후행 노드를 입력합니다.
각 노드는 number 타입입니다.
!topology_sort
```
{
  \"data\": [
    {
      \"prev\": 0,
      \"next\": 1
    },
    {
      \"prev\": 0,
      \"next\": 2
    },
    {
      \"prev\": 1,
      \"next\": 3
    },
    {
      \"prev\": 2,
      \"next\": 3
    }
  ]
}
```
";
    msg.channel_id.say(&ctx.http, topology_sort_expression).await?;
  }
  Ok(())
}

#[group]
#[commands(topology_sort)]
pub struct TopologySort;