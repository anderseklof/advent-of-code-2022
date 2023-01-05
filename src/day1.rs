use crate::data::DATA;

pub fn day1_part1() -> u32 {
    let data: &str = DATA;

    let mut calories: u32 = 0;
    let mut max_calories: u32 = 0;

    for line in data.lines() {
        if line == "" {
            if calories > max_calories {
                max_calories = calories;
            }

            calories = 0;
        } else {
            calories += match line.parse::<u32>() {
                Ok(cal) => cal,
                Err(error) => {
                    print!("'{}'", line);
                    panic!("Error in line: {}", error);
                }
            };
        }
    }

    return max_calories;
}

pub fn day1_part2() -> u32 {
    let data: &str = DATA;

    let mut calories: u32 = 0;
    let mut total_calories: Vec<u32> = vec![];

    for line in data.lines() {
        if line == "" {
            total_calories.push(calories);

            calories = 0;
        } else {
            calories += match line.parse::<u32>() {
                Ok(cal) => cal,
                Err(error) => {
                    print!("'{}'", line);
                    panic!("Error in line: {}", error);
                }
            };
        }
    }

    total_calories.sort();
    total_calories.reverse();

    return total_calories[0] + total_calories[1] + total_calories[2];
}
