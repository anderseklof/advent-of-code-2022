use super::day2_data::DATA;

#[derive(Debug, PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

pub fn start() -> u32 {
    let data = DATA;
    let mut score: u32 = 0;

    for line in data.lines() {
        let (opponent_play, outcome) = get_round(line);
        let me_play: Play = what_to_play(&opponent_play, &outcome);
        score += <u8 as Into<u32>>::into(calculate_score(&opponent_play, &me_play));

        // print!("Score: {}\n", calculate_score(&opponent_play, &me_play));
        // print!("Score acc: {}\n", score);
    }

    return score;
}

fn get_round(line: &str) -> (Play, Outcome) {
    let mut opponent_play: Play = Play::Paper;
    let mut outcome: Outcome = Outcome::Win;

    for (i, c) in line.chars().enumerate() {
        if i == 0 {
            opponent_play = match c {
                'A' => Play::Rock,
                'B' => Play::Paper,
                'C' => Play::Scissor,
                _ => panic!("Error: unsupported character {}\n", c),
            };
        } else if i == 2 {
            outcome = match c {
                'X' => Outcome::Lose,
                'Y' => Outcome::Draw,
                'Z' => Outcome::Win,
                _ => panic!("Error: unsupported character {}\n", c),
            };
        }
    }

    return (opponent_play, outcome);
}

fn calculate_score(opponent_play: &Play, me_play: &Play) -> u8 {
    const WIN: u8 = 6;
    const DRAW: u8 = 3;

    // Draw
    if opponent_play == me_play {
        return get_play_score(&me_play) + DRAW;
    }
    // Win Rock - Paper
    else if opponent_play == &Play::Rock && me_play == &Play::Paper {
        return get_play_score(&me_play) + WIN;
    }
    // Win Paper - Scissor
    else if opponent_play == &Play::Paper && me_play == &Play::Scissor {
        return get_play_score(&me_play) + WIN;
    }
    // Win Scissor - Rock
    else if opponent_play == &Play::Scissor && me_play == &Play::Rock {
        return get_play_score(&me_play) + WIN;
    }
    // Lose
    else {
        return get_play_score(&me_play);
    }
}

fn get_play_score(play: &Play) -> u8 {
    match play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissor => 3,
    }
}

fn what_to_play(opponent_play: &Play, outcome: &Outcome) -> Play {
    if outcome == &Outcome::Win {
        return match opponent_play {
            &Play::Rock => Play::Paper,
            &Play::Paper => Play::Scissor,
            &Play::Scissor => Play::Rock,
        };
    } else if outcome == &Outcome::Lose {
        return match opponent_play {
            &Play::Rock => Play::Scissor,
            &Play::Paper => Play::Rock,
            &Play::Scissor => Play::Paper,
        };
    } else {
        return match opponent_play {
            &Play::Rock => Play::Rock,
            &Play::Paper => Play::Paper,
            &Play::Scissor => Play::Scissor,
        };
    }
}
