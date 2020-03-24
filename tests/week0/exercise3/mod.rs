use onboarding_rust::week0::exercise3::arrays_intersection;

#[test]
fn test_week0_exercise3_example1() {
    let arr1 = vec![1,2,3,4,5];
    let arr2 = vec![1,2,5,7,9];
    let arr3 = vec![1,3,4,5,8];
    let expected = vec![1,5];

    assert_eq!(expected, arrays_intersection(arr1, arr2, arr3));
}
