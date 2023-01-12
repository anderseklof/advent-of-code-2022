pub fn start() -> u32 {
    let mut sum_overlapping = 0;

    for line in super::day4_data::DATA.lines() {
        if is_pairs_overlapping(get_pairs(line)) {
            sum_overlapping += 1;
        }
    }

    return sum_overlapping;
}

#[derive(Debug, PartialEq, Eq)]
struct Pairs {
    one: (u8, u8),
    two: (u8, u8),
}

fn get_pairs(line: &str) -> Pairs {
    match line.split_once(',') {
        Some(pairs_temp) => {
            let one = pairs_temp.0.split_once("-").unwrap();
            let two = pairs_temp.1.split_once("-").unwrap();

            return Pairs {
                one: (one.0.parse::<u8>().unwrap(), one.1.parse::<u8>().unwrap()),
                two: (two.0.parse::<u8>().unwrap(), two.1.parse::<u8>().unwrap()),
            };
        }
        None => panic!("Should not happen!\n"),
    };
}

fn is_pairs_overlapping(pairs: Pairs) -> bool {
    if pairs.one.0 >= pairs.two.0 && pairs.one.1 <= pairs.two.1 {
        return true;
    } else if pairs.two.0 >= pairs.one.0 && pairs.two.1 <= pairs.one.1 {
        return true;
    } else {
        return false;
    }
}
