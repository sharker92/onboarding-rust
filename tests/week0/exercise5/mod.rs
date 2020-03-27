use onboarding_rust::week0::exercise5::network_delay_time;

#[test]
fn test_week0_exercise5_example1() {
    let times = vec![vec![2,1,1],vec![2,3,1],vec![3,4,1]];
    let n = 4;
    let k = 2;
    let expected = 2;

    assert_eq!(expected, network_delay_time(times, N, K));
}
