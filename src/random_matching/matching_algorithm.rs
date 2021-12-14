use std::collections::{HashMap, HashSet};
use rand::Rng;

pub fn matching_algorithm(members_array: Vec<String>, pair_num: usize) -> HashMap<String, HashSet<String>> {
  if members_array.len() < pair_num { return HashMap::new(); }
  let mut dictionary: HashMap<String, HashSet<String>> = HashMap::new();
  let members_len = members_array.len();  
  for i in 0..members_len {
    let mem: String = members_array[i].clone();
    dictionary.insert(mem, HashSet::new());
  }
  let mut complete = false;  
  let (mut idx, limit) = (0, members_len * 10000);  
  while complete == false {
    idx += 1;
    let ran_num1 = rand::thread_rng().gen_range(0..members_len);
    let ran_num2 = rand::thread_rng().gen_range(0..members_len);
    if ran_num1 == ran_num2 { continue; }  
    let mem1 = members_array[ran_num1].clone();
    let mem2 = members_array[ran_num2].clone();  
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();  
    if let Some(p1) = dictionary.get(&*mem1) {
      if p1.len() >= pair_num {
        continue;
      }
      set1 = p1.clone();
    }
      if let Some(p2) = dictionary.get(&*mem2) {
        if p2.len() >= pair_num {
          continue;
        }
        set2 = p2.clone();
      }  
      set1.insert(mem2.clone());
      set2.insert(mem1.clone());
      dictionary.insert(mem1, set1);
      dictionary.insert(mem2, set2);  
      let mut cnt = 0;
      for d in &dictionary {
        if d.1.len() < pair_num {
          cnt += 1;
        }
      }
      if cnt <= 1 {
        complete = true;
      }
      if idx > limit {
        break;
      }
  }
  let tmp = dictionary.clone();
  for d in tmp {
    if d.1.len() < pair_num {
      let mut l = d.1.len();
      let (mut idx, limit) = (0, 10000);
      while l < pair_num {
        idx += 1;
        if let Some(p) = dictionary.get(&*d.0) {
          let mut set1 = p.clone();
          let mut set2 = HashSet::new();  
          let ran_num = rand::thread_rng().gen_range(0..members_len);
          let mem = members_array[ran_num].clone();
          if mem != *d.0 {
            l += 1;
            if let Some(pp) = dictionary.get(&*mem) {
              set2 = pp.clone();
            }  
            if let Some(_) = set1.get(&*mem){}
            else {
              set1.insert(mem.clone());
              set2.insert(d.0.clone());
              dictionary.insert(d.0.clone(), set1);
              dictionary.insert(mem, set2);
            }
          }
        }
        if idx > limit {
          break;
        }
      }
    }
  }
  dictionary
}
