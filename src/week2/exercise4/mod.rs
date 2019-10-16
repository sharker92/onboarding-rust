use std::collections::HashMap;

pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
    let mut words_count: HashMap<String, i32> = HashMap::new();
    let sentence = format!("{} {}", a, b);

    sentence.split(' ').for_each(|word| {
        *words_count.entry(word.to_string()).or_insert(0) += 1;
    });

    uncommon_words(&words_count)
}

fn uncommon_words(map: &HashMap<String, i32>) -> Vec<String> {
    map.iter()
        .filter(|(_, val)| **val == 1)
        .map(|(key, _)| key.to_string())
        .collect::<Vec<String>>()
}