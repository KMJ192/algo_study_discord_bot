pub const REQ_COMMENDS: &str = "!commands";
pub const RES_COMMENDS: &str = "!info !matching !ft(Feedback Template) !it(Interview Template) !kmp_code !trie_run !trie_code";

pub const REQ_MATCHING: &str = "!matching";
pub const RES_MATCHING: &str = "
  How to input
  !matching MemberName(Divide into space) matching count
  ex) !matching name1 name2 name3 name4 name5 name6 2
";

pub const REQ_TRIE_RUN: &str = "!trie_run";
pub const RES_TRIE_RUN: &str = "
  How to input
  !trie_run InputString(Divide into space)
  ex) !trie_run app apple as sc
";

pub const REQ_TRIE_CODE: &str = "!trie_code";
pub const RES_TRIE_CODE: &str = "
```ts
// ts 코드 (언어별 코드는 추후 업데이트)
export class TrieNode {
  public isWord: boolean;

  public word: string;

  public next: {
    [key: string]: TrieNode;
  };

  constructor() {
    this.isWord = false;
    this.next = {};
    this.word = '';
  }
}

export class TrieDataStructure {
  private root: TrieNode;

  constructor() {
    this.root = new TrieNode();
  }

  public insert(word: string) {
    let curNode: TrieNode = this.root;
    for (let i: number = 0; i < word.length; i++) {
      const c: string = word[i];
      if (!curNode.next[c]) {
        curNode.next[c] = new TrieNode();
      }
      curNode = curNode.next[c];
    }
    curNode.word = word;
    curNode.isWord = true;
  }

  public search(word: string): boolean {
    let curNode: TrieNode = this.root;
    for (let i: number = 0; i < word.length; i++) {
      curNode = curNode.next[word.charAt(i)];
      if (!curNode) return false;
    }
    return curNode.isWord;
  }

  public startsWith(prefix: string): boolean {
    let curNode = this.root;
    for (let i: number = 0; i < prefix.length; i++) {
      curNode = curNode.next[prefix.charAt(i)];
      if (!curNode) return false;
    }
    return true;
  }
}
```
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
Goal of project
  - Automation of pair programming matching.
  - Visualization of algorithm run result. (text type display)
  - Provision algorithm code / data structure code.
  - Provision sandard library method, how to use.

input command =>
!info !matching !ft(Feedback Template) !it(Interview Template) !kmp_code !trie_run !trie_code

dev stack
- rust
- tokio
- serenity
- heroku (deployment)

git address : https://github.com/KMJ192/algo_study_discord_bot
";