// --- Day 3: No Matter How You Slice It ---
// The Elves managed to locate the chimney-squeeze prototype fabric for Santa's suit (thanks to someone who helpfully wrote its box IDs on the wall of the warehouse in the middle of the night). Unfortunately, anomalies are still affecting them - nobody can even agree on how to cut the fabric.
//
// The whole piece of fabric they're working on is a very large square - at least 1000 inches on each side.
//
// Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims have an ID and consist of a single rectangle with edges parallel to the edges of the fabric. Each claim's rectangle is defined as follows:
//
// The number of inches between the left edge of the fabric and the left edge of the rectangle.
// The number of inches between the top edge of the fabric and the top edge of the rectangle.
// The width of the rectangle in inches.
// The height of the rectangle in inches.
// A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall. Visually, it claims the square inches of fabric represented by # (and ignores the square inches of fabric represented by .) in the diagram below:
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
// The problem is that many of the claims overlap, causing two or more claims to cover part of the same areas. For example, consider the following claims:
//
// #1 @ 1,3: 4x4
// #2 @ 3,1: 4x4
// #3 @ 5,5: 2x2
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
// The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to the others, does not overlap either of them.)
//
// If the Elves all proceed with their own plans, none of them will have enough fabric. How many square inches of fabric are within two or more claims?

// --- Part Two ---
// Amidst the chaos, you notice that exactly one claim doesn't overlap by even a single square inch of fabric with any other claim. If you can somehow draw attention to it, maybe the Elves will be able to make Santa's suit after all!
//
// For example, in the claims above, only claim 3 is intact after all claims are made.
//
// What is the ID of the only claim that doesn't overlap?
//

extern crate regex;

use regex::Match;
use regex::Regex;
use std::fs;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
struct Claim {
    id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}
impl Claim {
    fn x_range(self) -> std::ops::Range<u16> {
        self.x..(self.x+self.width)
    }

    fn y_range(self) -> std::ops::Range<u16> {
        self.y..(self.y+self.height)
    }
}

fn to_u16(cap: Option<Match>) -> u16 {
    cap.unwrap().as_str().parse::<u16>().unwrap_or(0)
}

fn main() {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let claims: Vec<Claim> = re.captures_iter(&contents).map(|cap| {
        Claim {
            id: to_u16(cap.get(1)),
            x: to_u16(cap.get(2)),
            y: to_u16(cap.get(3)),
            width: to_u16(cap.get(4)),
            height: to_u16(cap.get(5)),
        }
    }).collect();

    let mut claimed_once = HashSet::new();
    let mut claimed_twice = HashSet::new();

    claims.iter().for_each(|c| {
        for i in c.x_range() {
            for j in c.y_range() {
                let key = format!("{}-{}", i, j);
                if !claimed_once.insert(key.clone()) {
                    claimed_twice.insert(key.clone());
                }
            }
        }
    });

    println!("{} square inches of fabric are within two or more claims.", claimed_twice.len());

    let claim = claims.iter().find(|c| {
        let mut contains_claimed = false;
        for i in c.x_range() {
            for j in c.y_range() {
                let key = format!("{}-{}", i, j);
                if claimed_twice.contains(&key) {
                    contains_claimed = true;
                }
            }
        }
        !contains_claimed
    });

    match claim {
        Some(c) => println!("Claim #{} does not overlap with other claims.", c.id),
        None => println!("There are no claims that do not overlap."),
    }
}
