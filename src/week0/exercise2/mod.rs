pub fn partition_labels(s: String) -> Vec<i32> {
    let result = Vec::new();
    let mut word = s.clone();
    let mut cut = 0;

    for (i, x) in word.char_indices() {
        if i > cut {
            // result = cut; 
            break;
        }
        for (j, y) in word.char_indices().rev() {
            //println!("{}, {}, {}",x , y , i );
            if x == y && j > cut {
                cut = j;
                break;
            }
        }
    }

    result
}
