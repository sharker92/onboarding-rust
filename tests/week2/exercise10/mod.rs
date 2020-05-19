use onboarding_rust::week2::exercise10::word_pattern;

#[test]
fn test_week2_exercise10_example1() {
    let pattern = "abba".to_string();
    let st = "dog cat cat dog".to_string();

    let expected = true;

    assert_eq!(expected, word_pattern(pattern, st));
}

#[test]
fn test_week2_exercise10_example2() {
    let pattern = "abba".to_string();
    let st = "dog cat cat fish".to_string();

    let expected = false;

    assert_eq!(expected, word_pattern(pattern, st));
}

#[test]
fn test_week2_exercise10_example3() {
    let pattern = "aaaa".to_string();
    let st = "dog cat cat dog".to_string();

    let expected = false;

    assert_eq!(expected, word_pattern(pattern, st));
}
#[test]
fn test_week2_exercise10_example4() {
    let pattern = "abba".to_string();
    let st = "dog dog dog dog".to_string();

    let expected = false;

    assert_eq!(expected, word_pattern(pattern, st));
}

#[test]
fn test_week2_exercise10_example5() {
    let pattern = "aaa".to_string();
    let st = "aa aa aa aa".to_string();

    let expected = false;

    assert_eq!(expected, word_pattern(pattern, st));
}
