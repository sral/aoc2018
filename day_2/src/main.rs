// --- Day 2: Inventory Management System ---
// You stop falling through time, catch your breath, and check the screen on the device.
// "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10."
// You made it! Now, to find those anomalies.
//
// Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either.
// But now that so many people have chimneys, maybe he could sneak in that way?" Another voice
// responds, "Actually, we've been working on a new kind of suit that would let him fit through
// tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric,
// the design plans, everything! Nobody on the team can even seem to remember important details of
// the project!"
//
// "Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored
// together, so the box IDs should be similar. Too bad it would take forever to search the warehouse
// for two similar box IDs..." They walk too far away to hear any more.
//
// Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if
// you were discovered - and use your fancy wrist device to quickly scan every box and produce a list of
// the likely candidates (your puzzle input).
//
// To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number
// that have an ID containing exactly two of any letter and then separately counting those with exactly
// three of any letter. You can multiply those two counts together to get a rudimentary checksum and
// compare it to what your device predicts.
//
// For example, if you see the following box IDs:
//
//     - abcdef contains no letters that appear exactly two or three times.
//     - bababc contains two a and three b, so it counts for both.
//     - abbcde contains two b, but no letter appears exactly three times.
//     - abcccd contains three c, but no letter appears exactly two times.
//     - aabcdd contains two a and two d, but it only counts once.
//     - abcdee contains two e.
//     - ababab contains three a and three b, but it only counts once.
//
// Of these box IDs, four of them contain a letter which appears exactly twice, and three of them
// contain a letter which appears exactly three times. Multiplying these together produces a
// checksum of 4 * 3 = 12.
//
// What is the checksum for your list of box IDs?
//
// Your puzzle answer was 4920.
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
// Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.
//
// The boxes will have IDs which differ by exactly one character at the same position in both strings. For
// example, given the following box IDs:
//
//     - abcde
//     - fghij
//     - klmno
//     - pqrst
//     - fguij
//     - axcye
//     - wvxyz
//
// The IDs abcde and axcye are close, but they differ by two characters (the second and fourth).
// However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must
// be the correct boxes.
//
// What letters are common between the two correct box IDs? (In the example above, this is found by removing
// the differing character from either ID, producing fgij.)
//
// Your puzzle answer was fonbwmjquwtapeyzikghtvdxl.
//
// Both parts of this puzzle are complete! They provide two gold stars: **

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn count_occurrences(box_id: &str) -> (i32, i32) {
    let mut frequencies = HashMap::new();
    for c in box_id.chars() {
        let count = frequencies.entry(c).or_insert(0);
        *count += 1;
    }

    let mut result = (0, 0);
    for val in frequencies.values() {
        if *val == 2 {
            result.0 = 1;
        } else if *val == 3 {
            result.1 = 1;
        }
        if result == (1, 1) {
            return result;
        }
    }
    result
}

fn calculate_checksum(box_ids: &[String]) -> i32 {
    let mut sum = (0, 0);
    for id in box_ids {
        let c = count_occurrences(&id);
        sum.0 += c.0;
        sum.1 += c.1;
    }
    sum.0 * sum.1
}

fn get_common_letters(first: &str, second: &str) -> String {
    let mut common = String::new();
    for chars in first.chars().zip(second.chars()) {
        if chars.0 == chars.1 {
            common.push(chars.0);
        }
    }
    common
}

fn differs_by_one(first: &str, second: &str) -> bool {
    let mut diff_count = 0;
    for chars in first.chars().zip(second.chars()) {
        if chars.0 != chars.1 {
            diff_count += 1;
        }
        if diff_count > 1 {
            return false;
        }
    }
    diff_count == 1
}

fn find_prototype_boxes(box_ids: &[String]) -> Option<(&str, &str)> {
    for i in 0..box_ids.len() - 1 {
        for j in i + 1..box_ids.len() {
            if differs_by_one(&box_ids[i], &box_ids[j]) {
                return Some((&box_ids[i], &box_ids[j]));
            }
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let box_ids: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();
    println!("checksum: {}", calculate_checksum(&box_ids));
    let (first, second) = find_prototype_boxes(&box_ids).unwrap();
    println!(
        "common prototype box id letters: {}",
        get_common_letters(first, second)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_examples_test() {
        assert_eq!(count_occurrences("abcdef"), (0, 0));
        assert_eq!(count_occurrences("bababc"), (1, 1));
        assert_eq!(count_occurrences("abbcde"), (1, 0));
        assert_eq!(count_occurrences("abcccd"), (0, 1));
        assert_eq!(count_occurrences("aabcdd"), (1, 0));
        assert_eq!(count_occurrences("abcdee"), (1, 0));
        assert_eq!(count_occurrences("ababab"), (0, 1));

        assert_eq!(
            calculate_checksum(&vec![
                "abcdef".to_string(),
                "bababc".to_string(),
                "abbcde".to_string(),
                "abcccd".to_string(),
                "aabcdd".to_string(),
                "abcdee".to_string(),
                "ababab".to_string()
            ]),
            12
        );
    }

    #[test]
    fn part_two_examples_test() {
        assert_eq!(differs_by_one("fghij", "fguij"), true);
        assert_eq!(differs_by_one("abcde", "axcye"), false);

        assert_eq!(
            find_prototype_boxes(&vec![
                "abcde".to_string(),
                "fghij".to_string(),
                "klmno".to_string(),
                "pqrst".to_string(),
                "fguij".to_string(),
                "axcye".to_string(),
                "wvxyz".to_string()
            ]),
            Some(("fghij", "fguij"))
        );

        assert_eq!(get_common_letters(&"fghij", &"fguij"), "fgij");
    }
}
