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
use receive_event::*;

const INFO: &str = "
알고리즘 스터디 디스코드 봇 프로젝트

v0.1.0

input command => !info !matching !ft(Feedback Template) !it(Interview Template)

dev stack
- rust
- tokio
- serenity
- heroku (deployment)

git address : https://github.com/KMJ192/algo_study_discord_bot
";

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, INFO).await?;

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "test message pong!").await?;

    Ok(())
}


#[group]
#[commands(about, ping)]
struct General;

#[tokio::main]
async fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let framework = StandardFramework::new()
    .configure(|c| c.prefix("!"))
    .group(&GENERAL_GROUP);

  let mut client = Client::builder(token)
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Err creating client");

  if let Err(err) = client.start().await { 
    println!("{:?}", err);
  }
}