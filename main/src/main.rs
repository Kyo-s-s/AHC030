// pub const DEBUG: bool = true;
pub const DEBUG: bool = false;

// --- bandle on ---
// path: io.rs
mod io;
use io::*;

// path: solver.rs
mod solver;
use solver::*;

// path: probability.rs
mod probability;

// --- bandle off ---

fn main() {
    let source = proconio::source::line::LineSource::new(std::io::stdin().lock());
    let mut io = IO::new(source);
    let (n, m, e, oilfields) = io.init();
    let mut solver = Solver::new(io, n, m, e, oilfields);
    solver.solve();
}
