mod part_1;
mod part_2;
use std::fs::read_to_string;
pub use crate::part_1::part_1::parse_line;
use crate::part_2::part_2::{ScratchCard, process_cards, decode_line};

fn main() {
    let filename = "input.txt";
    let mut sum = 0;
    for line in read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
    {
        sum += parse_line(line);
    }
    println!("PART1: sum = {}", sum);
    //
    let part_2_test = "test.txt";
    let mut v : Vec<ScratchCard> = Vec::new();
    for line in read_to_string(part_2_test)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
    {
        v.push(decode_line(line));
    }
    println!("TAMANHO: {}", v.len());
    let all_scratch_cards = process_cards(&mut v);
    println!("ALL SCRATCH CARDS: {}", all_scratch_cards);
    //
    let mut v : Vec<ScratchCard> = Vec::new();
    for line in read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
    {
        v.push(decode_line(line));
    }
    println!("TAMANHO: {}", v.len());
    let all_scratch_cards = process_cards(&mut v);
    println!("ALL SCRATCH CARDS: {}", all_scratch_cards);

}
