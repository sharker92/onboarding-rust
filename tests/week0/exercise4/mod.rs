use onboarding_rust::week0::exercise4::next_greater_element;

#[test]
fn test_week0_exercise4_example1() {
    let nums1 = vec![4, 1, 2];
    let nums2 = vec![1, 3, 4, 2];
    let expected = vec![-1, 3, -1];

    assert_eq!(expected, next_greater_element(nums1, nums2));
}

#[test]
fn test_week0_exercise4_example2() {
    let nums1 = vec![2, 4];
    let nums2 = vec![1, 2, 3, 4];
    let expected = vec![3, -1];

    assert_eq!(expected, next_greater_element(nums1, nums2));
}

#[test]
fn test_week0_exercise4_example3() {
    let nums1 = vec![2, 4];
    let nums2 = vec![1, 2, 4, 3];
    let expected = vec![4, -1];

    assert_eq!(expected, next_greater_element(nums1, nums2));
}

#[test]
fn test_week0_exercise4_example4() {
    let nums1 = vec![3, 1, 5, 7, 9, 2, 6];
    let nums2 = vec![1, 2, 3, 5, 6, 7, 9, 11];
    let expected = vec![5, 2, 6, 9, 11, 3, 7];

    assert_eq!(expected, next_greater_element(nums1, nums2));
}
