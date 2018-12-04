// --- Day 2: Inventory Management System ---
// You stop falling through time, catch your breath, and check the screen on the device. "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.
//
// Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now that so many people have chimneys, maybe he could sneak in that way?" Another voice responds, "Actually, we've been working on a new kind of suit that would let him fit through tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans, everything! Nobody on the team can even seem to remember important details of the project!"
//
// "Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored together, so the box IDs should be similar. Too bad it would take forever to search the warehouse for two similar box IDs..." They walk too far away to hear any more.
//
// Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if you were discovered - and use your fancy wrist device to quickly scan every box and produce a list of the likely candidates (your puzzle input).
//
// To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number that have an ID containing exactly two of any letter and then separately counting those with exactly three of any letter. You can multiply those two counts together to get a rudimentary checksum and compare it to what your device predicts.
//
// For example, if you see the following box IDs:
//
// abcdef contains no letters that appear exactly two or three times.
// bababc contains two a and three b, so it counts for both.
// abbcde contains two b, but no letter appears exactly three times.
// abcccd contains three c, but no letter appears exactly two times.
// aabcdd contains two a and two d, but it only counts once.
// abcdee contains two e.
// ababab contains three a and three b, but it only counts once.
// Of these box IDs, four of them contain a letter which appears exactly twice, and three of them contain a letter which appears exactly three times. Multiplying these together produces a checksum of 4 * 3 = 12.
//
// What is the checksum for your list of box IDs?

// --- Part Two ---
// Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.
//
// The boxes will have IDs which differ by exactly one character at the same position in both strings. For example, given the following box IDs:
//
// abcde
// fghij
// klmno
// pqrst
// fguij
// axcye
// wvxyz
// The IDs abcde and axcye are close, but they differ by two characters (the second and fourth). However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must be the correct boxes.
//
// What letters are common between the two correct box IDs? (In the example above, this is found by removing the differing character from either ID, producing fgij.)

use std::fs;
use std::collections::HashMap;

// PART 1

type ByteHashMap = HashMap<u8, i32>;

struct Checksum {
    twice: i32,
    thrice: i32,
}
impl Checksum {
    fn calc(self) -> i32 {
        self.twice * self.thrice
    }

    fn update(self, inc_twice: bool, inc_thrice: bool) -> Checksum {
        Checksum {
            twice: self.twice + inc_twice as i32,
            thrice: self.thrice + inc_thrice as i32,
        }
    }
}

fn build_hash(id: &str) -> ByteHashMap {
    let mut hash = ByteHashMap::new();
    id.as_bytes().iter().for_each(|b| {
        let counter = hash.entry(*b).or_insert(0);
        *counter += 1;
    });
    hash
}

fn contains_twice(hash: &ByteHashMap) -> bool {
    hash.values().any(|&x| x == 2)
}

fn contains_thrice(hash: &ByteHashMap) -> bool {
    hash.values().any(|&x| x == 3)
}

fn calculate_checksum(ids: &[&str]) -> Checksum {
    ids.iter().fold(Checksum { twice: 0, thrice: 0 }, |acc, id| {
        let hash = build_hash(id);
        acc.update(contains_twice(&hash), contains_thrice(&hash))
    })
}

// PART 2

fn matches(id1: &str, id2: &str) -> Option<usize> {
    let mut difference: Option<usize> = None;

    for (i, bytes) in id1.as_bytes().iter().zip(id2.as_bytes().iter()).enumerate() {
        if bytes.0 != bytes.1 {
            if difference.is_none() {
                difference = Some(i);
            } else {
                difference = None;
                break;
            }
        }
    }

    difference
}

fn find_match(id: &str, ids: &[&str]) -> Option<usize> {
    ids.iter().find_map(|current_id| matches(current_id, id))
}

fn find_box_id<'a>(ids: &[&'a str]) -> Option<(&'a str, usize)> {
    let mut position = 0;
    ids.iter().enumerate().find(|(i, id)| {
        match find_match(id, &ids[i+1..]) {
            Some(pos) => {
                position = pos;
                true
            },
            None => false
        }
    }).map(|(_, id)| (*id, position))
}

fn common_letters((id, position): (&str, usize)) -> Option<String> {
    Some(format!("{}{}", &id[..position], &id[position+1..]))
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let ids: Vec<&str> = contents.lines().collect();

    // 7688
    let checksum = calculate_checksum(&ids[..]);
    println!("The checksum is: {}", checksum.calc());

    // lsrivmotzbdxpkxnaqmuwcchj
    let letters = find_box_id(&ids[..]).and_then(common_letters).unwrap_or("".to_string());
    println!("The common letters are: {}", letters);
}
