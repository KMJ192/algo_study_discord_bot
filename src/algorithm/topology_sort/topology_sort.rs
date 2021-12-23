use std::collections::VecDeque;

use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

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
  pub fn sorting(&mut self, graph: Vec<Vec<usize>>) {
    self.topology_table = vec![(0, vec![]); graph.len()];

    for node in graph.iter() {
      let prev = node[0];
      let next = node[1];
      self.topology_table[prev].0 += 1;
      self.topology_table[next].1.push(node[0]);
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

#[command]
async fn topology_sort(ctx: &Context, msg: &Message) -> CommandResult {

  Ok(())
}

#[group]
#[commands(topology_sort)]
pub struct TopologySort;