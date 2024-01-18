pub mod part_one {
    pub struct Race {
        pub time: u64,
        pub distance: u64,
    }
    impl Race {
        pub fn possible_win(&self) -> u32 {
            let delta = self.time * self.time - 4 * self.distance;
            let sqrt_delta = f64::sqrt(delta as f64);
            let t_min = ((self.time as f64 - sqrt_delta) / 2.0 + 0.1).ceil();
            let t_max = ((self.time as f64 + sqrt_delta) / 2.0 - 0.1).floor();
            (t_max - t_min + 1.0) as u32
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::part_one::part_one::Race;
        #[test]
        pub fn possible_win_test() {
            let r1 = Race {
                time: 7,
                distance: 9,
            };
            let r2 = Race {
                time: 15,
                distance: 40,
            };
            let r3 = Race {
                time: 30,
                distance: 200,
            };
            let r4 = Race {
                time: 71530,
                distance: 940200,
            };
            assert_eq!(r1.possible_win(), 4);
            assert_eq!(r2.possible_win(), 8);
            assert_eq!(r3.possible_win(), 9);
            assert_eq!(r4.possible_win(), 71503);
        }
    }
}
