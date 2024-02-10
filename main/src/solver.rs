// --- bandle on ---
use crate::io::*;
use crate::island::*;
use std::io::BufRead;

// --- bandle off ---

pub struct Solver<R: BufRead> {
    n: usize,
    m: usize,
    e: f64,
    io: IO<R>,
    island: Island,
}

impl<R: BufRead> Solver<R> {
    pub fn new(io: IO<R>, n: usize, m: usize, e: f64, oilfields: Vec<Vec<(usize, usize)>>) -> Self {
        let island = Island::new(n, m, e, oilfields);
        Self {
            n,
            m,
            e,
            io,
            island,
        }
    }

    fn excavate(&mut self, (x, y): (usize, usize)) -> usize {
        if let State::Decision(v) = self.island.field[x][y] {
            return v;
        }
        let v = self.io.excavate((x, y));
        self.island.excavate(x, y, v);
        v
    }

    pub fn solve(&mut self) {
        for x in 0..self.n {
            for y in 0..self.n {
                self.excavate((x, y));
            }
        }

        let ans = (0..self.n)
            .flat_map(|i| (0..self.n).map(move |j| (i, j)))
            .filter(|&(i, j)| {
                if let State::Decision(v) = self.island.field[i][j] {
                    v > 0
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        self.io.submit(ans);
    }
}
