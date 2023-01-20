use std::vec;

pub fn start() -> String {
    let mut line_stacks: Vec<&str> = vec![];
    let mut line_instructions: Vec<&str> = vec![];

    // Read all data
    for line in super::day5_data::DATA.lines() {
        // Stacks
        if line.contains("[") {
            line_stacks.push(line);
        }
        // Instructions
        else if line.contains("move") {
            line_instructions.push(line);
        }
        // Row with 1   2   3 ...
        else if line.contains("   ") {
            // Do nothing
        }
        // Empty row
        else {
            // Do nothing
        }
    }

    let mut stacks = get_stacks(line_stacks);

    let instructions = get_instuctions(line_instructions);

    move_crates(&mut stacks, instructions);

    let top_crates: Stack = get_top_crates(stacks);

    return top_crates.iter().collect();
}

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

fn get_stacks(mut line_stacks: Vec<&str>) -> Stacks {
    let mut stacks: Stacks = vec![];
    let mut first_row: bool = true;

    line_stacks.reverse();

    for line in line_stacks.iter() {
        let mut i: usize = 0;
        let mut x: usize = 1;

        while x < line.len() - 1 {
            let c: char = line.get(x..x + 1).unwrap().chars().nth(0).unwrap();

            if c == ' ' {
                // No crates in stack, do nothing
            } else {
                if first_row {
                    stacks.push(vec![]);
                }
                stacks[i].push(c);
            }
            i += 1; // Next stack position in vector
            x += 4; // Get next stack position in line
        }

        first_row = false;
    }

    return stacks;
}

#[derive(Debug)]
struct Instruction {
    no: usize,
    from: usize,
    to: usize,
}
type Instructions = Vec<Instruction>;

fn get_instuctions(line_instructions: Vec<&str>) -> Instructions {
    let mut instructions: Instructions = vec![];

    for line in line_instructions.iter() {
        let parts: Vec<&str> = line.split(" ").collect();

        instructions.push(Instruction {
            no: parts[1].parse::<usize>().unwrap(),
            from: parts[3].parse::<usize>().unwrap(),
            to: parts[5].parse::<usize>().unwrap(),
        });
    }

    return instructions;
}

fn move_crates(stacks: &mut Stacks, instructions: Instructions) {
    for instruction in instructions {
        let index = stacks[instruction.from - 1].len() - instruction.no;

        // Take the number of crates to move in order.
        let mut chars = stacks[instruction.from - 1].split_off(index);

        stacks[instruction.to - 1].append(&mut chars);
    }
}

fn get_top_crates(stacks: Stacks) -> Stack {
    let mut top_crates: Stack = vec![];

    for stack in stacks.iter() {
        top_crates.push(stack[stack.len() - 1]);
    }

    return top_crates;
}
