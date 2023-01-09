use super::day3_data;

pub fn start() -> u32 {
    let mut group: Vec<&str> = Vec::with_capacity(3);
    let mut sum_priority: u32 = 0;

    for (index, line) in day3_data::DATA.lines().enumerate() {
        group.push(line);

        if index != 0 && (index + 1) % 3 == 0 {
            sum_priority += get_priority(get_badge(&group));
            group.clear();
        }
    }

    return sum_priority;
}

fn get_badge(group: &Vec<&str>) -> char {
    let mut found: bool = false;
    let mut c: char = '_';
    let mut i: usize = 0;

    while !found {
        c = get_item_in_rucksack(group[0], i);

        if is_item_in_rucksack(group[1], c) && is_item_in_rucksack(group[2], c) {
            found = true;
        } else {
            i += 1;
        }
    }

    return c;
}

fn get_item_in_rucksack(rucksack: &str, index: usize) -> char {
    return match rucksack.chars().nth(index) {
        Some(c) => c,
        None => panic!("Index out of range!\n"),
    };
}

fn is_item_in_rucksack(rucksack: &str, item: char) -> bool {
    return rucksack.contains(item);
}

fn get_priority(item: char) -> u32 {
    let ascii_code = item as u32;

    // Lowercase a-z / 97-122
    if ascii_code > 96 {
        return ascii_code - 96;
    }
    // Uppercase A-Z / 65-90
    else {
        return ascii_code - 38;
    }
}
