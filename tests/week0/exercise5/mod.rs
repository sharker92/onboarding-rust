use onboarding_rust::week0::exercise5::network_delay_time;

#[test]
fn test_week0_exercise5_example1() {
    let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    let n = 4;
    let k = 2;
    let expected = 2;

    assert_eq!(expected, network_delay_time(times, n, k));
}

#[test]
fn test_week0_exercise5_example2() {
    let times = vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 2]];
    let n = 3;
    let k = 1;
    let expected = 2;

    assert_eq!(expected, network_delay_time(times, n, k));
}

#[test]
fn test_week0_exercise5_example3() {
    let times = vec![vec![1, 2, 1], vec![2, 3, 2], vec![1, 3, 1]];
    let n = 3;
    let k = 2;
    let expected = -1;

    assert_eq!(expected, network_delay_time(times, n, k));
}
