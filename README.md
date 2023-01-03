# Grecian Computer

I received the [`Grecian
Computer`](https://projectgeniusinc.com/grecian-computer/) wood brainteaser
puzzle as a gift. Here's my attempt to solve it with code!

![puzzle image](https://m.media-amazon.com/images/I/91-AcA-07gL.jpg)

## The puzzle

> Turn the dials until each of the 12 columns add up to 42.

The `puzzle.json` file contains a JSON serialized version of the puzzle. Each
array element represents a rotatable dial within the puzzle. The dials are
ordered from top to bottom (smaller dials at the top, larger dials at the
bottom). This order is significant as some dials contain cutouts that show the
numbers of the dials below them.

Each dial contains at least one level. Each level is an array of exactly 12
elements. These elements may be numbers (integers) or `null` to represent a
cutout in the dial. The array can be seen as a circular array where the first
element is adjacent to the last element.

## Solving

I'm going to try using Rust! My first attempt will just be a brute force, then
I'll come back later and try to optimize it. It seems like dynamic programming
might apply to this problem.

## The solution

TBD

