use onboarding_rust::week1::exercise19::two_sum;

#[test]
fn test_week1_exercise17_example1() {
    let input = vec![2, 7, 11, 15];
    let target = 9;
    let expected = vec![0, 1];
    assert_eq!(expected, two_sum(input, target));
}