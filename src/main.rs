use std::env;

use serenity::prelude::*;

pub mod receive_event;
use receive_event::*;

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let mut client = Client::new(token)
    .event_handler(Handler)
    .await
    .expect("Err creating client");

  if let Err(err) = client.start().await { 
    println!("{:?}", err);
  }
}
