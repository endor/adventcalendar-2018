// --- Day 6: Chronal Coordinates ---
// The device on your wrist beeps several times, and once again you feel like you're falling.
//
// "Situation critical," the device announces. "Destination indeterminate. Chronal interference detected. Please specify new target coordinates."
//
// The device then produces a list of coordinates (your puzzle input). Are they places it thinks are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a manual.
//
// If they're dangerous, maybe you can minimize the danger by finding the coordinate that gives the largest distance from the other points.
//
// Using only the Manhattan distance, determine the area around each coordinate by counting the number of integer X,Y locations that are closest to that coordinate (and aren't tied in distance to any other coordinate).
//
// Your goal is to find the size of the largest area that isn't infinite. For example, consider the following list of coordinates:
//
// 1, 1
// 1, 6
// 8, 3
// 3, 4
// 5, 5
// 8, 9
// If we name these coordinates A through F, we can draw them on a grid, putting 0,0 at the top left:
//
// ..........
// .A........
// ..........
// ........C.
// ...D......
// .....E....
// .B........
// ..........
// ..........
// ........F.
// This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan distance, each location's closest coordinate can be determined, shown here in lowercase:
//
// aaaaa.cccc
// aAaaa.cccc
// aaaddecccc
// aadddeccCc
// ..dDdeeccc
// bb.deEeecc
// bBb.eeee..
// bbb.eeefff
// bbb.eeffff
// bbb.ffffFf
// Locations shown as . are equally far from two or more coordinates, and so they don't count as being closest to any.
//
// In this example, the areas of coordinates A, B, C, and F are infinite - while not shown here, their areas extend forever outside the visible grid. However, the areas of coordinates D and E are finite: D is closest to 9 locations, and E is closest to 17 (both including the coordinate's location itself). Therefore, in this example, the size of the largest area is 17.
//
// What is the size of the largest area that isn't infinite?
//
//
// --- Part Two ---
// On the other hand, if the coordinates are safe, maybe the best you can do is try to find a region near as many coordinates as possible.
//
// For example, suppose you want the sum of the Manhattan distance to all of the coordinates to be less than 32. For each location, add up the distances to all of the given coordinates; if the total of those distances is less than 32, that location is within the desired region. Using the same coordinates as above, the resulting region looks like this:
//
// ..........
// .A........
// ..........
// ...###..C.
// ..#D###...
// ..###E#...
// .B.###....
// ..........
// ..........
// ........F.
// In particular, consider the highlighted location 4,3 located at the top middle of the region. Its calculation is as follows, where abs() is the absolute value function:
//
// Distance to coordinate A: abs(4-1) + abs(3-1) =  5
// Distance to coordinate B: abs(4-1) + abs(3-6) =  6
// Distance to coordinate C: abs(4-8) + abs(3-3) =  4
// Distance to coordinate D: abs(4-3) + abs(3-4) =  2
// Distance to coordinate E: abs(4-5) + abs(3-5) =  3
// Distance to coordinate F: abs(4-8) + abs(3-9) = 10
// Total distance: 5 + 6 + 4 + 2 + 3 + 10 = 30
// Because the total distance to all coordinates (30) is less than 32, the location is within the region.
//
// This region, which also includes coordinates D and E, has a total size of 16.
//
// Your actual region will need to be much larger than this example, though, instead including all locations with a total distance of less than 10000.
//
// What is the size of the region containing all locations which have a total distance to all given coordinates of less than 10000?
//

use std::fs;
use std::fmt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}
impl Coordinate {
    fn distance_from(&self, x: i32, y: i32) -> i32 {
        (x - self.x).abs() + (y - self.y).abs()
    }
}
impl Eq for Coordinate {}
impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn get_edge_coordinates(grid: Vec<Vec<&Coordinate>>, max_x: usize, max_y: usize) -> Vec<&Coordinate> {
    let mut c = Vec::new();

    for x in 0..max_x {
        c.push(grid[x][0]);
        c.push(grid[x][max_y-1]);
    }

    for y in 0..max_y {
        c.push(grid[0][y]);
        c.push(grid[max_x-1][y]);
    }

    c
}

fn main() {
    let contents = fs::read_to_string("input.txt")
       .expect("Something went wrong reading the file");
    let coordinates: Vec<Coordinate> = contents.lines()
        .map(|line| line.split(", ").collect())
        .map(|s: Vec<&str>| Coordinate { x: s[0].parse().unwrap(), y: s[1].parse().unwrap() })
        .collect();
    let max_x = coordinates.iter().map(|c| c.x).max().unwrap() as usize;
    let max_y = coordinates.iter().map(|c| c.y).max().unwrap() as usize;
    let max_total = 10000;

    let default = &coordinates[0];
    let mut grid = vec![vec![default; max_y]; max_x];
    let mut areas: HashMap<&Coordinate, i32> = HashMap::new();
    let mut central_area = 0;

    for x in 0..max_x {
        for y in 0..max_y {
            let mut min_distance = 9999;
            let mut equidistant = false;
            let mut coordinate = default;
            let mut distances = 0;

            for c in coordinates.iter() {
                let distance = c.distance_from(x as i32, y as i32);
                distances += distance;

                if distance < min_distance {
                    coordinate = c;
                    min_distance = distance;
                    equidistant = false;
                } else if distance == min_distance {
                    equidistant = true;
                }
            }

            grid[x][y] = coordinate;

            if !equidistant {
                let area = areas.entry(coordinate).or_insert(0);
                *area += 1;
            }

            if distances < max_total {
                central_area += 1;
            }
        }
    }

    let edge_coordinates = get_edge_coordinates(grid, max_x, max_y);

    let middle_coordinates: Vec<&Coordinate> = coordinates.iter()
        .filter(|c| !edge_coordinates.contains(c)).collect();

    let largest = middle_coordinates.iter().max_by_key(|c| &areas[*c]).unwrap();
    println!("Largest area of a finite coordinate: {:?}", areas[largest]);

    println!("Area with locations with a total distance less than {:?}: {:?}", max_total, central_area);
}
