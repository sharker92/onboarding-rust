pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut array1 = nums1;
    let mut array2 = nums2;

    array1.sort();
    array2.sort();
    array1.dedup();
    array2.dedup();

    array1
        .into_iter()
        .map(|x| {
            array2
                .iter()
                .cloned()
                .filter(|y| x == *y)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>()
        .concat()
}

pub fn intersection_2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut array1 = nums1;
    let mut array2 = nums2;
    let mut result = Vec::new();

    array1.sort();
    array2.sort();
    array1.dedup();
    array2.dedup();

    for a in array1 {
        for b in &array2 {
            if a == *b {
                result.push(a);
            }
        }
    }
    result
}
