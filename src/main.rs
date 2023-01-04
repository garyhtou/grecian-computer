use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader, time::Instant};

const NUM_COLUMNS: usize = 12;
const NUM_DIALS: usize = 5;
const NUM_LEVELS: usize = 4;

type Puzzle = [Dial; NUM_DIALS];

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
struct Dial {
    zero: Option<Level>,
    one: Option<Level>,
    two: Option<Level>,
    three: Option<Level>,
}

type Level = [Option<u32>; NUM_COLUMNS];

fn main() {
    println!("Grecian Puzzle!");

    let mut puzzle = read_puzzle().unwrap();

    let start = Instant::now();
    let solved_puzzle = solve(&mut puzzle).unwrap();
    let elapsed = start.elapsed();

    println!("Solved in {}ms", elapsed.as_millis());
    let table = table(&solved_puzzle);
    table.into_iter().for_each(|row| {
        row.into_iter().for_each(|col| print!("{:3} ", col));
        println!();
    });

    write_puzzle(&solved_puzzle).unwrap();
}

/// Solve the puzzle with brute force
fn solve(puzzle: &mut Puzzle) -> Result<Puzzle, Box<dyn Error>> {
    // Loop over all Dials except the bottom one
    for _ in 0..NUM_COLUMNS {
        for _ in 0..NUM_COLUMNS {
            for _ in 0..NUM_COLUMNS {
                for _ in 0..NUM_COLUMNS {
                    if validate(&puzzle) {
                        return Ok(*puzzle);
                    }
                    // rotate level
                    rotate(&mut puzzle[0]);
                }
                rotate(&mut puzzle[1]);
            }
            rotate(&mut puzzle[2]);
        }
        rotate(&mut puzzle[3]);
    }

    return Err("No solution found".into());

    fn rotate(dial: &mut Dial) {
        rotate(&mut dial.zero);
        rotate(&mut dial.one);
        rotate(&mut dial.two);
        rotate(&mut dial.three);

        fn rotate(level: &mut Option<Level>) {
            if let Some(level) = level {
                level.rotate_right(1); // yo! that's a pretty handy method :)
            }
        }
    }
}

/// Check if the puzzle is solved
fn validate(puzzle: &Puzzle) -> bool {
    for col in 0..NUM_COLUMNS {
        let mut sum = 0;
        for lvl in 0..NUM_LEVELS {
            for dial in puzzle.iter() {
                let level = match lvl {
                    0 => &dial.zero,
                    1 => &dial.one,
                    2 => &dial.two,
                    3 => &dial.three,
                    _ => panic!("Invalid level"),
                };

                let level = match level {
                    Some(level) => level,
                    None => continue,
                };

                let value = level[col];
                if let Some(value) = value {
                    sum += value;
                    break;
                }
            }
        }
        if sum != 42 {
            return false;
        }
    }
    return true;
}

/// Read in the puzzle from a file
fn read_puzzle() -> Result<Puzzle, Box<dyn Error>> {
    let file = File::open("puzzle.json")?;
    let reader = BufReader::new(file);
    let puzzle: Puzzle = serde_json::from_reader(reader)?;
    verify(&puzzle);
    Ok(puzzle)
}

/// Just a sanity check to make sure the puzzle is valid
fn verify(puzzle: &Puzzle) {
    // assert 5 dials
    assert_eq!(puzzle.len(), NUM_DIALS);

    for dial in puzzle.iter() {
        // Put level in array and remove none
        let levels = [dial.zero, dial.one, dial.two, dial.three];
        let levels = levels.iter().filter(|x| x.is_some());

        // Assert each level has 12 elements
        for level in levels {
            assert_eq!(level.unwrap().len(), NUM_COLUMNS);
        }
    }
}

/// Serialize the solved puzzle to a file
fn write_puzzle(puzzle: &Puzzle) -> Result<(), Box<dyn Error>> {
    let file = File::create("solved_puzzle.json")?;
    serde_json::to_writer_pretty(file, puzzle)?;
    Ok(())
}

/// Convert a puzzle into a 2D array (table) representing the visible state of
/// the puzzle. If all columns of this table are equal to 42, the puzzle is
/// solved.
fn table(puzzle: &Puzzle) -> [[u32; NUM_COLUMNS]; NUM_LEVELS] {
    let mut table = [[0; NUM_COLUMNS]; NUM_LEVELS];
    for col in 0..NUM_COLUMNS {
        for lvl in 0..NUM_LEVELS {
            for dial in puzzle.iter() {
                let level = match lvl {
                    0 => &dial.zero,
                    1 => &dial.one,
                    2 => &dial.two,
                    3 => &dial.three,
                    _ => panic!("Invalid level"),
                };

                let level = match level {
                    Some(level) => level,
                    None => continue,
                };

                let value = level[col];
                if let Some(value) = value {
                    table[lvl][col] = value;
                    break;
                }
            }
        }
    }
    return table;
}
