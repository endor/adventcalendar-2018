// --- Day 9: Marble Mania ---
// You talk to the Elves while you wait for your navigation system to initialize. To pass the time, they introduce you to their favorite marble game.
//
// The Elves play this game by taking turns arranging the marbles in a circle according to very particular rules. The marbles are numbered starting with 0 and increasing by 1 until every marble has a number.
//
// First, the marble numbered 0 is placed in the circle. At this point, while it contains only a single marble, it is still a circle: the marble is both clockwise from itself and counter-clockwise from itself. This marble is designated the current marble.
//
// Then, each Elf takes a turn placing the lowest-numbered remaining marble into the circle between the marbles that are 1 and 2 marbles clockwise of the current marble. (When the circle is large enough, this means that there is one marble between the marble that was just placed and the current marble.) The marble that was just placed then becomes the current marble.
//
// However, if the marble that is about to be placed has a number which is a multiple of 23, something entirely different happens. First, the current player keeps the marble they would have placed, adding it to their score. In addition, the marble 7 marbles counter-clockwise from the current marble is removed from the circle and also added to the current player's score. The marble located immediately clockwise of the marble that was removed becomes the new current marble.
//
// For example, suppose there are 9 players. After the marble with value 0 is placed in the middle, each player (shown in square brackets) takes a turn. The result of each of those turns would produce circles of marbles like this, where clockwise is to the right and the resulting current marble is in parentheses:
//
// [-] (0)
// [1]  0 (1)
// [2]  0 (2) 1
// [3]  0  2  1 (3)
// [4]  0 (4) 2  1  3
// [5]  0  4  2 (5) 1  3
// [6]  0  4  2  5  1 (6) 3
// [7]  0  4  2  5  1  6  3 (7)
// [8]  0 (8) 4  2  5  1  6  3  7
// [9]  0  8  4 (9) 2  5  1  6  3  7
// [1]  0  8  4  9  2(10) 5  1  6  3  7
// [2]  0  8  4  9  2 10  5(11) 1  6  3  7
// [3]  0  8  4  9  2 10  5 11  1(12) 6  3  7
// [4]  0  8  4  9  2 10  5 11  1 12  6(13) 3  7
// [5]  0  8  4  9  2 10  5 11  1 12  6 13  3(14) 7
// [6]  0  8  4  9  2 10  5 11  1 12  6 13  3 14  7(15)
// [7]  0(16) 8  4  9  2 10  5 11  1 12  6 13  3 14  7 15
// [8]  0 16  8(17) 4  9  2 10  5 11  1 12  6 13  3 14  7 15
// [9]  0 16  8 17  4(18) 9  2 10  5 11  1 12  6 13  3 14  7 15
// [1]  0 16  8 17  4 18  9(19) 2 10  5 11  1 12  6 13  3 14  7 15
// [2]  0 16  8 17  4 18  9 19  2(20)10  5 11  1 12  6 13  3 14  7 15
// [3]  0 16  8 17  4 18  9 19  2 20 10(21) 5 11  1 12  6 13  3 14  7 15
// [4]  0 16  8 17  4 18  9 19  2 20 10 21  5(22)11  1 12  6 13  3 14  7 15
// [5]  0 16  8 17  4 18(19) 2 20 10 21  5 22 11  1 12  6 13  3 14  7 15
// [6]  0 16  8 17  4 18 19  2(24)20 10 21  5 22 11  1 12  6 13  3 14  7 15
// [7]  0 16  8 17  4 18 19  2 24 20(25)10 21  5 22 11  1 12  6 13  3 14  7 15
// The goal is to be the player with the highest score after the last marble is used up. Assuming the example above ends after the marble numbered 25, the winning score is 23+9=32 (because player 5 kept marble 23 and removed marble 9, while no other player got any points in this very short example game).
//
// Here are a few more examples:
//
// 10 players; last marble is worth 1618 points: high score is 8317
// 13 players; last marble is worth 7999 points: high score is 146373
// 17 players; last marble is worth 1104 points: high score is 2764
// 21 players; last marble is worth 6111 points: high score is 54718
// 30 players; last marble is worth 5807 points: high score is 37305
// What is the winning Elf's score?
//
//
// Your puzzle answer was 371284.
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
// Amused by the speed of your answer, the Elves are curious:
//
// What would the new winning Elf's score be if the number of the last marble were 100 times larger?
//

use std::collections::HashMap;

struct Marble {
    points: usize,
    left: usize,
    right: usize,
}

fn move_left(circle: &Vec<Marble>, current_position: usize, steps: usize) -> usize {
    let mut next_position = current_position;
    for _ in 0..steps {
        next_position = circle[next_position].left;
    }
    next_position
}

fn move_right(circle: &Vec<Marble>, current_position: usize, steps: usize) -> usize {
    let mut next_position = current_position;
    for _ in 0..steps {
        next_position = circle[next_position].right;
    }
    next_position
}

fn insert(circle: &mut Vec<Marble>, current_position: usize, points: usize) -> usize {
    let next_position = circle.len();

    let current_right = circle[current_position].right;
    circle.push(Marble { points: points, left: current_position, right: current_right });
    circle[current_position].right = next_position;
    circle[current_right].left = next_position;

    next_position
}

fn remove(circle: &mut Vec<Marble>, current_position: usize) -> usize {
    let left = circle[current_position].left;
    let right = circle[current_position].right;
    circle[left].right = right;
    circle[right].left = left;
    circle[current_position].points
}

fn main() {
    // I first used a very simple implementation with just a vector of points, but
    // that didn't scale up. I tried implementing a linked list, but that was quite
    // complicated with ownership issues when calling methods, etc. While similar
    // in complexity to this one, mine just didn't work:
    // https://github.com/udoprog/rust-advent-of-code-2018/blob/master/src/bin/day9.rs
    // The current implementation is based on the idea here:
    // https://github.com/Vzaa/advent_of_code_2018/blob/master/day9/src/main.rs
    // This seems like another good way to implement it:
    // https://github.com/fornwall/advent-of-code-2018-rs/blob/master/src/day9.rs

    let last_points = 7090400;
    let number_of_players = 473;

    let mut current_position: usize = 0;
    let mut current_player = 0;
    let mut circle = vec![Marble {points: 0, left: 0, right: 0}];
    let mut scores = HashMap::new();

    for points in 1..=last_points {
        if points % 23 == 0 {
            current_position = move_left(&circle, current_position, 7);
            let next_position = move_right(&circle, current_position, 1);
            let counter = scores.entry(current_player + 1).or_insert(0);
            *counter += points;
            *counter += remove(&mut circle, current_position);
            current_position = next_position;
        } else {
            current_position = move_right(&circle, current_position, 1);
            current_position = insert(&mut circle, current_position, points);
        }

        current_player = (current_player + 1) % number_of_players;
    }

    let high_score = scores.iter().max_by_key(|s| s.1);
    println!("{:?}", high_score);
}
