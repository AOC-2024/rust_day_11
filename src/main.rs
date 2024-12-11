use day_11::count_stones;

fn main() {
    let stones_after_25_blinks = count_stones("src/resources/puzzle.txt", 25);

    println!("Stones after 25 blinks: {}", stones_after_25_blinks);
}
