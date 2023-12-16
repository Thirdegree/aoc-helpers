
#[derive(aoc_helpers::TwoDArray)]
struct MyBasics {
    elems: Vec<Vec<u32>>
}

fn setup() -> MyBasics {
    MyBasics {
        elems: (0..100).map(|a| (0..10).map(|b| a * b).collect()).collect()
    }
}

#[test]
fn test_index_checking() {
    let basics = setup();
    assert_eq!(basics.x_len(), 10);
    assert_eq!(basics.y_len(), 100);
    assert!(basics.is_within_bounds((5, 50)));
    assert!(!basics.is_within_bounds((50, 5)));
    assert_eq!(basics[(5, 50)], (50 * 5) as u32)
}
