use serenity::{
  async_trait,
  model::{channel::Message, gateway::Ready},
  prelude::*,
};

const REQ_COMMENDS: &str = "!command";
const RES_COMMENDS: &str = "!Matching !FeedbackTemplate !InterviewTemplate";

const REQ_MATCHING: &str = "!matching";
const RES_MATCHING: &str = "준비중입니다.";

const REQ_FD_TEMPLATE: &str = "!feedbacktemplate";
const RES_FD_TEMPLATE: &str = "
========================================
< Feedback Template >
0. Question/Solver : 문제/푼 사람
1. Problem Solving: 문제 풀었냐 못 풀었냐
2. Coding: 코드 퀄리티(가독성/확장성?/버그FREE)
3. Communication: 의사 전달력 - 얼마나 자기 생각을 말로 잘 표현해내었냐
4. 잘한 것/ 좀 더 노력해야되는 부분 - 각각 최소 2가지씩
========================================
";

const REQ_INTERVIEW_TEMPLATE: &str = "!interviewtemplate";
const RES_INTERVIEW_TEMPLATE: &str = "
========================================
<Interview Template>
1. Input

2. Output

3. Constraints

4. Edge Cases

5. Data Structure

6. Algorithm

7. Time Complexity

8. Space Complexity

9. Solution

========================================
";

pub struct Handler;

fn string_matching(str1: &str, str2: &str) -> bool {
  if str1.to_lowercase() == str2 {
    return true;
  }
  false
}

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    let input_msg = msg.content;
    if string_matching(&input_msg, REQ_COMMENDS) {
      if let Err(err) = msg.channel_id.say(&ctx.http, RES_COMMENDS).await {
        println!("Error sending message: {:?}", err);
      }
    } else if string_matching(&input_msg, REQ_INTERVIEW_TEMPLATE) {
      if let Err(err) = msg.channel_id.say(&ctx.http, RES_INTERVIEW_TEMPLATE).await {
        println!("Error sending message: {:?}", err);
      }
    } else if string_matching(&input_msg, REQ_FD_TEMPLATE) {
      if let Err(err) = msg.channel_id.say(&ctx.http, RES_FD_TEMPLATE).await {
        println!("Error sending message: {:?}", err);
      }
    } else if string_matching(&input_msg, REQ_MATCHING) {
      if let Err(err) = msg.channel_id.say(&ctx.http, RES_MATCHING).await {
        println!("Error sending message: {:?}", err);
      }
    } 
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}