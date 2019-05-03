// A crop of this size requires significant logistics to transport produce, soil, fertilizer, and so on. The Elves are very busy pushing things around in carts on some kind of rudimentary system of tracks they've come up with.
//
// Seeing as how cart-and-track systems don't appear in recorded history for another 1000 years, the Elves seem to be making this up as they go along. They haven't even figured out how to avoid collisions yet.
//
// You map out the tracks (your puzzle input) and see where you can help.
//
// Tracks consist of straight paths (| and -), curves (/ and \), and intersections (+). Curves connect exactly two perpendicular pieces of track; for example, this is a closed loop:
//
// /----\
// |    |
// |    |
// \----/
//
// Intersections occur when two perpendicular paths cross. At an intersection, a cart is capable of turning left, turning right, or continuing straight. Here are two loops connected by two intersections:
//
// /-----\
// |     |
// |  /--+--\
// |  |  |  |
// \--+--/  |
//    |     |
//    \-----/
//
// Several carts are also on the tracks. Carts always face either up (^), down (v), left (<), or right (>). (On your initial map, the track under each cart is a straight path matching the direction the cart is facing.)
//
// Each time a cart has the option to turn (by arriving at any intersection), it turns left the first time, goes straight the second time, turns right the third time, and then repeats those directions starting again with left the fourth time, straight the fifth time, and so on. This process is independent of the particular intersection at which the cart has arrived - that is, the cart has no per-intersection memory.
//
// Carts all move at the same speed; they take turns moving a single step at a time. They do this based on their current location: carts on the top row move first (acting from left to right), then carts on the second row move (again from left to right), then carts on the third row, and so on. Once each cart has moved one step, the process repeats; each of these loops is called a tick.
//
// For example, suppose there are two carts on a straight track:
//
// |  |  |  |  |
// v  |  |  |  |
// |  v  v  |  |
// |  |  |  v  X
// |  |  ^  ^  |
// ^  ^  |  |  |
// |  |  |  |  |
//
// First, the top cart moves. It is facing down (v), so it moves down one square. Second, the bottom cart moves. It is facing up (^), so it moves up one square. Because all carts have moved, the first tick ends. Then, the process repeats, starting with the first cart. The first cart moves down, then the second cart moves up - right into the first cart, colliding with it! (The location of the crash is marked with an X.) This ends the second and last tick.
//
// Here is a longer example:
//
// /->-\
// |   |  /----\
// | /-+--+-\  |
// | | |  | v  |
// \-+-/  \-+--/
//   \------/
//
// /-->\
// |   |  /----\
// | /-+--+-\  |
// | | |  | |  |
// \-+-/  \->--/
//   \------/
//
// /---v
// |   |  /----\
// | /-+--+-\  |
// | | |  | |  |
// \-+-/  \-+>-/
//   \------/
//
// /---\
// |   v  /----\
// | /-+--+-\  |
// | | |  | |  |
// \-+-/  \-+->/
//   \------/
//
// /---\
// |   |  /----\
// | /->--+-\  |
// | | |  | |  |
// \-+-/  \-+--^
//   \------/
//
// /---\
// |   |  /----\
// | /-+>-+-\  |
// | | |  | |  ^
// \-+-/  \-+--/
//   \------/
//
// /---\
// |   |  /----\
// | /-+->+-\  ^
// | | |  | |  |
// \-+-/  \-+--/
//   \------/
//
// /---\
// |   |  /----<
// | /-+-->-\  |
// | | |  | |  |
// \-+-/  \-+--/
//   \------/
//
// /---\
// |   |  /---<\
// | /-+--+>\  |
// | | |  | |  |
// \-+-/  \-+--/
//   \------/
//
// /---\
// |   |  /--<-\
// | /-+--+-v  |
// | | |  | |  |
// \-+-/  \-+--/
//   \------/
//
// /---\
// |   |  /-<--\
// | /-+--+-\  |
// | | |  | v  |
// \-+-/  \-+--/
//   \------/
//
// /---\
// |   |  /<---\
// | /-+--+-\  |
// | | |  | |  |
// \-+-/  \-<--/
//   \------/
//
// /---\
// |   |  v----\
// | /-+--+-\  |
// | | |  | |  |
// \-+-/  \<+--/
//   \------/
//
// /---\
// |   |  /----\
// | /-+--v-\  |
// | | |  | |  |
// \-+-/  ^-+--/
//   \------/
//
// /---\
// |   |  /----\
// | /-+--+-\  |
// | | |  X |  |
// \-+-/  \-+--/
//   \------/
//
// After following their respective paths for a while, the carts eventually crash. To help prevent crashes, you'd like to know the location of the first crash. Locations are given in X,Y coordinates, where the furthest left column is X=0 and the furthest top row is Y=0:
//
//            111
//  0123456789012
// 0/---\
// 1|   |  /----\
// 2| /-+--+-\  |
// 3| | |  X |  |
// 4\-+-/  \-+--/
// 5  \------/
//
// In this example, the location of the first crash is 7,3.
//
// --- Part Two ---
//
// There isn't much you can do to prevent crashes in this ridiculous system. However, by predicting the crashes, the Elves know where to be in advance and instantly remove the two crashing carts the moment any crash occurs.
//
// They can proceed like this for a while, but eventually, they're going to run out of carts. It could be useful to figure out where the last cart that hasn't crashed will end up.
//
// For example:
//
// />-<\
// |   |
// | /<+-\
// | | | v
// \>+</ |
//   |   ^
//   \<->/
//
// /---\
// |   |
// | v-+-\
// | | | |
// \-+-/ |
//   |   |
//   ^---^
//
// /---\
// |   |
// | /-+-\
// | v | |
// \-+-/ |
//   ^   ^
//   \---/
//
// /---\
// |   |
// | /-+-\
// | | | |
// \-+-/ ^
//   |   |
//   \---/
//
// After four very expensive crashes, a tick ends with only one cart remaining; its final location is 6,4.
//
// What is the location of the last cart at the end of the first tick where it is the only cart left?

use std::fs;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]
enum TurnedTowards {
    Left,
    Straight,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct Cart {
    id: String,
    x: usize,
    y: usize,
    direction: Direction,
    turned_towards_last: TurnedTowards,
}
impl Cart {
    fn new(x: usize, y: usize, direction: Direction) -> Cart {
        Cart {
            id: format!("{}-{}", x, y),
            x: x,
            y: y,
            direction: direction,
            turned_towards_last: TurnedTowards::Right
        }
    }
    fn change_direction(&mut self) {
        match self.turned_towards_last {
            TurnedTowards::Left => {
                self.turned_towards_last = TurnedTowards::Straight;
                self.walk();
            },
            TurnedTowards::Straight => {
                self.turned_towards_last = TurnedTowards::Right;
                self.turn_right();
            },
            TurnedTowards::Right => {
                self.turned_towards_last = TurnedTowards::Left;
                self.turn_left();
            },
        }
    }
    fn turn_left(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Left,
            Direction::Down => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Down,
            Direction::Right => self.direction = Direction::Up,
        }
        self.walk();
    }
    fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
        self.walk();
    }
    fn walk(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
    fn horizontal(&self) -> bool {
        self.direction == Direction::Left || self.direction == Direction::Right
    }
    fn vertical(&self) -> bool {
        !self.horizontal()
    }
    fn position(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
    fn update_position(&mut self, position: Point) {
        self.x = position.x;
        self.y = position.y;
    }
}
impl Eq for Cart {}
impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.id == other.id
    }
}

fn collect_carts(map: &Vec<Vec<char>>) -> Vec<Cart> {
    let mut carts: Vec<Cart> = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                '^' => carts.push(Cart::new(x, y, Direction::Up)),
                'v' => carts.push(Cart::new(x, y, Direction::Down)),
                '<' => carts.push(Cart::new(x, y, Direction::Left)),
                '>' => carts.push(Cart::new(x, y, Direction::Right)),
                _ => (),
            }
        }
    }

    carts
}

fn move_cart(cart: &mut Cart, map: &Vec<Vec<char>>) {
    let position = map[cart.y][cart.x];

    if position == '+' {
        cart.change_direction();
    } else if position == '/' && cart.horizontal() {
        cart.turn_left();
    } else if position == '/' && cart.vertical() {
        cart.turn_right();
    } else if position == '\\' && cart.horizontal() {
        cart.turn_right();
    } else if position == '\\' && cart.vertical() {
        cart.turn_left();
    } else {
        cart.walk();
    }
}

fn find_duplicate(point: &Point, points: &Vec<Point>) -> Option<Point> {
    points.iter().find(|p| p == &point).map(|p| Point { x: p.x, y: p.y })
}

fn move_carts(map: &mut Vec<Vec<char>>, carts: &mut Vec<Cart>, crashes: &mut Vec<Point>) -> Vec<Cart> {
    let mut i = 0;
    let mut positions = carts.iter().map(|c| c.position()).collect();
    let mut crashed_carts: Vec<Cart> = Vec::new();
    let mut carts_copy = carts.clone();

    for cart in carts {
        move_cart(cart, map);
        carts_copy[i].update_position(cart.position().clone());

        if let Some(crashed_at) = find_duplicate(&cart.position(), &positions) {
            crashes.push(crashed_at.clone());

            carts_copy.iter().filter(|c| {
                c.position() == crashed_at
            }).for_each(|c| crashed_carts.push(c.clone()));
        }

        positions[i] = cart.position().clone();
        i += 1;
    }

    crashed_carts
}

fn sort_carts(carts: &mut Vec<Cart>) {
    carts.sort_by(|a, b| {
        match a.y.cmp(&b.y) {
            Ordering::Equal => a.x.cmp(&b.x),
            x => x,
        }
    });
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut map: Vec<Vec<char>> = contents.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut carts: Vec<Cart> = collect_carts(&map);
    let mut crashes: Vec<Point> = Vec::new();

    loop {
        let crashed_carts = move_carts(&mut map, &mut carts, &mut crashes);
        carts.retain(|c| !crashed_carts.contains(c));

        if carts.len() == 1 {
            println!("{:?}", crashes[0]);
            println!("{:?}", carts[0].position());
            break;
        }

        sort_carts(&mut carts);
    }
}
