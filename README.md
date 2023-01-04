# Grecian Computer

I received the [`Grecian
Computer`](https://projectgeniusinc.com/grecian-computer/) wood brainteaser
puzzle as a gift. Here's my attempt to solve it with code!

![puzzle image](https://m.media-amazon.com/images/I/91-AcA-07gL.jpg)

## The puzzle

> Turn the dials until each of the 12 columns add up to 42.

The [`puzzle.json`](puzzle.json) file contains a JSON serialized version of the
puzzle. Each array element represents a rotatable dial within the puzzle. The
dials are ordered from top to bottom (smaller dials at the top, larger dials at
the bottom). This order is significant as some dials contain cutouts that show
the numbers of the dials below them.

Each dial contains at least one level. Each level is an array of exactly 12
elements. These elements may be numbers (integers) or `null` to represent a
cutout in the dial. The array can be seen as a circular array where the first
element is adjacent to the last element.

## Solving

Since this is some one-off code that I'm writing, I'm going to use this as an
opportunity to learn a new language: **Rust**!

My first attempt will just be a brute force, then I'll come back later and try
to optimize it (it seems like [dynamic
programming](https://en.wikipedia.org/wiki/Dynamic_programming) might apply to
this problem).

### Brute force

Given that there are 5 dials (actually, it's 4, since the bottom dial can be
seen as stationary) and 12 possible positions per dial, there are `12^4 =
20,736` possible dial configurations (hopefully my math's right!).

I'll start by writing a function that takes in the JSON serialized puzzle
([`puzzle.json`](puzzle.json)) and deserializes it into Rust data structures.
Since we already know the number of dials, columns, levels, etc. we can set
those up as constants. This will help with setting up types for deserialization.

```rust
const NUM_COLUMNS: usize = 12;
const NUM_DIALS: usize = 5;
const NUM_LEVELS: usize = 4; // A level is a row (circle) of numbers on a dial
```

Next up, using these constants, we can define the types for the puzzle.

```rust
type Puzzle = [Dial; NUM_DIALS]; // A puzzle is an array of 5 dials

#[derive(Clone, Copy, Serialize, Deserialize, Debug)] // I'm not too familiar with Rust's derive macros, but this lets us use serde to serialize and deserialize the puzzle (as well as copy the object).
struct Dial { // A dial is composed to 4 levels, with each level being optional.
  zero: Option<Level>,
  one: Option<Level>,
  two: Option<Level>,
  three: Option<Level>,
}

type Level = [Option<u32>; NUM_COLUMNS]; // A level is an array of 12 numbers (or nulls)
```

Now that we have the types defined, we can deserialize the puzzle using `serde`.

```rust
// We'll want to use at the top of the file
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader};


fn read_puzzle() -> Result<Puzzle, Box<dyn Error>> {
  let file = File::open("puzzle.json")?;
  let reader = BufReader::new(file);
  let puzzle: Puzzle = serde_json::from_reader(reader)?;
  verify(&puzzle);
  Ok(puzzle)
}
```

Next up, I wrote a `verify` function is just a sanity check to make sure that
the puzzle is somewhat valid. For example, it makes sure all levels have exactly
12 elements.

This isn't really necessary for solving the puzzle, so see
[`main.rs`](src/main.rs) for the full code.

#### `solve()`

Now that we have a valid puzzle, we can start solving it. Since we're using a
brute force approach, we'll just iterate through all possible dial permutations.
This means a nested loop for each dial, with each loop iterating through all
possible positions for that dial.

```rust
for _ in 0..NUM_COLUMNS {
  for _ in 0..NUM_COLUMNS {
    for _ in 0..NUM_COLUMNS {
      for _ in 0..NUM_COLUMNS {
        // ...
      }
    }
  }
}
```

For each iteration of the loop, we'll need to rotate the dials. This means
rotating all levels of a dial by one position. Thankfully Rust has a very handy
[`rotate_right`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.rotate_right)
method.

```rust
fn rotate(dial: &mut Dial) { // Helper function to rotate an entire dial
  rotate(&mut dial.zero);
  rotate(&mut dial.one);
  rotate(&mut dial.two);
  rotate(&mut dial.three);

  fn rotate(level: &mut Option<Level>) { // Rotates a single level
    if let Some(level) = level {
      level.rotate_right(1); // yo! that's a pretty handy method :)
    }
  }
}
```

Altogether, this is what the `solve()` function looks like:

```rust
/// Solve the puzzle with brute force
fn solve(puzzle: &mut Puzzle) -> Result<Puzzle, Box<dyn Error>> {
  // Loop over all Dials except the bottom one
  for _ in 0..NUM_COLUMNS {
    for _ in 0..NUM_COLUMNS {
      for _ in 0..NUM_COLUMNS {
        for _ in 0..NUM_COLUMNS {
          if validate(&puzzle) {
            // Return if the puzzle was solved
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

  // Error if the puzzle was not solved
  return Err("No solution found".into());

  // Helper function to rotate a dial
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
```

And that's pretty much it! There are a few other helper functions used to print
and serialize the solved puzzle.

**See [`main.rs`](src/main.rs) for the full code!**

### Optimizing

*coming ~~soon~~ whenever*

## The solution

```
Dial 0:   3  14   6   8  10  11   7  11  15   6   8   7
Dial 1:  14   7  15  13  21  14  15   9   9  12  11   4
Dial 2:  16  14   9  13   5   9  10  19   8  12  22  26
Dial 3:   9   7  12   8   6   8  10   3  10  12   1   5

Dial 0 is the smallest (at the top)
Dial 3 is the largest (at the bottom)
```

![Solved Grecian Computer Puzzle](https://user-images.githubusercontent.com/20099646/210469135-87fec34e-89a8-4f5b-9343-434960a238ab.png)
