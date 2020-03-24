pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for n in nums1 {
        let mut iter = nums2.iter();
        iter.find(|&&x| x == n);
        if iter.as_slice().is_empty() {
            println!("{:?} e", iter.as_slice());
            result.push(-1);
        } else {
            while !iter.as_slice().is_empty() {
                let a = iter.next();
                if *a.unwrap() > n {
                    result.push(*a.unwrap());
                    break;
                }
                if iter.as_slice().is_empty() {
                    result.push(-1);
                }

            }
        }

    }
    result
}

pub fn next_greater_element_2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for n in nums1 {
        let mut index = 0;
        for (i, m) in nums2.iter().enumerate() {
            if *m == n {
                index = i + 1;
                break;
            }
        }
        for j in (index as usize)..nums2.len() {
            if nums2[j] > n {
                result.push(nums2[j]);
                break;
            }
            if j == nums2.len() - 1 {
                result.push(-1);
            }
        }
        if index == nums2.len() {
            result.push(-1);
        }
    }
    result
}
