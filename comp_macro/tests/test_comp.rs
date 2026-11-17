use comp_macro::{comp, comp_vec};

#[test]
fn test_single_loop_with_condition() {
    let evens_doubled: Vec<i32> = comp![x * 2 for x in 0..6 if x % 2 == 0].collect();
    assert_eq!(evens_doubled, vec![0, 4, 8]);
}

#[test]
fn test_nested_loops_and_multiple_conditions() {
    let pairs = comp_vec![
        (x, y)
        for x in 1..=4 if x != 2
        for y in 1..x if (x + y) % 2 == 1
    ];
    assert_eq!(pairs, vec![(3, 2), (4, 1), (4, 3)]);
}

#[test]
fn test_destructuring_patterns() {
    let items = vec![("a", 1), ("b", 2), ("c", 3)];
    let filtered = comp_vec![name for (name, count) in items if count >= 2];
    assert_eq!(filtered, vec!["b", "c"]);
}

