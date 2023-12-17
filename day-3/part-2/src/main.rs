use std::io;
use std::num;
use std::fs::read_to_string;
use regex::Regex;


#[derive(PartialEq, Debug, Clone)]
struct Number {
    value: u32,
    first_idx: usize,
    last_idx: usize,
    touches: bool,
    maybe_gear : bool,
    gear_idx: i32,
    gear_rowid: i32,
    used: bool
}

impl Number {
    fn analyze(&mut self, row: Option<&str>, rowid: i32) {
        let line = row.unwrap();
        if row != None {
            self.has_symbols_on_sides(line, rowid);
        }
    }

    fn has_symbols_on_sides(&mut self, line : &str, rowid: i32) -> bool {
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
                self.maybe_gear = result.contains("*");
                if self.maybe_gear {
                    self.gear_idx = (first_idx + c.find("*").unwrap()) as i32;
                    self.gear_rowid = rowid;
                }
                else {
                    self.gear_idx = -1;
                }
                return true
            }
        }
        return false
    }

    fn new(value: &String, indexes: &Vec<usize>) -> Number {
        Number{value: value.parse::<u32>().unwrap(), first_idx: *indexes.first().unwrap(), last_idx: *indexes.last().unwrap(), touches: false, maybe_gear:false, gear_idx:-1, gear_rowid: 0, used: false}
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

fn combine_gears(ll1 : &Vec<Number>, ll2: &Vec<Number>, self_intersection: bool) -> u64 {
    let mut l1 = ll1.clone();
    let mut l2 = ll2.clone();
    let mut sum : u64 = 0;
    for idx1 in (0..l1.len()) {
        let g1 = l1[idx1].clone();
        for idx2 in (0..l2.len()) {
            let g2 = l2[idx2].clone();
            if self_intersection {
                if idx1 == idx2 {
                    continue;
                }
            }
            if g1.gear_idx == g2.gear_idx && (g1.gear_rowid-g2.gear_rowid).abs() <=2 && !g1.used && !g2.used {
                sum += (g1.value * g2.value) as u64;
                if self_intersection {
                    l1[idx2].used = true;
                    l2[idx1].used = true;
                }
                l1[idx1].used = true;
                l2[idx2].used = true;
            }
        }
    }
    sum
}

fn join_gears(all_maybe_gears: Vec<Vec<Number>>) -> u64 {
    let mut sum : u64 = 0;
    for i in (0..all_maybe_gears.len()-2) {
        let current = &all_maybe_gears[i];
        let next = &all_maybe_gears[i+1];
        let next_next = &all_maybe_gears[i+2];
        sum += combine_gears(current, current, true);
        sum += combine_gears(current, next, false);
        sum += combine_gears(current, next_next, false);
    }
    let mut current = &all_maybe_gears[all_maybe_gears.len()-2];
    let mut next = &all_maybe_gears[all_maybe_gears.len()-1];
    sum += combine_gears(current, current, true);
    sum += combine_gears(current, next, false);
    sum += combine_gears(next, next, true);
    sum
}

fn main() {
    let filename = "../input.txt";
    let mut sum : u64 = 0;
    let mut last_line: Option<&str> = None;
    let mut last_numbers: Vec<Number> = Vec::new();
    let mut maybe_gears: Vec<Vec<Number>> = Vec::new();
    for (rowid,line) in read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines()
        .enumerate()
    // split the string into an iterator of string slices
    {
        let mut answer = String::new();
        // if last_numbers.len() > 0 {
            for n in last_numbers.iter_mut() {
                n.analyze(Some(line), rowid as i32);
            }
            let a:Vec<Number> = last_numbers.iter().filter(|x| x.maybe_gear==true).cloned().collect();
            maybe_gears.push(a);
        // }
        let mut current_numbers = parse_line(line);
        for n in current_numbers.iter_mut() {
            if last_line != None {
                n.analyze(last_line, (rowid-1) as i32);
            }
            n.analyze(Some(line), rowid as i32);
        }
        last_line = Some(line);
        last_numbers = current_numbers;
    }
    //
    if last_numbers.len() > 0 {
        let a:Vec<Number> = last_numbers.iter().filter(|x| x.maybe_gear==true).cloned().collect();
        maybe_gears.push(a);
    }
    println!("sum = {}", join_gears(maybe_gears));
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
