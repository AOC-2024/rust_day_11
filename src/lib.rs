use std::fs::read_to_string;
use std::str::FromStr;

pub fn count_stones(input_path: &str, blinks: usize) -> usize {
    let puzzle = read_to_string(input_path).unwrap()
        .trim()
        .split_whitespace()
        .map(usize::from_str)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();

    let mut stone_count = 0;
    for stone in puzzle.iter() {
        stone_count += blink_stone_time(*stone, blinks).iter().count();
    }

    stone_count
}

fn blink_stone_time(stone_number: usize, times: usize) -> Vec<usize> {
    let mut current_stones = vec![stone_number];

    for _ in 0..times {
        let mut new_stones = Vec::new();
        for &stone in &current_stones {
            new_stones.extend(blink_stone(stone));
        }
        current_stones = new_stones;
    }
    current_stones
}


fn blink_stone(stone_number: usize) -> Vec<usize> {
    if stone_number == 0 {
        return vec![1];
    }

    let stone_number_string = stone_number.to_string();
    if stone_number_string.len() % 2 == 0 {
        vec![stone_number_string[..stone_number_string.len() / 2].parse().unwrap(), stone_number_string[stone_number_string.len() / 2..].parse().unwrap()]
    } else {
        vec![stone_number * 2024]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_blink_multiply_times_3() {
        assert_eq!(blink_stone_time(125, 2), vec![512072, 1]);
    }

    #[test]
    fn should_blink_multiply_times_2() {
        assert_eq!(blink_stone_time(125, 2), vec![253, 0]);
    }

    #[test]
    fn should_blink_multiply_times_1() {
        assert_eq!(blink_stone_time(125, 1), vec![253000]);
    }

    #[test]
    fn should_blink_multiply_by_2024_when_no_rules_appplied() {
        assert_eq!(blink_stone(1), vec![2024]);
    }

    #[test]
    fn should_split_in_two_stones_when_digits_even_and_remove_leading_0() {
        assert_eq!(blink_stone(109007), vec![109, 7]);
    }

    #[test]
    fn should_split_in_two_stones_when_digits_even() {
        assert_eq!(blink_stone(123456), vec![123, 456]);
    }

    #[test]
    fn should_blink_return_one_if_stone_number_is_0() {
        assert_eq!(blink_stone(0), vec![1]);
    }
}