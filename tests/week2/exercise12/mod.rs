use onboarding_rust::week2::exercise12::MyHashSet;

#[test]
fn test_week2_exercise12_example1() {
    let mut hash_map: MyHashSet = MyHashSet::new();
    hash_map.put(1, 1);
    hash_map.put(2, 2);
    assert_eq!(1, hash_map.get(1));
    assert_eq!(-1, hash_map.get(3));

    hash_map.put(2, 1);
    assert_eq!(1, hash_map.get(2));

    hash_map.remove(2);
    assert_eq!(-1, hash_map.get(2));
}