pub fn partition_labels(word: String) -> Vec<i32> {
    let mut result = Vec::new();
    let mut cut = 0;
    let mut val = 0;

    for (i, x) in word.char_indices() {

        for (j, y) in word.char_indices().rev() {
            if x == y && j > cut {
                cut = j;
                break;
            }
        }
        if i == cut {
            result.push(cut as i32 + 1 - val);
            cut = 0;
            val = i as i32 + 1;
        }
    }
    result
}
