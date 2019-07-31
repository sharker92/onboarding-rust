use onboarding_rust::week2::exercise21::longest_word;

#[test]
fn test_week2_exercise21_example1() {
    let input = vec!["w".to_string(),"wo".to_string(),"wor".to_string(),"worl".to_string(), "world".to_string()];
    let expected = "world".to_string();
    assert_eq!(expected, longest_word(input));
}

#[test]
fn test_week2_exercise21_example2() {
    let input = vec!["a".to_string(), "banana".to_string(), "app".to_string(), "appl".to_string(), "ap".to_string(), "apply".to_string(), "apple".to_string()];
    let expected = "apple".to_string();
    assert_eq!(expected, longest_word(input));
}
