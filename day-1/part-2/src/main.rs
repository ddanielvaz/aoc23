use std::collections::HashMap;
use std::fs::read_to_string;

fn first_digit(line: &str) -> Option<(u32, u32)> {
    let mut idx : Option<(u32,u32)> = None; // (index, value)
    for (index,c) in line.chars().enumerate() {
        if c.is_digit(10) {
            idx = Some( (index as u32, c.to_digit(10).unwrap()) );
            break
        }
    }
    idx
}

fn last_digit(line: &str) -> Option<(u32, u32)> {
    let mut idx : Option<(u32, u32) > = None; // (index, value)
    let len = line.len();
    for (index,c) in line.chars().rev().enumerate() {
        if c.is_digit(10) {
            idx = Some( ((len-index-1) as u32, c.to_digit(10).unwrap()));
            break
        }
    }
    idx
}

fn first_word(line: &str) -> Option<(u32, u32)> {
    let mut choose : Option<(u32, u32)> = None; // (index, value)
    let mut idx = line.len();
    let numbers = HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine")
    ]);
    for a in numbers {
        match line.find(a.1) {
            None => continue,
            Some(i) => if i < idx { idx = i; choose = Some( (idx as u32, a.0) ); }
        }
    }
    choose
}

fn last_word(line: &str) -> Option<(u32, u32)> {
    let mut choose : Option<(u32, u32)> = None; // (index, value)
    let mut idx = 0;
    let numbers = HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine")
    ]);
    for a in numbers {
        match line.rfind(a.1) {
            None => continue,
            Some(i) => if i >= idx { idx = i; choose = Some( (idx as u32, a.0) ); }
        }
    }
    choose
}

fn first_number(line: &str) -> u32 {
    let d = first_digit(line);
    let w = first_word(line);
    if w == None && d != None {
        return d.unwrap().1
    }
    else if d == None && w != None {
        return w.unwrap().1
    }
    else if d == None && w == None {
        return 0
    }
    else if w.unwrap().0 <= d.unwrap().0 {
        return w.unwrap().1
    }
    d.unwrap().1
}

fn last_number(line: &str) -> u32 {
    let d = last_digit(line);
    let w = last_word(line);
    if w == None && d != None {
        return d.unwrap().1
    }
    else if d == None && w != None {
        return w.unwrap().1
    }
    else if d == None && w == None {
        return 0
    }
    else if w.unwrap().0 >= d.unwrap().0 {
        return w.unwrap().1
    }
    d.unwrap().1
}

fn compose_number(line: &str) -> u32 {
    10*first_number(line) + last_number(line)
}

fn main() {
    let filename = "../input.txt";
    let mut sum = 0;

    for line in read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
    {
        sum += compose_number(line);
    }
    println!("sum = {}", sum);
}


#[cfg(test)]
mod tests {
    use crate::compose_number;
    use crate::first_digit;
    use crate::last_digit;
    use crate::first_word;
    use crate::last_word;

    #[test]
    fn first_word_test() {
        assert_eq!(first_word("two1nine"), Some( (0, 2) ));
        assert_eq!(first_word("eightwothree"), Some( (0, 8) ));
        assert_eq!(first_word("abcone2threexyz"), Some( (3, 1) ));
        assert_eq!(first_word("xtwone3four"), Some( (1,2) ));
        assert_eq!(first_word("131"), None);
        assert_eq!(first_word("4nineeightseven2"), Some( (1, 9) ));
        assert_eq!(first_word("zoneight234"), Some( (1, 1) ));
        assert_eq!(first_word("7pqrstsixteen"), Some( (6, 6) ));
    }

    #[test]
    fn last_word_test() {
        assert_eq!(last_word("two1nine"), Some( (4, 9) ));
        assert_eq!(last_word("eightwothree"), Some( (7, 3) ));
        assert_eq!(last_word("abcone2threexyz"), Some( (7, 3) ));
        assert_eq!(last_word("xtwone3four"), Some( (7, 4) ));
        assert_eq!(last_word("131"), None);
        assert_eq!(last_word("4nineeightseven2"), Some( (10, 7) ));
        assert_eq!(last_word("zoneight234"), Some( (3, 8) ));
        assert_eq!(last_word("7pqrstsixteen"), Some( (6, 6) ));
    }

    #[test]
    fn first_digit_test() {
        assert_eq!(first_digit("two1nine"), Some( (3, 1) ));
        assert_eq!(first_digit("eightwothree"), None);
        assert_eq!(first_digit("abcone2threexyz"), Some((6, 2)));
        assert_eq!(first_digit("xtwone3four"), Some((6, 3)));
        assert_eq!(first_digit("4nineeightseven2"), Some((0, 4)));
        assert_eq!(first_digit("zoneight234"), Some((8, 2)));
        assert_eq!(first_digit("7pqrstsixteen"), Some((0, 7)));
    }

    #[test]
    fn last_digit_test() {
        assert_eq!(last_digit("two12nine"), Some( (4, 2)));
        assert_eq!(last_digit("eightwothree"), None);
        assert_eq!(last_digit("abcone2three3xyz"), Some( (12, 3)));
        assert_eq!(last_digit("xtwone3four"), Some( (6, 3)));
        assert_eq!(last_digit("4nineeightseven2"), Some( (15, 2)));
        assert_eq!(last_digit("zoneight234"), Some( (10, 4)));
        assert_eq!(last_digit("7pqrstsixteen"), Some( (0, 7)));
    }

    #[test]
    fn compose_number_test() {
        assert_eq!(compose_number("two1nine"), 29);
        assert_eq!(compose_number("eightwothree"), 83);
        assert_eq!(compose_number("abcone2threexyz"), 13);
        assert_eq!(compose_number("xtwone3four"), 24);
        assert_eq!(compose_number("4nineeightseven2"), 42);
        assert_eq!(compose_number("zoneight234"), 14);
        assert_eq!(compose_number("7pqrstsixteen"), 76);
        assert_eq!(compose_number("131"), 11);
    }
}
