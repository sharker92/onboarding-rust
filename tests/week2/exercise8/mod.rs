use onboarding_rust::week2::exercise8::is_valid_sudoku;

#[test]
fn test_week2_exercise8_example1() {
    let input = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let expected = true;
    assert_eq!(expected, is_valid_sudoku(input));
}

#[test]
fn test_week2_exercise8_example2() {
    let input = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let expected = false;
    assert_eq!(expected, is_valid_sudoku(input));
}

#[test]
fn test_week2_exercise8_example3() {
    let input = vec![
        vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
        vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
        vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
        vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
        vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
        vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
        vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
    ];
    let expected = false;
    assert_eq!(expected, is_valid_sudoku(input));
}
