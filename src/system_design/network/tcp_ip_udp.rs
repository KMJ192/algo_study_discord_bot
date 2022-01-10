use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

use super::super::super::db::firebase::IMAGE_STORAGE;

#[command]
async fn tcp(ctx: &Context, msg: &Message) -> CommandResult {
let info = "
**TCP - Transmission Control Protocol**
- 컴퓨터 사이 통신을 위한 **규약(protocol)**
- Data의 순차 전송을 보장
- UDP보다 높은 신뢰성, 낮은 속도
- 흐름제어 혼잡제어 오류감지
```
┌──────┐──────────┐──────┐
|Header|TCP Header| Data |
└──────┘──────────┘──────┘
```
tcp header 정보 -> !tcp_header
";

  msg.channel_id.say(&ctx.http, info).await?;
  let desc = "
1. client는 server에 접속을 요청하는 syn 패킷을 전송 후 ack 응답을 기다리는 상태가 됨
2. server는 syn요청을 받고 client에 요청을 허락하는 ack와 syn flag가 설정된 패킷을 발송 후 ack응답을 기다리는 상태가 됨
3. client는 server에 ack를 보낸 후 연결이 되어(established) 데이터를 송수신할 수 있게 됨
(segment라는 패킷으로 통신)

segment
- PDU(Protocol Data Unit)로 그룹화
- 전송 할 데이터를 쪼개는데 여기서 header 영역이 있고 tcp header와 data로 이루어진 세그먼트가 있음
  ";
  let url = format!("{}/3wh.PNG?alt=media", IMAGE_STORAGE);
  msg.channel_id.send_message(&ctx.http, |m| {
    m.embed(|e|
      e.title("3-way handshake")
        .description(desc)
        .image(url)
    )
  }).await?;

  Ok(())
}

#[command]
async fn tcp_header(ctx: &Context, msg: &Message) -> CommandResult {
let info = "
20bytes 크기
송신측ip, 수신측ip, port, Seq number, ack, syn 등의 정보를 가지고 있음
**port**
- 송/수신측 port정보를 가짐
**Seq number**
- 데이터의 순서를 의미하는 sequence number
- 32bit 크기 할당, 0 ~ 4,294,967,296
- 수신측에서 데이터를 조립할때, sequence number를 이용하여 올바른 데이터를 얻을 수 있음
**Data offset**
- Data영역의 위치값을 포함
- 32bit Word 단위를 사용, 32bit에서는 1Word가 4byte
- Data offset에 4를 곱하면 실제 데이터의 시작 위치를 알 수 있음
```
Word: 하나의 기계어 명령어나 연산을 통해 저장된 장치로부터 레지스터에 옮겨놓을 수 있는 데이터 단위
```
**Reserved**
- 예약 필드로써 0으로 채워진 영역
**Flags**
- SYN: 상대방과 연결을 생성할 때, 시퀀스 번호의 동기화를 맞춤
- ACK: 승인번호(Acknowledgement)필드에 값이 채워져 있음을 알려주는 플래그
- FIN: Finish 플래그, 연결 종료 요청
**Window Size**
- 한번에 전송할 수 있는 데이터량을 의미
**Checksum**
- TCP Segment data 송신 도중 발생될 수 있는 오류를 검출하기 위한 수단
**Urgent Pointer**
- 긴급 포인터
- URG플래그사 1일 경우 우선 처리 대상
**Options**
- TCP 기능을 확장할 때 사용하는 필드
";

  let url = format!("{}/tcp.PNG?alt=media", IMAGE_STORAGE);
  msg.channel_id.send_message(&ctx.http, |m| {
    m.embed(|e|
      e.title("Tcp header")
        .description(info)
        .image(url)
    )
  }).await?;
  Ok(())
}

#[command]
async fn ip(ctx: &Context, msg: &Message) -> CommandResult {
let info = "
송수신 host가 데이터를 주고받는데 사용하는 규약
OSI 7Layer의 ***네트워크 계층***에 속함
IPv4, IPv6가 있음
IPv4
- 3자리 숫자가 4마디로 표기됨, **.**로 분리된 형태
- 각 마디는 옥텟 (octet)이라고 함
- 클래스 A ~ E로 분리되며, 각 클래스는 0 ~ 255 사이의 숫자로 이루어짐
```
class A -> 0.0.0.0 ~ 127.255.255.255
class B -> 128.0.0.0 ~ 191.255.255.255
class C -> 192.0.0.0 ~ 223.255.255.255
class D -> 224.0.0.0 ~ 239.255.255.255
class E -> 240.0.0.0 ~ 255.255.255.255
```
- IPv4의 물리적인 한계를 보완하기 위해 IPv6가 고안됨
- localhost == 127.0.0.1
IPv6
- **:**로 분리된 8개의 16진수 block형태
";

  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[command]
async fn udp(ctx: &Context, msg: &Message) -> CommandResult {
let info = "
**UDP - User Datagram Protocol**
- 컴퓨터 사이 통신을 위한 **규약(protocol)**
- **Server**에서 **Client**로 일방적인 데이터 전송
- Connection less
- TCP보다 높은 속도, 낮은 신뢰성
- 데이터의 신뢰성보다 실시간성이 중요할 경우 사용할 수 있음 -> socket, webRTC 등
```
┌──────┐──────────┐──────┐
|Header|UDP Header| Data |
└──────┘──────────┘──────┘
```
- Header: ip정보가 담김
- UDP Header: port번호 정보, header와 data를 합한 길이 checksum등을 포함
";
  let url = format!("{}/udp.PNG?alt=media", IMAGE_STORAGE);
  msg.channel_id.send_message(&ctx.http, |m| {
    m.embed(|e|
      e.title("UDP")
        .description(info)
        .image(url)
    )
  }).await?;
  Ok(())
}

#[group]
#[commands(tcp, tcp_header, ip, udp)]
pub struct TcpIpUdp;