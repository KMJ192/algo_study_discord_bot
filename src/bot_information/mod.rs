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
!info
!matching
!ft
!it 
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
!osi7layer
!osi_physical_layer
!osi_datalink_layer
!osi_network_layer
!osi_transport_layer
!osi_session_layer
!osi_presentation_layer
!osi_application_layer
";
  let com  = str::replace(com, "\n", " ");
  msg.channel_id.say(&ctx.http, &com).await?;

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