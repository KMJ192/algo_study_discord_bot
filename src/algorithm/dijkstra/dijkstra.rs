use std::collections::BinaryHeap;

use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct DijkstraDataType {
  from: u32,
  to: u32,
  weight: u32
}

#[derive(Debug, Deserialize, Serialize)]
struct DijkstraType {
  start: u32,
  dest: u32,
  #[serde(rename="nodeCnt")]
  node_cnt: u32,
  data: Vec<DijkstraDataType>,
}

struct DijkstraClass {
  infinity: i32,
  path: Vec<Vec<(u32, u32)>>,
  start: u32,
  dest: u32,
  table: Vec<i32>,
}

impl DijkstraClass {
  fn new() -> Self {
    DijkstraClass {
      infinity: std::i32::MAX,
      path: vec![],
      start: 0,
      dest: 0,
      table: vec![]
    }
  }

  fn dijkstra_logic(&mut self) {
    let mut pq = BinaryHeap::new();
    self.table[self.start as usize] = 0;
    pq.push((0, self.start));

    while let Some(unit) = pq.pop() {
      let start_node = unit.1 as usize;
      let weight = -unit.0 as i32;
      
      if (self.table[start_node] as i32) < weight { continue; }

      for i in 0..self.path[start_node].len() {
        let (next_node, node_dist) 
          = (self.path[start_node][i].0, (self.path[start_node][i].1 as i32) + weight);
        if self.table[next_node as usize] > node_dist {
          self.table[next_node as usize] = node_dist;
          pq.push((-node_dist, next_node));
        }
      }
    }
  }

  pub fn run(&mut self, input_data: DijkstraType) -> String {
    self.start = input_data.start;
    self.dest = input_data.dest;
    self.table = vec![self.infinity as i32; input_data.node_cnt as usize];
    self.path = vec![vec![]; input_data.node_cnt as usize + 1];

    for i in 0..input_data.data.len() {
      if input_data.data[i].from > input_data.node_cnt
        || input_data.data[i].to > input_data.node_cnt {
          return String::from("입력 데이터가 잘못되었습니다. nodeCnt보다 큰 노드(from, to)는 입력할 수 없습니다.");
      }
      let from = input_data.data[i].from;
      let to = input_data.data[i].to;
      let weight = input_data.data[i].weight;
      self.path[from as usize].push((to, weight));
    }

    self.dijkstra_logic();
    
    if self.table[self.dest as usize] == std::i32::MAX {
      return 
        format!("dijkstra table: ```{:?}```result: ```{}```", 
          self.table, String::from("경로를 찾을 수 없습니다.")
        );
    }

    format!("dijkstra table ```{:?}``` \"{}\"(INT_MAX | Infinity)는 해당 노드까지의 경로를 찾을 수 없는 결과입니다.)\nresult ```{}```", 
      self.table, std::i32::MAX, self.table[self.dest as usize]
    )
  }
}

fn data_parsing(data: String) -> (Option<DijkstraType>, String) {
  let v = serde_json::from_str(&data);
  let v: DijkstraType = match v {
    Ok(r) => r,
    Err(e) => {
      return (None, format!("{:?}", e));
    }
  };
  (Some(v), String::from(""))
}

#[command]
async fn dijkstra(ctx: &Context, msg: &Message) -> CommandResult {
  let input_msg = msg.content.to_string();
  if input_msg.len() > 9 {
    let input_data = String::from(&input_msg[10..]);
    let data = input_data.replace("```\n", "").replace("\n```", "");
    let (data, error) = data_parsing(data);
    if error.len() > 0 {
      msg.channel_id.say(&ctx.http, &error).await?;
    } else {
      if let Some(d) = data {
        let mut dijk = DijkstraClass::new();
        let result = dijk.run(d);
        msg.channel_id.say(&ctx.http, &result).await?;
      }
    }
  } else {
let dijkstra_expression = "
dijkstra(다익스트라) 알고리즘을 실행합니다.
최소비용으로 목표거리까지의 경로를 찾는 알고리즘입니다.
각 노드는 number 타입이며, 2개의 노드 사이의 비용이 음수일 경우 활용할 수 없는 알고리즘입니다.

JSON 형태로 입력합니다.
\"start\"에는 시작 노드를 입력합니다.
\"dest\"는 목표 노드를 입력합니다.
\"nodeCnt\"에는 노드의 갯수를 입력합니다.
ex)
!dijkstra
```
{
  \"start\": 1,
  \"dest\": 5,
  \"nodeCnt\": 5,
  \"data\": [
    {
      \"from\": 0,
      \"to\": 1,
      \"weight\": 3
    },
    {
      \"from\": 0,
      \"to\": 3,
      \"weight\": 1
    },
    {
      \"from\": 1,
      \"to\": 2,
      \"weight\": 7
    },
    {
      \"from\": 2,
      \"to\": 3,
      \"weight\": 3
    },
    {
      \"from\": 3,
      \"to\": 4,
      \"weight\": 10
    },
    {
      \"from\": 3,
      \"to\": 5,
      \"weight\": 1
    },
    {
      \"from\": 5,
      \"to\": 4,
      \"weight\": 1
    }
  ]
}
```
";
    msg.channel_id.say(&ctx.http, dijkstra_expression).await?;
  }
  Ok(())
}

#[group]
#[commands(dijkstra)]
pub struct Dijkstra;