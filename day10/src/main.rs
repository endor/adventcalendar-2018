// --- Day 10: The Stars Align ---
// It's no use; your navigation system simply isn't capable of providing walking directions in the arctic circle, and certainly not in 1018.
//
// The Elves suggest an alternative. In times like these, North Pole rescue operations will arrange points of light in the sky to guide missing Elves back to base. Unfortunately, the message is easy to miss: the points move slowly enough that it takes hours to align them, but have so much momentum that they only stay aligned for a second. If you blink at the wrong time, it might be hours before another message appears.
//
// You can see these points of light floating in the distance, and record their position in the sky and their velocity, the relative change in position per second (your puzzle input). The coordinates are all given from your perspective; given enough time, those positions and velocities will move the points into a cohesive message!
//
// Rather than wait, you decide to fast-forward the process and calculate what the points will eventually spell.
//
// For example, suppose you note the following points:
//
// position=< 9,  1> velocity=< 0,  2>
// position=< 7,  0> velocity=<-1,  0>
// position=< 3, -2> velocity=<-1,  1>
// position=< 6, 10> velocity=<-2, -1>
// position=< 2, -4> velocity=< 2,  2>
// position=<-6, 10> velocity=< 2, -2>
// position=< 1,  8> velocity=< 1, -1>
// position=< 1,  7> velocity=< 1,  0>
// position=<-3, 11> velocity=< 1, -2>
// position=< 7,  6> velocity=<-1, -1>
// position=<-2,  3> velocity=< 1,  0>
// position=<-4,  3> velocity=< 2,  0>
// position=<10, -3> velocity=<-1,  1>
// position=< 5, 11> velocity=< 1, -2>
// position=< 4,  7> velocity=< 0, -1>
// position=< 8, -2> velocity=< 0,  1>
// position=<15,  0> velocity=<-2,  0>
// position=< 1,  6> velocity=< 1,  0>
// position=< 8,  9> velocity=< 0, -1>
// position=< 3,  3> velocity=<-1,  1>
// position=< 0,  5> velocity=< 0, -1>
// position=<-2,  2> velocity=< 2,  0>
// position=< 5, -2> velocity=< 1,  2>
// position=< 1,  4> velocity=< 2,  1>
// position=<-2,  7> velocity=< 2, -2>
// position=< 3,  6> velocity=<-1, -1>
// position=< 5,  0> velocity=< 1,  0>
// position=<-6,  0> velocity=< 2,  0>
// position=< 5,  9> velocity=< 1, -2>
// position=<14,  7> velocity=<-2,  0>
// position=<-3,  6> velocity=< 2, -1>
// Each line represents one point. Positions are given as <X, Y> pairs: X represents how far left (negative) or right (positive) the point appears, while Y represents how far up (negative) or down (positive) the point appears.
//
// At 0 seconds, each point has the position given. Each second, each point's velocity is added to its position. So, a point with velocity <1, -2> is moving to the right, but is moving upward twice as quickly. If this point's initial position were <3, 9>, after 3 seconds, its position would become <6, 3>.
//
// Over time, the points listed above would move like this:
//
// Initially:
// ........#.............
// ................#.....
// .........#.#..#.......
// ......................
// #..........#.#.......#
// ...............#......
// ....#.................
// ..#.#....#............
// .......#..............
// ......#...............
// ...#...#.#...#........
// ....#..#..#.........#.
// .......#..............
// ...........#..#.......
// #...........#.........
// ...#.......#..........
//
// After 1 second:
// ......................
// ......................
// ..........#....#......
// ........#.....#.......
// ..#.........#......#..
// ......................
// ......#...............
// ....##.........#......
// ......#.#.............
// .....##.##..#.........
// ........#.#...........
// ........#...#.....#...
// ..#...........#.......
// ....#.....#.#.........
// ......................
// ......................
//
// After 2 seconds:
// ......................
// ......................
// ......................
// ..............#.......
// ....#..#...####..#....
// ......................
// ........#....#........
// ......#.#.............
// .......#...#..........
// .......#..#..#.#......
// ....#....#.#..........
// .....#...#...##.#.....
// ........#.............
// ......................
// ......................
// ......................
//
// After 3 seconds:
// ......................
// ......................
// ......................
// ......................
// ......#...#..###......
// ......#...#...#.......
// ......#...#...#.......
// ......#####...#.......
// ......#...#...#.......
// ......#...#...#.......
// ......#...#...#.......
// ......#...#..###......
// ......................
// ......................
// ......................
// ......................
//
// After 4 seconds:
// ......................
// ......................
// ......................
// ............#.........
// ........##...#.#......
// ......#.....#..#......
// .....#..##.##.#.......
// .......##.#....#......
// ...........#....#.....
// ..............#.......
// ....#......#...#......
// .....#.....##.........
// ...............#......
// ...............#......
// ......................
// ......................
// After 3 seconds, the message appeared briefly: HI. Of course, your message will be much longer and will take many more seconds to appear.
//
// What message will eventually appear in the sky?
//
// Your puzzle answer was RGRKHKNA.
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
// Good thing you didn't have to wait, because that would have taken a long time - much longer than the 3 seconds in the example above.
//
// Impressed by your sub-hour communication capabilities, the Elves are curious: exactly how many seconds would they have needed to wait for that message to appear?

extern crate regex;

use std::fs;
use regex::Match;
use regex::Regex;

#[derive(Clone, Debug)]
struct PointWithVelocity {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

fn to_usize(cap: Option<Match>) -> i32 {
    cap.unwrap().as_str().parse::<i32>().unwrap_or(0)
}

fn move_points(points: &mut Vec<PointWithVelocity>) {
    for point in points {
        point.x += point.velocity_x;
        point.y += point.velocity_y;
    }
}

fn close_enough(points: &Vec<PointWithVelocity>) -> bool {
    let diff = 150;
    let min_x_ = min_x(points);
    let max_x_ = max_x(points);
    let min_y_ = min_y(points);
    let max_y_ = max_y(points);

    min_x_ - max_x_ < diff && min_x_ - max_x_ > -diff &&
        min_y_ - max_y_ < diff && min_y_ - max_y_ > -diff
}

fn draw(points: &Vec<PointWithVelocity>) {
    let coordinates: Vec<String> = points.iter().map(|p| format!("{:?},{:?}", p.x, p.y)).collect();

    for x in min_x(points)..=max_x(points) {
        for y in min_y(points)..=max_y(points) {
            if coordinates.contains(&format!("{:?},{:?}", x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn min_x(points: &Vec<PointWithVelocity>) -> i32 {
    points.iter().map(|p| p.x).min().unwrap()
}

fn max_x(points: &Vec<PointWithVelocity>) -> i32 {
    points.iter().map(|p| p.x).max().unwrap()
}

fn min_y(points: &Vec<PointWithVelocity>) -> i32 {
    points.iter().map(|p| p.y).min().unwrap()
}

fn max_y(points: &Vec<PointWithVelocity>) -> i32 {
    points.iter().map(|p| p.y).max().unwrap()
}

fn main() {
    let re = Regex::new(r"position=<\s?(-?\d+),\s?\s?(-?\d+)> velocity=<\s?(-?\d+),\s?\s?(-?\d+)>").unwrap();

    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut points: Vec<PointWithVelocity> = re.captures_iter(&contents).map(|cap| {
        PointWithVelocity {
            x: to_usize(cap.get(1)),
            y: to_usize(cap.get(2)),
            velocity_x: to_usize(cap.get(3)),
            velocity_y: to_usize(cap.get(4)),
        }
    }).collect();

    let mut seconds = 0;
    let mut abs_min_x = 10000;
    let mut cache: Vec<PointWithVelocity> = points.clone();

    while !close_enough(&points) {
        seconds += 1;
        move_points(&mut points);
    }

    for _ in 1.. {
        let min_x = min_x(&points);
        let max_x = max_x(&points);

        if max_x - min_x < abs_min_x {
            abs_min_x = max_x - min_x;
        } else {
            draw(&cache);
            seconds -= 1;
            break;
        }

        cache = points.clone();
        seconds += 1;
        move_points(&mut points);
    }

    println!("seconds: {:?}", seconds);
}
