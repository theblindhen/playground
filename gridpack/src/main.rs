#![allow(unused)]
mod bits2d;
mod model;

fn main() {
    let stair_step = model::shapes::stair_step();
    let ell = model::shapes::ell();
    let mut grid = model::Grid::new(10, 10);
    grid.must_place(&ell, 0, 0);
    grid.must_place(&stair_step, 2, 2);
    print!("{}", grid);
}
