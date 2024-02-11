// --- bandle on ---
use crate::{io::*, probability::Probability};
use std::io::BufRead;

// --- bandle off ---

pub struct Solver<R: BufRead> {
    n: usize,
    m: usize,
    e: f64,
    oilfields: Vec<Vec<(usize, usize)>>,
    io: IO<R>,
    probability: Probability,
}

impl<R: BufRead> Solver<R> {
    pub fn new(io: IO<R>, n: usize, m: usize, e: f64, oilfields: Vec<Vec<(usize, usize)>>) -> Self {
        Self {
            n,
            m,
            e,
            oilfields: oilfields.clone(),
            io,
            probability: Probability::new(n, m, e, oilfields),
        }
    }

    fn excavate(&mut self, (x, y): (usize, usize)) -> usize {
        self.io.excavate((x, y))
    }

    pub fn solve(&mut self) {
        self.print_expected();
        self.honesty();
    }

    fn print_expected(&self) {
        let ev = self.probability.expected_value();
        for x in 0..self.n {
            for y in 0..self.n {
                let v = ((ev[x][y] * 255.0) as usize).min(255);
                println!("#c {} {} #{:02x}{:02x}{:02x}", x, y, 255, 255 - v, 255 - v);
            }
        }
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
