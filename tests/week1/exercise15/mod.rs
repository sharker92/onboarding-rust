use onboarding_rust::week1::exercise15::get_complement_number;

#[test]
fn test_week1_exercise15_example1() {
    let input = 5;      //101b
    let expected = 2;   //010b
    assert_eq!(expected, get_complement_number(input));
}

#[test]
fn test_week1_exercise15_example2() {
    let input = 1;      
    let expected = 0;
    assert_eq!(expected, get_complement_number(input));
}

#[test]
fn test_week1_exercise15_example5() {
    let input = 3;      //11b
    let expected = 0;   //00b
    assert_eq!(expected, get_complement_number(input));
}