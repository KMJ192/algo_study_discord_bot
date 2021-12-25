use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};
use chrono::prelude::*;

#[command]
async fn christmas(ctx: &Context, msg: &Message) -> CommandResult {
  let dt = Local::now();
  if (dt.month(), dt.day()) == (12, 24) || (dt.month(), dt.day()) == (12, 25) {
    msg.channel_id.say(&ctx.http, "메리크리스마스🎄").await?;
  } else {
    msg.channel_id.say(&ctx.http, "크리스마스아니다").await?;
  }

  Ok(())
}

#[group]
#[commands(christmas)]
pub struct DogDrip;