use super::day6_data;

pub fn start() -> usize {
    let mut test_marker: Vec<char> = vec![];

    for (i, c) in day6_data::DATA.chars().enumerate() {
        test_marker.push(c);

        if test_marker.len() == 4 {
            if is_marker(&mut test_marker) {
                return i + 1;
            } else {
                test_marker.remove(0);
            }
        }
    }

    panic!("Shouldn't happen!\n");
}

fn is_marker(test_marker: &Vec<char>) -> bool {
    let mut _test_marker: Vec<char> = test_marker.clone();
    _test_marker.sort();
    _test_marker.dedup();

    if _test_marker.len() == 4 {
        return true;
    } else {
        return false;
    }
}
