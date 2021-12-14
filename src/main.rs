use std::env;

use serenity::client::{Client, Context};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

pub mod random_matching;
pub mod receive_event;
pub mod commands;
pub mod ds;
pub mod algorithm;

use commands::INFO;
use receive_event::*;

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, INFO).await?;

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "test message pong!").await?;

    Ok(())
}

use random_matching::MATCHING_GROUP;
use ds::trie::TRIE_GROUP;
use algorithm::knapsack::KNAPSACK_GROUP;

#[group]
#[commands(info, ping)]
struct General;

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let framework = StandardFramework::new()
    .configure(|c| c.prefix("!"))
    .group(&GENERAL_GROUP)
    .group(&MATCHING_GROUP)
    .group(&TRIE_GROUP)
    .group(&KNAPSACK_GROUP);

  let mut client = Client::builder(token)
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Err creating client");

  if let Err(err) = client.start().await { 
    println!("{:?}", err);
  }
}