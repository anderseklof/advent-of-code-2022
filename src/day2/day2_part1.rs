use super::day2_data::DATA;

#[derive(Debug, PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissor,
}

pub fn start() -> u32 {
    let data = DATA;
    let mut score: u32 = 0;

    for line in data.lines() {
        let (opponent_play, me_play) = get_round(line);
        score += <u8 as Into<u32>>::into(calculate_score(&opponent_play, &me_play));
    }

    return score;
}

fn get_round(line: &str) -> (Play, Play) {
    let mut opponent_play: Play = Play::Paper;
    let mut me_play: Play = Play::Paper;

    for (i, c) in line.chars().enumerate() {
        if i == 0 {
            opponent_play = match c {
                'A' => Play::Rock,
                'B' => Play::Paper,
                'C' => Play::Scissor,
                _ => panic!("Error: unsupported character {}\n", c),
            };
        } else if i == 2 {
            me_play = match c {
                'X' => Play::Rock,
                'Y' => Play::Paper,
                'Z' => Play::Scissor,
                _ => panic!("Error: unsupported character {}\n", c),
            };
        }
    }

    return (opponent_play, me_play);
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
