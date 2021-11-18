use std::env;

use serenity::{
  async_trait,
  model::{channel::Message, gateway::Ready},
  prelude::*,
};

const HELP_MESSAGE: &str = "
  test용 bot 입니다. ^^7
";

const HELP_COMMAND: &str = "!test_command";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content == HELP_COMMAND {
      if let Err(err) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
        println!("Error sending message: {:?}", err);
        return;
      }
    }
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN")
    .expect("Expected a token in the environment");

  let mut client = Client::new(&token)
    .event_handler(Handler)
    .await
    .expect("Err creating client");

  if let Err(err) = client.start().await { 
    println!("{:?}", err);
  }
}