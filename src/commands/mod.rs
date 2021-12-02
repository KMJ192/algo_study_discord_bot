pub const REQ_COMMENDS: &str = "!commands";
pub const RES_COMMENDS: &str = "!info !matching !ft(Feedback Template) !it(Interview Template) !kmp_code";

pub const REQ_MATCHING: &str = "!matching";
pub const RES_MATCHING: &str = "
  입력방법
  !matching 멤버이름(이름마다 space로 구분) 매칭숫자
  ex) !matching name1 name2 name3 name4 name5 name6 2
";

pub const REQ_KMP_CODE: &str = "!kmp_code";
pub const RES_KMP_CODE: &str = "
```
// cpp 코드 (언어별 코드는 추후 업데이트)
#include <iostream>
#include <vector>

using namespace std;

class KMPAlgorithm {
private:
  void makeTable(string s, vector<int>& table) {
    int sLen = (int)s.size();
    for (int i = 1; i < sLen; i++) {
      int j = table[i - 1];
      
      while (j > 0 && s[i] != s[j]) {
        j = table[j - 1];
      }
      if(s[i] == s[j]) j++;

      table[i] = j;
    }
  }

  void kmp(string &s, string &pattern, vector<int>& table) {
    int sLen = (int)s.size(), patternLen = (int)pattern.size();
    int j = 0;
    for (int i = 0; i < sLen; i++) {
      while (j > 0 && s[i] != pattern[j]) {
        j = table[j - 1];
      }
      if (s[i] == pattern[j]) {
        if (j == patternLen - 1){
          cout << i - patternLen + 1 << endl;
          j = table[j];
        } else j++;
      }
    }
  }
  
public:
  void run(string &s, string &pattern) {
    vector<int> table((int)s.size(), 0);
    makeTable(s, table);
    kmp(s, pattern, table);
  }
};
```
";

pub const REQ_FD_TEMPLATE: &str = "!ft";
pub const RES_FD_TEMPLATE: &str = "
========================================
< Feedback Template >
0. Question/Solver : 문제/푼 사람
1. Problem Solving: 문제 풀었냐 못 풀었냐
2. Coding: 코드 퀄리티(가독성/확장성?/버그FREE)
3. Communication: 의사 전달력 - 얼마나 자기 생각을 말로 잘 표현해내었냐
4. 잘한 것/ 좀 더 노력해야되는 부분 - 각각 최소 2가지씩
========================================
";

pub const REQ_INTERVIEW_TEMPLATE: &str = "!it";
pub const RES_INTERVIEW_TEMPLATE: &str = "
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

pub const INFO: &str = "
Algorithm study discord bot project v0.0.1
프로젝트 목표
  - algorithm 스터디 페어프로그래밍 matching 자동화
  - algorithm 실행 결과 시각화 (text 형태로 출력)
  - algorithm / Data Structure 코드 제공 자동화
  - 언어별 표준 라이브러리 메서드 사용법, 예시 제공

input command =>
!info !matching !ft(Feedback Template) !it(Interview Template) !kmp_code

dev stack
- rust
- tokio
- serenity
- heroku (deployment)

git address : https://github.com/KMJ192/algo_study_discord_bot
";