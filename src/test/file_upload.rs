// use serenity::builder::{CreateMessage, CreateEmbed};
use serenity::client::Context;
use serenity::model::{channel::Message};
use serenity::framework::standard::{CommandResult, macros::{group, command}};

#[command]
async fn file_upload(ctx: &Context, msg: &Message) -> CommandResult {
  // let embed_msg: |&mut CreateMessage| -> &mut CreateMessage = |m| {
  //   m.embed(|e| 
  //     e.title("Embed title")
  //     .description("test")
  //     .image("../../assets/discordgo.png")
  //     .field("A field", "Some content", false)
  //   )
  // };
  msg.channel_id.send_message(&ctx.http, |m| {
    m.embed(|e| 
      e.title("Embed message 테스트")
      .image("../../assets/discordgo.png")
    )
  }).await?;
  Ok(())
}

#[group]
#[commands(file_upload)]
struct FileUploadTest;