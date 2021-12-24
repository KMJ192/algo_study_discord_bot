use std::env;

use serenity::client::Client;
use serenity::framework::standard::StandardFramework;

pub mod bot_information;
pub mod interview_template;
pub mod random_matching;
pub mod ds;
pub mod algorithm;

// pub mod receive_event;
// use receive_event::*;

use bot_information::BOTINFORMATION_GROUP;
use interview_template::INTERVIEWTEMPLATE_GROUP;
use random_matching::matching_algorithm::MATCHING_GROUP;
use ds::trie::trie::TRIE_GROUP;
use algorithm::{
  kmp::kmp::KMP_GROUP, 
  knapsack::knapsack::KNAPSACK_GROUP, 
  dijkstra::dijkstra::DIJKSTRA_GROUP,
  topology_sort::topology_sort::TOPOLOGYSORT_GROUP,
};


#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let framework = StandardFramework::new()
    .configure(|c| c.prefix("!"))
    .group(&INTERVIEWTEMPLATE_GROUP)
    .group(&MATCHING_GROUP)
    .group(&BOTINFORMATION_GROUP)
    .group(&TRIE_GROUP)
    .group(&KNAPSACK_GROUP)
    .group(&KMP_GROUP)
    .group(&TOPOLOGYSORT_GROUP)
    .group(&DIJKSTRA_GROUP);

  let mut client = Client::builder(token)
    // .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Err creating client");

  if let Err(err) = client.start().await { 
    println!("{:?}", err);
  }
}