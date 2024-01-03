pub mod part_1 {

    pub fn parse_line(line: &str) -> u32 {
        let aux = line.split(":").last().unwrap();
        let mut winners : Vec<i32> = Vec::new();
        let mut mine : Vec<i32> = Vec::new();
        let mut count: u32 = 0;
        numbers_to_vec( aux.split("|").next().unwrap(), &mut winners);
        numbers_to_vec(aux.split("|").last().unwrap(), & mut mine);
        for x in mine.iter() {
            if winners.contains(x) {
                count += 1;
            }
        }
        let base: i32 = 2;
        if count == 0 {
            return 0
        }
        base.pow((count-1) as u32).try_into().unwrap()
    }

    fn numbers_to_vec(s: &str, v: & mut Vec<i32>) {
        for n in s.split(" ") {
            if n.trim().len() > 0 {
                v.push(n.parse::<i32>().unwrap());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_line;

    #[test]
    fn parse_line_test() {
        assert_eq!(parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 8);
        assert_eq!(parse_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), 2);
        assert_eq!(parse_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), 2);
        assert_eq!(parse_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), 1);
        assert_eq!(parse_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), 0);
        assert_eq!(parse_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 0);
    }
}