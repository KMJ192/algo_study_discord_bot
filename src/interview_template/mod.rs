use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

#[command]
async fn it(ctx: &Context, msg: &Message) -> CommandResult {
  let interview_template = "
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

  msg.channel_id.say(&ctx.http, interview_template).await?;

  Ok(())
}

#[command]
async fn ft(ctx: &Context, msg: &Message) -> CommandResult {
  let feedback_template = "
========================================
< Feedback Template >
0. Question/Solver : 문제/푼 사람
1. Problem Solving: 문제 풀었냐 못 풀었냐
2. Coding: 코드 퀄리티(가독성/확장성?/버그FREE)
3. Communication: 의사 전달력 - 얼마나 자기 생각을 말로 잘 표현해내었냐
4. 잘한 것/ 좀 더 노력해야되는 부분 - 각각 최소 2가지씩
========================================
  ";

  msg.channel_id.say(&ctx.http, feedback_template).await?;

  Ok(())
}

#[group]
#[commands(ft, it)]
pub struct InterviewTemplate;