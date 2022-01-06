use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group
    }
};

#[command]
async fn commands(ctx: &Context, msg: &Message) -> CommandResult {
let com = "
!info !matching 
!ft(Feedback Template)
!it(Interview Template)
!kmp_code
!trie_run
!trie_code
!knapsack
!system_design
!http
!https
!cdn
!tcp
!tcp_header
!ip
!udp
!type_happen
";
  msg.channel_id.say(&ctx.http, com).await?;

  Ok(())
}

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
let info = "
Algorithm study discord bot project v0.0.1
  - Automation of pair programming matching.
  - Visualization of algorithm run result. (text type display)
  - Provision algorithm code / data structure code.
  - Provision sandard library method, how to use.

input command => !commands

dev stack
- rust
- tokio
- serenity
- heroku (deployment)
";
  msg.channel_id.say(&ctx.http, info).await?;

  Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
  msg.channel_id.say(&ctx.http, "pong").await?;

  Ok(())
}

#[group]
#[commands(info, ping, commands)]
pub struct BotInformation;