use day_11::count_stones;

#[test]
fn should_count_stones_after_25_blinks() {
    assert_eq!(count_stones("tests/resources/puzzle.txt", 25), 55312);
}