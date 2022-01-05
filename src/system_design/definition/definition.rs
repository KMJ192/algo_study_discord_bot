use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

#[command]
async fn system_design(ctx: &Context, msg: &Message) -> CommandResult {
let definition = "
시스템이 특정 요구사항을 충족하도록,
Architecture, module, interface, data 등을 정의하는 과정이다.
만약 Server와 Client의 물리적 거리가 길어질수록 ***어떤 알고리즘***을 사용하여,
어떤 ***네트워크***를 구축할지, 어떤 네트워크 ***통신***을 이용하게 할지, 
만약 ***Side Effect***(Server 과부화 등의)가 발생하였을때 어떤 ***대처***를 해야 될지,
이러한 상황을 예방하기 위해 어떤 시스템을 구축해 놓을지 등등 S/W, H/W 시스템을 설계하기 위한 모든것이 포함된다.

시스템 디자인을 위해 고려해야 할 항목
- **Low Latency** : 요청 / 응답의 짧은 지연
- **System reliability** : 시스템의 신뢰성
- **System scalability** : 시스템의 확장성
- **System performance** : 시스템의 성능
- **Cost effective** : 비용 효율성
- **Data consistency** : 데이터의 일관성
- **Security** : 보안
- ETC...

";
  msg.channel_id.say(&ctx.http, definition).await?;
  Ok(())
}

#[group]
#[commands(system_design)]
pub struct SystemDesign;