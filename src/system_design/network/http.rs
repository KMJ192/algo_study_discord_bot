use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

#[command]
async fn http(ctx: &Context, msg: &Message) -> CommandResult {
let info = "
**HTTP - Hyper Text Transfer Protocol**
- HTML, XML등의 문서와 같은 리소스를 가져올 수 있도록 해주는 **규약(protocol)**
- 웹에서 이루어지는 모든 데이터 교환의 기초
- stateless 프로토콜 (이전 데이터 요청과 다음 데이터 요청이 서로 관련이 없음)
- 기본 port는 80번 port를 이용함
- HTTP Request, HTTP Response로 데이터를 주고받음
- 메서드
```
GET    : 데이터 조회
POST   : 데이터 생성
PUT    : 데이터 변경
DELETE : 데이터 삭제
```
- 상태 코드
```
1xx : 조건부 응답, 요청을 받았으며, 작업을 계속 진행함
2xx : 요청 성공
3xx : Redirection 완료, 요청 완료를 위해 추가 작업 조치가 필요함을 의미
4xx : 클라이언트 오류, 요창 문법이 잘못되었거나 요청을 처리할 수 없음
5xx : 서버 오류, 유효한 요청에 대하여 처리할 수 없음
```
";

  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[command]
async fn https(ctx: &Context, msg: &Message) -> CommandResult {
let info = "
**HTTPS - Hyper Text Transfer Protocol Secure**
- HTTP의 보안 버전
- HTTP는 일반 텍스트 protocol로 설계되었으므로 도청, 중간공격에 취약
- HTTPS는 SSL 또는 TLS를 이용하여 Client의 민감한 정보(결제시스템 등)를 암호화 하여, Server와 안전하게 주고받도록 함
";

  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[group]
#[commands(http, https)]
pub struct Http;