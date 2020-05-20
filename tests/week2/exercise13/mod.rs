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
