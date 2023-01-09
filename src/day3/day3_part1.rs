use super::day3_data::DATA;

struct Content {
    compartment_1: String,
    compartment_2: String,
}

pub fn start() -> u32 {
    let mut sum_priorities: u32 = 0;

    for line in DATA.lines() {
        sum_priorities += get_priority(find_item(get_content(line)));
    }

    return sum_priorities;
}

fn get_content(content: &str) -> Content {
    let content = content.split_at(content.len() / 2);

    return Content {
        compartment_1: format!("{}", content.0),
        compartment_2: format!("{}", content.1),
    };
}

fn find_item(content: Content) -> char {
    for c1 in content.compartment_1.chars() {
        for c2 in content.compartment_2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    }

    panic!("Found no duplicates!\n");
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
