// --- Day 3: No Matter How You Slice It ---
// The Elves managed to locate the chimney-squeeze prototype fabric for Santa's
// suit (thanks to someone who helpfully wrote its box IDs on the wall of the
// warehouse in the middle of the night). Unfortunately, anomalies are still affecting
// them - nobody can even agree on how to cut the fabric.
//
// The whole piece of fabric they're working on is a very large square - at least 1000
// inches on each side.
//
// Each Elf has made a claim about which area of fabric would be ideal for Santa's suit.
// All claims have an ID and consist of a single rectangle with edges parallel to the edges of
// the fabric. Each claim's rectangle is defined as follows:
//
//     - The number of inches between the left edge of the fabric and the left edge of the rectangle.
//     - The number of inches between the top edge of the fabric and the top edge of the rectangle.
//     - The width of the rectangle in inches.
//     - The height of the rectangle in inches.
//
// A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from
// the left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall. Visually,
// it claims the square inches of fabric represented by # (and ignores the square inches of
// fabric represented by .) in the diagram below:
//
// ...........
// ...........
// ...#####...
// ...#####...
// ...#####...
// ...#####...
// ...........
// ...........
// ...........
//
// The problem is that many of the claims overlap, causing two or more claims to cover
// part of the same areas. For example, consider the following claims:
//
// #1 @ 1,3: 4x4
// #2 @ 3,1: 4x4
// #3 @ 5,5: 2x2
//
// Visually, these claim the following areas:
//
// ........
// ...2222.
// ...2222.
// .11XX22.
// .11XX22.
// .111133.
// .111133.
// ........
//
// The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while
// adjacent to the others, does not overlap either of them.)
//
// If the Elves all proceed with their own plans, none of them will have enough fabric. How
// many square inches of fabric are within two or more claims?
//
// Your puzzle answer was 119551.
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
// Amidst the chaos, you notice that exactly one claim doesn't overlap by even a single square
// inch of fabric with any other claim. If you can somehow draw attention to it, maybe
// the Elves will be able to make Santa's suit after all!
//
// For example, in the claims above, only claim 3 is intact after all claims are made.
//
// What is the ID of the only claim that doesn't overlap?

use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Claim {
    fn from_str(claim: &str) -> Claim {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }
        let cap = RE.captures(claim).unwrap();
        Claim {
            id: cap[1].parse().unwrap(),
            x: cap[2].parse().unwrap(),
            y: cap[3].parse().unwrap(),
            w: cap[4].parse().unwrap(),
            h: cap[5].parse().unwrap(),
        }
    }
}

fn main() {
    let mut fabric: HashSet<(u32, u32)> = HashSet::new();
    let mut overlapping: HashSet<(u32, u32)> = HashSet::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let claim = Claim::from_str(&line.unwrap());
        for y in claim.y..claim.y + claim.h {
            for x in claim.x..claim.x + claim.w {
                if !fabric.insert((x, y)) {
                    overlapping.insert((x, y));
                }
            }
        }
    }

    println!("{:?}", overlapping.iter().count());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_claim() {
        let input = "#14 @ 206,626: 20x28";
        let expected = Claim {
            id: 14,
            x: 206,
            y: 626,
            w: 20,
            h: 28,
        };

        assert_eq!(Claim::from_str(&input), expected);
    }
}
