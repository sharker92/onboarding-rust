use std::collections::HashMap;

pub fn word_pattern(pattern: String, st: String) -> bool {
  let mut hashmap = HashMap::new();
  for (c, word) in pattern.chars().zip(st.split_whitespace()) {
    //buscar po dog, cat, etc
    let x = *hashmap.entry(c).or_insert(word);
    if x != word {
      println!("ja");
      return false;
    }
    println!("{}, {}", x, *hashmap.get(&c).unwrap());
  }
  println!("{:?}", hashmap);
  true
}
