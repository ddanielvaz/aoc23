use std::io;
use std::fs::read_to_string;
use regex::Regex;


#[derive(PartialEq, Debug, Clone)]
struct Number {
    value: u32,
    first_idx: usize,
    last_idx: usize,
    touches: bool,
}

impl Number {
    fn analyze(&mut self, row: Option<&str>) {
        let line = row.unwrap();
        if self.touches {
            return;
        }
        if row != None {
            self.has_symbols_on_sides(line);
        }
    }

    fn has_symbols_on_sides(&mut self, line : &str) -> bool {
        const LEN : usize = 139;
        let first_idx : usize = if self.first_idx > 0 { self.first_idx-1 } else { self.first_idx };
        let last_idx : usize = if self.last_idx < LEN { self.last_idx+1 } else { self.last_idx };
        let re = Regex::new(r"[0-9.]").unwrap();
        for c in line.get(first_idx..last_idx+1) {
            //println!("substr {}", c);
            let result = re.replace_all(c, "");
            //println!("result {}", result);
            if result.len() > 0 {
                self.touches = true;
                return true
            }
        }
        return false
    }

    fn new(value: &String, indexes: &Vec<usize>) -> Number {
        Number{value: value.parse::<u32>().unwrap(), first_idx: *indexes.first().unwrap(), last_idx: *indexes.last().unwrap(), touches: false}
    }
}

fn parse_line(line: &str) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    let mut n : String = String::new();
    let mut indexes : Vec<usize> = Vec::new();
    for (idx, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            n.push(c);
            indexes.push(idx);
        } else if n.len() > 0 {
            let bla : Number = Number::new(&n, &indexes);
            n.clear();
            indexes.clear();
            numbers.push(bla);
        }
    }
    //
    if n.len() > 0 {
        let bla : Number = Number::new(&n, &indexes);
        n.clear();
        indexes.clear();
        numbers.push(bla);
    }
    numbers
}

fn main() {
    let filename = "../input.txt";
    let mut sum = 0;
    let mut last_line: Option<&str> = None;
    let mut last_numbers: Vec<Number> = Vec::new();
    for line in read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines()
    // split the string into an iterator of string slices
    {
        let mut answer = String::new();
        if last_line != None {
            println!("LAST {}", last_line.unwrap());
        }
        println!("CURRENT {}", line);
        if last_numbers.len() > 0 {
            for n in last_numbers.iter_mut() {
                n.analyze(Some(line));
                if n.touches {
                    sum += n.value;
                    println!("{}", n.value);
                }
            }
        }
        let mut current_numbers = parse_line(line);
        println!("CURRENT NUMBERS {:?}", current_numbers);
        println!("LAST NUMBERS {:?}", last_numbers);
        println!("");
        for n in current_numbers.iter_mut() {
            if last_line != None {
                n.analyze(last_line);
            }
            n.analyze(Some(line));
        }
        last_line = Some(line);
        last_numbers = current_numbers;
        println!("SUM {}", sum);
    }
    //
    if last_numbers.len() > 0 {
        for n in last_numbers {
            if n.touches {
                sum += n.value;
                println!("{}", n.value);
            }
        }
    }
    println!("sum = {}", sum);
}

#[cfg(test)]
mod tests {
    use crate::parse_line;
    use crate::Number;

    #[test]
    fn parse_line_test() {
        assert_eq!(parse_line("=467..114.."), Vec::from([Number{value: 467, first_idx:1, last_idx:3, touches:false}, Number { value: 114, first_idx: 6, last_idx: 8, touches: false }]) );
        assert_eq!(parse_line("...*......"), Vec::from([]));
        assert_eq!(parse_line("..35..633."), Vec::from([Number{value: 35, first_idx:2, last_idx:3, touches:false}, Number { value: 633, first_idx: 6, last_idx: 8, touches: false }]) );
        assert_eq!(parse_line("......#..."), Vec::from([]));
        assert_eq!(parse_line("617*......"), Vec::from([Number{value: 617, first_idx:0, last_idx:2, touches:false}]));
        assert_eq!(parse_line(".....+.58."), Vec::from([Number{value: 58, first_idx:7, last_idx:8, touches:false}]));
        assert_eq!(parse_line("..592.912."), Vec::from([Number{value: 592, first_idx:2, last_idx:4, touches:false}, Number{value: 912, first_idx:6, last_idx:8, touches:false}]));
        assert_eq!(parse_line(".......755"), Vec::from([Number{value: 755, first_idx:7, last_idx:9, touches:false}]));
        assert_eq!(parse_line("...$.*...."), Vec::from([]));
        assert_eq!(parse_line(".664.598.."), Vec::from([Number{value: 664, first_idx:1, last_idx:3, touches:false}, Number { value: 598, first_idx: 5, last_idx: 7, touches: false }]) );
    }

    #[test]
    fn has_symbols_on_sides_test() {
        let mut n = Number{value: 467, first_idx:0, last_idx:2, touches:false};
        assert_eq!(n.has_symbols_on_sides("...*......"), true);
        //
        n = Number { value: 114, first_idx: 5, last_idx: 7, touches: false };
        assert_eq!(n.has_symbols_on_sides("...*......"), false);
        //
        n = Number{value: 35, first_idx:2, last_idx:3, touches:false};
        assert_eq!(n.has_symbols_on_sides("...*......"), true);
        assert_eq!(n.has_symbols_on_sides("......#..."), false);
        //
        n = Number { value: 633, first_idx: 6, last_idx: 8, touches: false };
        assert_eq!(n.has_symbols_on_sides("...*......"), false);
        assert_eq!(n.has_symbols_on_sides("......#..."), true);
        //
        n = Number{value: 617, first_idx:0, last_idx:2, touches:false};
        assert_eq!(n.has_symbols_on_sides("617*......"), true);
        //
        n = Number{value: 755, first_idx:6, last_idx:8, touches:false};
        assert_eq!(n.has_symbols_on_sides("..592....."), false);
        assert_eq!(n.has_symbols_on_sides("...$.*...."), true);
    }
}
