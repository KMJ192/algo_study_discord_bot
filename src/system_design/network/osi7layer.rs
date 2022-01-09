use serenity::client::Context;
use serenity::model::{channel::Message};
use serenity::framework::standard::{CommandResult, macros::{group, command}};

#[command]
async fn osi7layer(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
네트워크에서 통신이 일어나는 과정을 7단계로 나눈 형태
표준화를 통한 유지보수 용이

7 - 응용 계층 (Application layer)
6 - 표현 계층 (Presentation layer)
5 - 세션 계층 (Session layer)
4 - 전송 계층 (Transport layer)
3 - 네트워크 계층 (Network layer)
2 - 데이터링크 계층 (Data Link layer)
1 - 물리 계층 (Physical layer)

1 ~ 4 계층 - 하위 계층(물리적 계층)
5 ~ 7 계층 - 상우 계층(논리적 계층)

송신
7->6->5->4->3->2->1
수신
1->2->3->4->5->6->7
";
  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[command]
async fn osi_physical_layer(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
**물리계층**
통신 케이블 등의 하드웨어를 통하여 물리적 신호를 전송
비트 스트림을 전송하며, 통신 단위는 **1**, **0**

장비 : 케이블, 리피터, 허브 등
프로토콜 : RS-232c, X.25, X.21 등
";
    msg.channel_id.say(&ctx.http, info).await?;
    Ok(())
}

#[command]
async fn osi_datalink_layer(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
**데이터링크 계층**
같은 네트워크 대역을 사용하는 Device에 대하여 신뢰성있는 전송을 보장
H/W와 S/W 사이에 위치 -> S/W의 정보를 H/W로 전달 
**Mac address**를 활용하여 같은 구간 내의 Endpoint 혹은 Switching 장비에 전달
물리계층에서 발생할 수 있는 오류 탐색
```
Device의 고유한 물리적 주소
```

장비 : 브리지, 스위치 등
프로토콜 : Ethernet, HDLC(High Level Data Link Control), PPP(Point to Point Protocol) 등
";
  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[command]
async fn osi_network_layer(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
**네트워크 계층**
송신 -> 수신 패킷 전달
송/수신측 논리주소 지정(IP주소 등) 및 패킷이 최종 목적지에 도달하도록 경로 배정 - Routing 기능

장비 : 라우터
프로토콜 : IP, ICMP, IGMP 등
";
  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[command]
async fn osi_transport_layer(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
**전송 계층**
E2E(End to End)의 사용자들이 신뢰성 있는 연결 유지
데이터 분할/재조립, 연결제어, 흐름제어, 오류제어의 역할

장비 : 게이트웨이
프로토콜 : TCP, UDP 등
";
  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[command]
async fn osi_session_layer(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
**세션 계층**
전송계층, 표현계층 사이의 대화제어 및 동기화 담당
세션의 연결, 유지, 해제 담당
```
세션
일정 시간 사용자로 부터 들어오는 요청을 하나의 상태로 보고, 그 상태를 유지 시키는 기술
ex) 웹 서버에 접속한 시점 ~ 웹 브라우저를 종료하여 끝내는 시점
```
연결 손실 시 연결 복구기능

프로토콜
NetBOIS, RPC(Remote Procedure Call), WinSock(Window Socket) 등
";
  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[command]
async fn osi_presentation_layer(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
**표현 계층**
데이터의 변환, 압축, 암호화

프로토콜
SSL(Secure Socket Layer), Ascii 등
";
  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[command]
async fn osi_application_layer(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
**응용 게층**
사용자(Client)에게 서비스를 제공하는 계층
이메일 전송 요청, 파일 전송 요청, 웹 사이트 조회 등 Application에 대한 서비스를 제공하는 계층

프로토콜
HTTP, SMTP, FTP, DNS, SSH 등
";
  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[group]
#[commands(
  osi7layer, 
  osi_physical_layer, 
  osi_datalink_layer, 
  osi_network_layer, 
  osi_transport_layer, 
  osi_session_layer, 
  osi_presentation_layer, 
  osi_application_layer
)]
pub struct OSI7Layer;