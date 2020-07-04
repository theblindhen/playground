#![allow(unused)]
mod bits2d;
mod model;

// What should be the rules for the game?
// - Fill as many squares as possible with full lookahead?
//   - That requires a SAT solver or metaheuristic.
// - Lookahead 1
//   - Machine learning?
//   - What space are we then searching? Not A*.
// - Require placing adjacent to existing fill?
//   - That's more like a game, but it's a strange rule.
//   - That surface can grow large.
// - Have a concept of a player position and moves?
//   - Even more like a game.
//   - Can accomodate some machine learning?

fn main() {
    let stair_step = model::shapes::stair_step();
    let ell = model::shapes::ell();
    let mut grid = model::Grid::new(10, 10);
    grid.must_place(&ell, 0, 0);
    grid.must_place(&stair_step, 2, 2);
    print!("{}", grid);
}
