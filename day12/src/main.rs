// --- Day 12: Subterranean Sustainability ---
//
// The year 518 is significantly more underground than your history books implied. Either that, or you've arrived in a vast cavern network under the North Pole.
//
// After exploring a little, you discover a long tunnel that contains a row of small pots as far as you can see to your left and right. A few of them contain plants - someone is trying to grow things in these geothermally-heated caves.
//
// The pots are numbered, with 0 in front of you. To the left, the pots are numbered -1, -2, -3, and so on; to the right, 1, 2, 3.... Your puzzle input contains a list of pots from 0 to the right and whether they do (#) or do not (.) currently contain a plant, the initial state. (No other pots currently contain plants.) For example, an initial state of #..##.... indicates that pots 0, 3, and 4 currently contain plants.
//
// Your puzzle input also contains some notes you find on a nearby table: someone has been trying to figure out how these plants spread to nearby pots. Based on the notes, for each generation of plants, a given pot has or does not have a plant based on whether that pot (and the two pots on either side of it) had a plant in the last generation. These are written as LLCRR => N, where L are pots to the left, C is the current pot being considered, R are the pots to the right, and N is whether the current pot will have a plant in the next generation. For example:
//
//     A note like ..#.. => . means that a pot that contains a plant but with no plants within two pots of it will not have a plant in it during the next generation.
//     A note like ##.## => . means that an empty pot with two plants on each side of it will remain empty in the next generation.
//     A note like .##.# => # means that a pot has a plant in a given generation if, in the previous generation, there were plants in that pot, the one immediately to the left, and the one two pots to the right, but not in the ones immediately to the right and two to the left.
//
// It's not clear what these plants are for, but you're sure it's important, so you'd like to make sure the current configuration of plants is sustainable by determining what will happen after 20 generations.
//
// For example, given the following input:
//
// initial state: #..#.#..##......###...###
//
// ...## => #
// ..#.. => #
// .#... => #
// .#.#. => #
// .#.## => #
// .##.. => #
// .#### => #
// #.#.# => #
// #.### => #
// ##.#. => #
// ##.## => #
// ###.. => #
// ###.# => #
// ####. => #
//
// For brevity, in this example, only the combinations which do produce a plant are listed. (Your input includes all possible combinations.) Then, the next 20 generations will look like this:
//
//                  1         2         3
//        0         0         0         0
//  0: ...#..#.#..##......###...###...........
//  1: ...#...#....#.....#..#..#..#...........
//  2: ...##..##...##....#..#..#..##..........
//  3: ..#.#...#..#.#....#..#..#...#..........
//  4: ...#.#..#...#.#...#..#..##..##.........
//  5: ....#...##...#.#..#..#...#...#.........
//  6: ....##.#.#....#...#..##..##..##........
//  7: ...#..###.#...##..#...#...#...#........
//  8: ...#....##.#.#.#..##..##..##..##.......
//  9: ...##..#..#####....#...#...#...#.......
// 10: ..#.#..#...#.##....##..##..##..##......
// 11: ...#...##...#.#...#.#...#...#...#......
// 12: ...##.#.#....#.#...#.#..##..##..##.....
// 13: ..#..###.#....#.#...#....#...#...#.....
// 14: ..#....##.#....#.#..##...##..##..##....
// 15: ..##..#..#.#....#....#..#.#...#...#....
// 16: .#.#..#...#.#...##...#...#.#..##..##...
// 17: ..#...##...#.#.#.#...##...#....#...#...
// 18: ..##.#.#....#####.#.#.#...##...##..##..
// 19: .#..###.#..#.#.#######.#.#.#..#.#...#..
// 20: .#....##....#####...#######....#.#..##.
//
// The generation is shown along the left, where 0 is the initial state. The pot numbers are shown along the top, where 0 labels the center pot, negative-numbered pots extend to the left, and positive pots extend toward the right. Remember, the initial state begins at pot 0, which is not the leftmost pot used in this example.
//
// After one generation, only seven plants remain. The one in pot 0 matched the rule looking for ..#.., the one in pot 4 matched the rule looking for .#.#., pot 9 matched .##.., and so on.
//
// In this example, after 20 generations, the pots shown as # contain plants, the furthest left of which is pot -2, and the furthest right of which is pot 34. Adding up all the numbers of plant-containing pots after the 20th generation produces 325.
//
// After 20 generations, what is the sum of the numbers of all pots which contain a plant?
//
// You realize that 20 generations aren't enough. After all, these plants will need to last another 1500 years to even reach your timeline, not to mention your future.
//
// After fifty billion (50000000000) generations, what is the sum of the numbers of all pots which contain a plant?
//

fn change(state: &mut str, rules: &Vec<&str>, pot_zero_at: i64) -> (String, i64) {
    let mut current_state = ".....".to_owned();
    current_state.push_str(state);
    current_state.push_str("..........");

    let mut new_state = "".to_owned();
    for i in 0..current_state.len()-5 {
        let current_range: String = current_state.clone().drain(i..i+5).collect();
        let next = if rules.iter().any(|r| r == &current_range) { "#" } else { "." };
        new_state.push_str(next);
    }

    let new_pod_zero_at = pot_zero_at + 3 - (new_state.clone().find('#').unwrap_or(3) as i64);

    return (new_state, new_pod_zero_at);
}

fn main() {
    let mut state = String::from("##.#############........##.##.####..#.#..#.##...###.##......#.#..#####....##..#####..#.#.##.#.##");
    let rules = "###.# => #
        .#### => #
        #.### => .
        .##.. => .
        ##... => #
        ##.## => #
        .#.## => #
        #.#.. => #
        #...# => .
        ...## => #
        ####. => #
        #..## => .
        #.... => .
        .###. => .
        ..#.# => .
        ..### => .
        #.#.# => #
        ..... => .
        ..##. => .
        ##.#. => #
        .#... => #
        ##### => .
        ###.. => #
        ..#.. => .
        ##..# => #
        #..#. => #
        #.##. => .
        ....# => .
        .#..# => #
        .#.#. => #
        .##.# => .
        ...#. => .";

    let parsed_rules: Vec<&str> = rules
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.ends_with("#"))
        .map(|l| l.split_at(5).0)
        .collect();

    let mut pot_zero_at: i64 = 0;
    let y: i64 = 50000000000;

    for i in 1..=y {
        let (new_state, new_pot_zero_at) = change(&mut state, &parsed_rules, pot_zero_at);
        pot_zero_at = new_pot_zero_at;
        let trimmed_new_state = new_state.as_str().trim_start_matches('.').trim_end_matches('.').to_owned();

        if state == trimmed_new_state {
            pot_zero_at = pot_zero_at - ((y - i) as i64);
            break;
        }

        state = trimmed_new_state;
    }

    let mut sum: i64 = 0;

    for c in state.char_indices() {
        if let (i, '#') = c {
            sum += i as i64 - pot_zero_at;
        }
    }

    println!("{}", sum);
}
