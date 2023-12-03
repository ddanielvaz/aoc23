use std::fs::read_to_string;

fn parse_line(line: &str) -> u32 {
    let splitted = line.split(":").collect::<Vec<&str>>();
    let mut min_red=0;
    let mut min_green=0;
    let mut min_blue=0;
    for game in splitted[1].split(";") {
        for cubes in game.trim().split(", ") {
            let drawn: Vec<&str> = cubes.split(" ").collect();
            let val = drawn[0].parse::<u32>().unwrap();
            match drawn[1] {
                "red" => if min_red < val { min_red = val; },
                "green" => if min_green < val { min_green = val; },
                "blue" => if min_blue < val { min_blue = val; },
                &_ => panic!("no way..."),
            }
        }
    }
    min_red * min_green * min_blue
}

fn main() {
    let filename = "../input.txt";
    let mut sum = 0;
    for line in read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines()
    // split the string into an iterator of string slices
    {
        sum += parse_line(line);
    }
    println!("sum = {}", sum);
}

#[cfg(test)]
mod tests {
    use crate::parse_line;

    #[test]
    fn parse_line_test() {
        assert_eq!(
            parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        assert_eq!(
            parse_line("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            12
        );
        assert_eq!(
            parse_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            1560
        );
        assert_eq!(
            parse_line("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            630
        );
        assert_eq!(
            parse_line("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            36
        );
    }
}
