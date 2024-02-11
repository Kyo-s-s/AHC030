// --- bandle on ---
use crate::io::*;
use std::io::BufRead;

// --- bandle off ---

pub struct Solver<R: BufRead> {
    n: usize,
    m: usize,
    e: f64,
    oilfields: Vec<Vec<(usize, usize)>>,
    io: IO<R>,
}

impl<R: BufRead> Solver<R> {
    pub fn new(io: IO<R>, n: usize, m: usize, e: f64, oilfields: Vec<Vec<(usize, usize)>>) -> Self {
        Self {
            n,
            m,
            e,
            oilfields,
            io,
        }
    }

    fn excavate(&mut self, (x, y): (usize, usize)) -> usize {
        self.io.excavate((x, y))
    }

    pub fn solve(&mut self) {
        self.honesty();
    }

    fn honesty(&mut self) {
        let mut island = vec![vec![false; self.n]; self.n];
        for x in 0..self.n {
            for y in 0..self.n {
                let v = self.excavate((x, y));
                if v > 0 {
                    island[x][y] = true;
                }
            }
        }

        let ans = (0..self.n)
            .flat_map(|i| (0..self.n).map(move |j| (i, j)))
            .filter(|&(i, j)| island[i][j])
            .collect::<Vec<_>>();

        self.io.submit(ans);
    }
}
