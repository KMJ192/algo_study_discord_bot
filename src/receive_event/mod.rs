use serenity::{
  async_trait,
  model::{channel::Message, gateway::Ready, guild::ThreadMember, event::{ThreadListSyncEvent, ThreadMembersUpdateEvent}},
  prelude::*,
};

use super::commands::*;
use super::random_matching::run;

pub struct Handler;

fn string_matching(str1: &str, str2: &str) -> bool {
  if str1 == str2 {
    return true;
  }
  false
}

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    let input_msg = msg.content.to_lowercase();
    if string_matching(&input_msg, REQ_COMMENDS) {
      if let Err(err) = msg.channel_id.say(&ctx.http, RES_COMMENDS).await {
        println!("Error sending message: {:?}", err);
      }
    } else if string_matching(&input_msg, REQ_INTERVIEW_TEMPLATE) {
      if let Err(err) = msg.channel_id.say(&ctx.http, RES_INTERVIEW_TEMPLATE).await {
        println!("Error sending message: {:?}", err);
      }
    } else if string_matching(&input_msg, REQ_FD_TEMPLATE) {
      if let Err(err) = msg.channel_id.say(&ctx.http, RES_FD_TEMPLATE).await {
        println!("Error sending message: {:?}", err);
      }
    } else if string_matching(&input_msg, REQ_MATCHING) {
      if let Err(err) = msg.channel_id.say(&ctx.http, RES_MATCHING).await {
        println!("Error sending message: {:?}", err);
      }
    } else if input_msg.len() > 9 && string_matching(&String::from(&input_msg[0..9]), REQ_MATCHING) {
      let members = String::from(&input_msg[10..]);
      let result = run(members);
      if let Err(err) = msg.channel_id.say(&ctx.http, &result).await {
        println!("Error sending message: {:?}", err);
      }
    } else if string_matching(&input_msg, REQ_KMP_CODE) {
      if let Err(err) = msg.channel_id.say(&ctx.http, RES_KMP_CODE).await {
        println!("Error sending message: {:?}", err);
      }
    }
  }

  async fn thread_member_update(&self, _ctx: Context, _thread_member: ThreadMember) {
    println!("{:?}", _thread_member);
  }

  async fn thread_members_update(
    &self,
    _ctx: Context,
    _thread_members_update: ThreadMembersUpdateEvent,
  ) {
    println!("{:?}", _thread_members_update);
  }

  async fn thread_list_sync(&self, _ctx: Context, _thread_list_sync: ThreadListSyncEvent) {
    println!("{:?}", _thread_list_sync);
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}