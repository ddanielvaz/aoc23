mod part_one;

use part_one::part_one::Race;

fn main() {
    let races = vec![
        Race {
            time: 53,
            distance: 313,
        },
        Race {
            time: 89,
            distance: 1090,
        },
        Race {
            time: 76,
            distance: 1214,
        },
        Race {
            time: 98,
            distance: 1201,
        },
    ];
    let mut a = 1;
    for r in races {
        a *= r.possible_win();
    }
    println!("SOLUTION PART ONE: {}", a);
    let race_part_two = Race {
        time: 53897698,
        distance: 313109012141201,
    };
    println!("SOLUTION PART TWO: {}", race_part_two.possible_win());
}
