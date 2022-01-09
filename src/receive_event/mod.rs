use rand::Rng;
use serenity::{
  async_trait,
  model::{channel::Message, gateway::Ready, guild::ThreadMember, event::{ThreadListSyncEvent, ThreadMembersUpdateEvent}},
  prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    let random_num = rand::thread_rng().gen_range(0..1000);
    if random_num == 139 {
      let tmp = "코딩야기하세요";
      if let Err(_) = msg.channel_id.say(&ctx.http, tmp).await {}
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