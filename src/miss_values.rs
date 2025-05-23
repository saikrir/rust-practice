pub fn sum_with_missing(input: &Vec<Option<i32>>) -> i32 {
    input.iter().map(|o| o.unwrap_or(0)).sum()
}

#[test]
fn empty() {
    let nn = vec![];
    assert_eq!(sum_with_missing(&nn), 0);
}

#[test]
fn no_missing() {
    let nn = vec![Some(1), Some(5), Some(4)];
    assert_eq!(sum_with_missing(&nn), 10);
}

#[test]
fn some_missing() {
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    assert_eq!(sum_with_missing(&nn), 10);
}

#[test]
fn all_missing() {
    let nn = vec![None, None, None];
    assert_eq!(sum_with_missing(&nn), 0);
}
