use onboarding_rust::week2::exercise13::find_anagrams;

#[test]
fn test_week2_exercise13_example1() {
    let s = "cbaebabacd".to_string();
    let p = "abc".to_string();

    let expected = vec![0, 6];

    assert_eq!(expected, find_anagrams(s, p));
}

#[test]
fn test_week2_exercise13_example2() {
    let s = "abab".to_string();
    let p = "ab".to_string();

    let expected = vec![0, 1, 2];

    assert_eq!(expected, find_anagrams(s, p));
}

#[test]
fn test_week2_exercise13_example3() {
    let s = "".to_string();
    let p = "a".to_string();

    let expected: Vec<i32> = Vec::new();

    assert_eq!(expected, find_anagrams(s, p));
}

#[test]
fn test_week2_exercise13_example4() {
    let s = "aaaaaaaaaa".to_string();
    let p = "aaaaaaaaaaaaa".to_string();

    let expected: Vec<i32> = Vec::new();

    assert_eq!(expected, find_anagrams(s, p));
}

#[test]
fn test_week2_exercise13_example5() {
    let s = "aaabaaa".to_string();
    let p = "aaa".to_string();

    let expected = vec![0, 4];

    assert_eq!(expected, find_anagrams(s, p));
}

#[test]
fn test_week2_exercise13_example6() {
    let s = "abcdabcd".to_string();
    let p = "abcd".to_string();

    let expected = vec![0, 1, 2, 3, 4];

    assert_eq!(expected, find_anagrams(s, p));
}

#[test]
fn test_week2_exercise13_example7() {
    let s = "abaacbabc".to_string();
    let p = "abc".to_string();

    let expected = vec![3, 4, 6];

    assert_eq!(expected, find_anagrams(s, p));
}
#[test]
fn test_week2_exercise13_example8() {
    let s = "baa".to_string();
    let p = "aa".to_string();

    let expected = vec![1];

    assert_eq!(expected, find_anagrams(s, p));
}
