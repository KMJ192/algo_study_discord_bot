use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{group, command}};

#[command]
async fn kmp_code(ctx: &Context, msg: &Message) -> CommandResult {
  let kmp_code = "
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
  msg.channel_id.say(&ctx.http, kmp_code).await?;
  Ok(())
}

#[group]
#[commands(kmp_code)]
pub struct Kmp;