use serenity::client::Context;
use serenity::model::{channel::Message};
use serenity::framework::standard::{CommandResult, macros::{group, command}};

#[command]
async fn capacity(ctx: &Context, msg: &Message) -> CommandResult {
let info = "
용량 단위
- 8bits = 1byte
- 1024 bytes = 1kb
- 1024kb = 1mb
- charactor = 1byte
- int = 4byte
- timestamp = 4byte

시간 환산
60s * 60s = 3,600s (1 hour)
3,600s * 24h = 86,400s (1 day)
86,400s * 30 = 2,500,000s (1 month)

";
  msg.channel_id.say(&ctx.http, info).await?;

  Ok(())
}

#[command]
async fn cpu_memory_time_type(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
CPY cycle을 1sec로 가정했을 때 그외 h/w의 속도
```
TYPE             | TIME   | SEC
CPU cycle        | 0.3ms  | 1s
RAM              | 120ms  | 6m
SSD              | 150ms  | 6days
HDD              | 10ms   | 12months
CA               | 183ms  | 19years
(Europe latency)
```
  ";

  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[command]
async fn dau(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
**DAU**(Daily Active Users(DAU), 하루동안 해당 서비스를 이용한 순수 이용자 수)

ex) **SNS서비스 시스템 디자인 예시**
Global service의**read/write** 비율은 **8:2**로 가정
User 1명이 하루에 30개의 사진을 읽고, 1개의 사진을 업로드 한다고 가정
1000만명의 사용자가 사진을 읽고 쓴다고 가정 (사진용량: 500bytes)
```
10 milion DAU * 30 photos = 300 milion (Get)
10 milion DAU * 1 photos upload = 300 milion (Post)
```
초당 요청 수
```
300 milion Get / 86,400s = 3472 reads/s
10 milion Post / 86,400s = 115 write/s
```
팔로워 수에 따라 게시글이 노출되는 정도를 20%라고 가정
DB로 접근하지 않고 Caching을 활용하는것이 효율적임
```
300 milon Request * 500bytes * 0.2 = 30GB / 1day
```
여러가지 장애를 방어하기 위해 Server를 5개 운영한다고 가정(**Replica 적용**)
```
30GB * 5 = 150GB
```
사진 데이터뿐만 아니라 영상, 텍스트 등의 데이터가 추가됨에 따라 감당해야 할 트래픽의 양은 커짐
이를 감당할 시스템 디자인의 중요성↑↑
";
  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[command]
async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
let description = "
데이터를 한 지점에서 다른 지점으로 보내는데 소요되는 시간
Server-Client방식에서의 총 지연시간
1. Client -> Server 
2. Server -> DB
3. DB작업 (search / read / write)
4. DB -> Server
5. Server -> Client
";
  msg.channel_id.send_message(&ctx.http, |m| {
    m.embed(|e| 
      e.title("Latency")
        .description(description)
        .image("https://cdn.discordapp.com/attachments/462496789581529100/929399683032440832/unknown.png")
    )
  }).await?;

  Ok(())
}

#[command]
async fn throughput(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
**Throughput (처리율)**
주어진 시간동안 Device가 감당할 수 있는 처리량
서버가 감당할 수 있는 Request 수
";
  msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[command]
async fn availablity(ctx: &Context, msg: &Message) -> CommandResult {
  let info = "
**Availablity (응답성)**
Server Operating time
하나의 서버로 물리적으로 availablity가 100%일수는 없음
99%일 경우 1년에 5일(24h * 5)의 시간동안 서버가 작동하지 않음
**HA(Highly Availability)**
availablity가 99.999%일때
1년에 5분동안 서버가 작동하지 않음
**Replication**, **Redundancy**등의 시스템을 구축
```
Replication -> 같은 데이터를 공유 여러개의 서버를 구축하는 시스템
Redundancy -> 장애를 대비한 여분의 서버를 구축하는 시스템
```
**서비스 수준 고려**
SLA (Service Level Agreement)
```
서비스가 특정 기대에 못미쳤을 경우 고객 보상을 제공해주는 계약
```
SLI (Service Level Indicators)
```
서비스의 측정 가능한 특성, 시간이 정해지고 측정 가능해야 됨
```
SLO (Service Level Object)
```
주어진 SLI에서 성취할 서비스 수준의 목표 또는 숫자 지표
```
";
    msg.channel_id.say(&ctx.http, info).await?;
  Ok(())
}

#[group]
#[commands(capacity, cpu_memory_time_type, dau, latency, throughput, availablity)]
pub struct Consider;