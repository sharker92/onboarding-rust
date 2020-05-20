use onboarding_rust::week2::exercise12::MyHashSet;

#[test]
fn test_week2_exercise12_example1() {
    let mut hash_set: MyHashSet = MyHashSet::new();
    hash_set.add(1);
    hash_set.add(2);
    assert!(hash_set.contains(1), "not contains 1");
    assert!(!hash_set.contains(3), "contains 3");
    hash_set.add(2);
    assert!(hash_set.contains(2), "not contains 2"); // returns true
    hash_set.remove(2);
    assert!(!hash_set.contains(2,) "contains 2");

}
