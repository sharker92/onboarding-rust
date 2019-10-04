use onboarding_rust::week3::exercise15::my_pow;

#[test]
fn test_week3_exercise10_example1() {
    let x = 2.00000;
    let n = 10;
    let expected: f64 = 1024.00000;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

#[test]
fn test_week3_exercise10_example2() {
    let x = 2.10000;
    let n = 3;
    let expected: f64 = 9.26100;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

#[test]
fn test_week3_exercise10_example3() {
    let x = 2.00000;
    let n = -2;
    let expected: f64 = 0.25000;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

#[test]
fn test_week3_exercise10_example4() {
    let x = 2.00000;
    let n = 0;
    let expected: f64 = 1.0;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

//Using 5 decimal precision
fn convert_to_i32(x: f64) -> i32 {
    let base = (10.0 as f64).powi(5);
    (x * base) as i32
}