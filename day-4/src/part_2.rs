pub mod part_2 {

    #[derive(PartialEq, Debug, Clone)]
    pub struct ScratchCard {
        pub id: u32,
        pub winners: u32,
        pub copies: u32,
    }

    impl ScratchCard {
        pub fn add_copies(&mut self, v : u32) {
            self.copies += v;
        }
    }

    pub fn decode_line(line: &str) -> ScratchCard {
        let id = line.split(":").next().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap();
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
        ScratchCard{ id, winners: count, copies: 1 }
    }

    pub fn process_cards(cards: & mut Vec<ScratchCard>) -> u32 {
        for idx in 0..cards.len() {
            let mut w = cards[idx].winners as usize;
            let copies = cards[idx].copies;
            while w != 0 {
                if (idx+w < cards.len()) {
                    cards[idx+w].add_copies(copies);
                }
                w -= 1;
            }
        }
        let mut copies = 0;
        for c in cards {
            println!("{:?}", c);
            copies += c.copies;
        }
        copies
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
    use crate::part_2::part_2::{decode_line, ScratchCard, process_cards};

    #[test]
    fn parse_line_test() {
        assert_eq!(decode_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), ScratchCard{id: 1, winners: 4, copies: 1});
        assert_eq!(decode_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), ScratchCard{id: 2, winners: 2, copies: 1});
        assert_eq!(decode_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), ScratchCard{id: 3, winners: 2, copies: 1});
        assert_eq!(decode_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), ScratchCard{id: 4, winners: 1, copies: 1});
        assert_eq!(decode_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), ScratchCard{id: 5, winners: 0, copies: 1});
        assert_eq!(decode_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), ScratchCard{id: 6, winners: 0, copies: 1});
    }

    #[test]
    fn process_cards_test() {
        let mut v : Vec<ScratchCard> = Vec::new();
        v.push(ScratchCard{id: 1, winners: 4, copies: 1});
        v.push(ScratchCard{id: 2, winners: 2, copies: 1});
        v.push(ScratchCard{id: 3, winners: 2, copies: 1});
        v.push(ScratchCard{id: 4, winners: 1, copies: 1});
        v.push(ScratchCard{id: 5, winners: 0, copies: 1});
        v.push(ScratchCard{id: 6, winners: 0, copies: 1});
        assert_eq!(process_cards(&mut v), 30);
    }
}