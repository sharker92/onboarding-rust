use std::collections::HashMap;

pub fn word_pattern(pattern: String, st: String) -> bool {
  let mut hashmap = HashMap::new();
  let mut vec = Vec::new();
  let st2 = st.replace(" ", "");
  if st.len() - st2.len() + 1 != pattern.len() {
    return false;
  }
  for (c, word) in pattern.chars().zip(st.split_whitespace()) {
    if *hashmap.entry(c).or_insert(word) != word {
      return false;
    }
    vec.push(word);
  }
  vec.sort();
  vec.dedup();
  if hashmap.values().len() != vec.len() {
    return false;
  }
  true
}
