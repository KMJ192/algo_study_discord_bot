use std::cmp::max;

#[derive(Debug)]
pub struct KnapsackProblem {
  capacity: i32,
  items: Vec<Vec<i32>>,
  dp_table: Vec<Vec<i32>>,
  result: i32,
  item: Vec<i32>,
}

impl KnapsackProblem {
  pub fn new(capacity: i32, items: Vec<Vec<i32>>) -> Self {
    let item_cnt = items.len();
    KnapsackProblem {
      capacity,
      items,
      dp_table: vec![vec![0; capacity as usize + 1]; item_cnt + 1],
      result: 0,
      item: vec![]
    }
  }

  fn back_tracking(&mut self) {
    let mut cur_capacity = self.capacity;
    let cnt = self.dp_table.len();
    
    for i in (0..cnt).rev() {
      let mut weight= 0;
      let (mut now_dp_weight, mut prev_dp_weight) = (0, 0);
      if i != 0 {
        weight = self.items[i as usize - 1][1];
        now_dp_weight = self.dp_table[i][cur_capacity as usize];
        prev_dp_weight = self.dp_table[i - 1][cur_capacity as usize]
      }
      if now_dp_weight == prev_dp_weight {
        continue;
      }
      self.item.push(i as i32 - 1);
      cur_capacity = cur_capacity - weight;
      if cur_capacity < 0 { 
        break;
      }
    }
    self.item.reverse();
  }

  fn make_table(&mut self) {
    for i in 1..self.dp_table.len() {
      let ele_len = self.dp_table[i].len();
      for cur_capacity in 1..ele_len {
        let (value, weight) = (self.items[i - 1][0], self.items[i - 1][1]);
        let prev_value = self.dp_table[i - 1][cur_capacity];
        
        let mut now_value = 0;
        if cur_capacity as i32 >= weight {
          now_value = value + self.dp_table[i - 1][cur_capacity - weight as usize];
        }

        self.dp_table[i][cur_capacity] = max(prev_value, now_value);
      }
    }
  }

  pub fn algorithm_run (&mut self) -> String{
    self.make_table();
    self.result = self.dp_table[self.dp_table.len() - 1][self.capacity as usize];
    self.back_tracking();
    format!("[{}, {:?}]", self.result, self.item)
  }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct InputType {
  capacity: i32,
  data: String,
}

// use serde_json::Value;

fn confirm_data (input_data: String) -> (bool, i32, Vec<Vec<i32>>) {
  let v = serde_json::from_str(&input_data);
  let v: InputType = match v {
    Ok(r) => r,
    Err(_) => {
      return (false, 0, vec![]);
    }
  };
  let capacity = v.capacity;
  let data_iter = v.data.split("|");
  let arr = data_iter.collect::<Vec<&str>>();
  let mut items: Vec<Vec<i32>> = vec![];
  for i in 0..arr.len() {
    let tmp = arr[i].split_whitespace();
    let tmp = tmp.collect::<Vec<&str>>();
    if tmp.len() < 2 { return (false, 0, vec![]); }
    let (first, second) = (tmp[0].parse::<i32>(), tmp[1].parse::<i32>());
    let f = match first {
      Ok(f) => f,
      Err(_) => {
        return (false, 0, vec![]);
      }
    };

    let s = match second {
      Ok(s) => s,
      Err(_) => {
        return (false, 0, vec![]);
      }
    };
    let tmp = vec![f, s];
    items.push(tmp);
  }
  
  (true, capacity, items)
}

pub fn knapsack_run(input_data: String) -> String {
  let data = input_data.replace("```\n", "").replace("\n```", "");
  let data_parsing = confirm_data(data);
  if data_parsing.0 == false {
    return String::from("데이터 형식이 잘못되었습니다.");
  }
  let (capacity, items) = (data_parsing.1, data_parsing.2);
  
  let mut knapsack_problem = KnapsackProblem::new(capacity, items);
  knapsack_problem.algorithm_run()
}