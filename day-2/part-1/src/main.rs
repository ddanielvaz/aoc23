use std::collections::HashMap;
use std::fs::read_to_string;

fn parse_line(line : &str) ->  u32 {
    let bag = HashMap::from([("red",12), ("green",13), ("blue",14)]);
    let splitted = line.split(":").collect::<Vec<&str>>();
    let game_id = splitted[0].split(" ").last().unwrap();
    for game in splitted[1].split(";") {
        for cubes in game.trim().split(", "){
            let drawn : Vec<&str> = cubes.split(" ").collect();
            if bag.get(drawn[1]).unwrap() >= &drawn[0].parse::<u32>().unwrap() {
                continue
            }
            else {
                return 0
            }
        }
    }
    game_id.parse::<u32>().unwrap()
}

fn main() {
    let filename = "../input.txt";
    let mut sum = 0;
    for line in read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
    {
        sum += parse_line(line);
    }
    println!("sum = {}", sum);
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;
    use crate::GameSet;
    use crate::parse_line;

    #[test]
    fn parse_line_test() {
        assert_eq!(parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 1);
        assert_eq!(parse_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 2);
        assert_eq!(parse_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 0);
        assert_eq!(parse_line("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 0);
        assert_eq!(parse_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 5);
        assert_eq!(parse_line("Game 10: 18 red, 11 green, 3 blue; 2 blue, 19 red, 7 green; 4 green, 1 blue, 6 red; 4 green, 2 red, 4 blue; 10 green, 5 red, 2 blue; 13 red, 12 green, 4 blue"), 0);
    }
}
    