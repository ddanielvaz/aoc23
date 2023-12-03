use std::fs::read_to_string;

fn compose_number(line: &str) -> u32 {
    let mut a : String = Default::default();
    for c in line.chars() {
        if c.is_digit(10) {
            a.push(c);
            break
        }
    }
    for c in line.chars().rev() {
        if c.is_digit(10) {
            a.push(c);
            break
        }
    }
    a.parse().unwrap()
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
