use super::day1_data::DATA;

pub fn start() -> u32 {
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
