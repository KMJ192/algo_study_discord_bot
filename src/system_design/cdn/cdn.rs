use serenity::client::Context;
use serenity::model::{channel::Message};
use serenity::framework::standard::{CommandResult, macros::{group, command}};

#[command]
async fn cdn(ctx: &Context, msg: &Message) -> CommandResult {
let definition = "
**CDN - Content Delivery Network**
- CDN 도입 배경
```
Global service를 할 때,
-> Client와 Server간 물리적인 거리가 길어 지연시간도 길어짐
-> main server에 모든 사용자가 접근하려고 할 경우 막대한 트래픽이 발생하여
서버에 부하를 줄 리스크가 있음
```
- 여러 지역에 CDN을 두어 물리적인 거리를 줄임 (Replica server)
- **Caching**기능으로 응답시간을 단축
```
cdn에 요청 컨텐츠가 없음
┌────────┐       ┌──────┐        ┌─────────────┐  
| client | <---  | cdn  |  <---  | main server |
|        | --->  |      |  --->  |             |
└────────┘       └──────┘        └─────────────┘
cdn에 요청 컨텐츠가 있음
┌────────┐       ┌────────┐       ┌─────────────┐  
| client | <---  | cdn    |       | main server |
|        | --->  |(cached)|       |             |
└────────┘       └────────┘       └─────────────┘
```
";
  msg.channel_id.say(&ctx.http, definition).await?;
  Ok(())
}

#[group]
#[commands(cdn)]
pub struct CDN;