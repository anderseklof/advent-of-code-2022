use super::day1_data::DATA;

pub fn start() -> u32 {
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
