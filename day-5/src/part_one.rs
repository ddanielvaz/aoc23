pub mod solution_one {
    #[derive(Debug, PartialEq, Copy, Clone)]
    
    pub struct Seed {
        pub ini: u64,
        pub fin: u64
    }

    impl Seed {
        pub fn new(ini: u64, fin: u64) -> Seed {
            Seed{ini, fin}
        }
    }
    #[derive(Clone, Copy, Debug)]
    pub struct MapTransform {
        start_destination_range : u64,
        start_source_range : u64,
        range: u64
    }

    impl MapTransform {
        pub fn transform(&self, source_val: u64) -> u64 {
            self.start_destination_range + (source_val - self.start_source_range)
        }

        pub fn is_in_source_range(&self, source_val: u64) -> bool {
            (self.start_source_range..self.start_source_range+self.range).contains(&source_val)
        }
        pub fn new(start_destination_range: u64, start_source_range: u64, range: u64) -> MapTransform {
            MapTransform{start_destination_range, start_source_range, range}
        }
    }
    //
    pub fn parse_seeds(line: &str) -> Vec<u64> {
        let mut seeds : Vec<u64> = Vec::new();
        for n in line.split(": ").last().unwrap().split(" ") {
            if n.trim().len() > 0 {
                seeds.push(n.parse().unwrap());
            }
        }
        seeds
    }
    //
    pub fn parse_seeds_part_two(line: &str) -> Vec<Seed> {
        let mut seeds : Vec<Seed> = Vec::new();
        let pairs: Vec<_> = line.split(": ").last().unwrap().split(" ").collect();
        for i in (0..pairs.len()).step_by(2) {
            let ini : u64 = pairs[i].parse().unwrap();
            let fin : u64 = ini + pairs[i+1].parse::<u64>().unwrap();
            seeds.push(Seed::new(ini, fin));
        }
        seeds
    }
    //
    pub fn parse_map_transform(line: &str) -> MapTransform {
        let mut raw = line.split(" ");
        let destination = raw.next().unwrap().parse::<u64>().unwrap();
        let source = raw.next().unwrap().parse::<u64>().unwrap();
        let range = raw.next().unwrap().parse::<u64>().unwrap();
        MapTransform::new(destination, source, range)
    }
    #[cfg(test)]
    mod tests {
        use crate::part_one::solution_one::{parse_seeds, parse_map_transform, MapTransform};

        #[test]
        fn parse_seeds_test() {
            assert_eq!(parse_seeds("seeds: 79 14 55 13"), vec![79, 14, 55, 13]);
        }

        #[test]
        fn parse_map_transform_test() {
            assert_eq!(parse_map_transform("50 98 2"), MapTransform::new(50, 98, 2));
            assert_eq!(parse_map_transform("52 50 48"), MapTransform::new(52, 50, 48));
        }

        #[test]
        fn is_in_source_range_test() {
            assert_eq!(MapTransform::new(50, 98, 2).is_in_source_range(97), false);
            assert_eq!(MapTransform::new(50, 98, 2).is_in_source_range(99), true);
            assert_eq!(MapTransform::new(50, 98, 2).is_in_source_range(98), true);
            assert_eq!(MapTransform::new(50, 98, 2).is_in_source_range(100), false);
            //
            assert_eq!(MapTransform::new(52, 50, 48).is_in_source_range(49), false);
            for i in 50..98 {
                assert_eq!(MapTransform::new(52, 50, 48).is_in_source_range(i), true);
            }
            assert_eq!(MapTransform::new(52, 50, 48).is_in_source_range(98), false);
        }
    }
}