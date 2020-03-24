use onboarding_rust::week0::exercise2::partition_labels;

#[test]
fn test_week0_exercise2_example1() {
    let input = "ababcbacadefegdehijhklij".to_string();
    let expected = vec![9, 7, 8];

    assert_eq!(expected, partition_labels(input));
}

#[test]
fn test_week0_exercise2_example2() {
    let input = "ababcbacadefegdehbijhklij".to_string();
    let expected = vec![25];

    assert_eq!(expected, partition_labels(input));
}
