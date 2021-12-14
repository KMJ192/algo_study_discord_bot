use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

mod knapsack;

use knapsack::knapsack_run;

const RES_KNAPSACK: &str = "
knapsack 알고리즘을 실행합니다.
데이터 입력 방식
JSON 형태로 입력합니다.
capacity -> integer type
data -> string type
(data의 value와 weight는 space로 구분, 각 data는 '|' 로 구분)
ex)
!knapsack
```
{
  \"capacity\": 5,
  \"data\": \"3 2 | 4 3 | 5 4 | 6 5\"
}
```
";

#[command]
pub async fn knapsack(ctx: &Context, msg: &Message) -> CommandResult {
  let input_msg = msg.content.to_string();
  if input_msg.len() > 9 {
    let input_data = String::from(&input_msg[10..]);
    let result = knapsack_run(String::from(input_data));
    msg.channel_id.say(&ctx.http, &result).await?;
  } else {
    msg.channel_id.say(&ctx.http, RES_KNAPSACK).await?;
  }
  Ok(())
}

#[group]
#[commands(knapsack)]
pub struct Knapsack;