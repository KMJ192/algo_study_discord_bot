use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

#[command]
async fn type_happen(ctx: &Context, msg: &Message) -> CommandResult {
let info = "
**Web browser에 www.google.com을 검색할 때 일어나는 일**
**1.** www.google.com을 브라우저 주소창에 작성
**2.** Browser는 Cached DNS기록을 통해 www.google.com에 대응되는 IP주소의 유무를 확인
```
DNS - Domain Name System
- URL의 이름과 IP주소가 저장되어 있는 DB
- 인터넷 상 모든 URL에 고유 IP Address가 지정되어 있는데, 
IP를 통해 해당 웹사이트를 호스팅하는 Server Device에 접근할 수 있음
```
**3.** 요청한 URL이 Cache에 없다면 ISP의 DNS 서버가 www.google.com을 호스팅 하고 있는 서버의 IP 주소를 찾기위해 DNS query를 날림
```
DNS query 
- 여러 다른 DNS 서버들을 검색하여 해당 사이트의 IP 주소를 찾는것
- 여러 DNS 서버들을 옮겨가며 해당 사이트의 IP주소를 찾을때 까지(또는 못찾아서 에러를 발생할때 까지) 검색
```
**4.** Browser가 Server와 TCP connection
**5.** Browser가 웹 서버에 HTTP request
**6.** Server가 요청을 처리하고 response를 생성
**7.** Server는 Client로 HTTP response
**8.** Client의 Browser는 HTML content를 rendering
";
  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[group]
#[commands(type_happen)]
pub struct TypeHappen;