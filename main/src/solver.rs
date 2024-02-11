// --- bandle on ---
use crate::{io::*, probability::Probability, DEBUG};
use std::io::BufRead;

// --- bandle off ---

use rand::seq::SliceRandom;

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

        let points = {
            let mut points = (0..self.n)
                .flat_map(|i| (0..self.n).map(move |j| (i, j)))
                .collect::<Vec<_>>();
            points.shuffle(&mut rand::thread_rng());
            points
        };

        for &(x, y) in points.iter() {
            let v = self.excavate((x, y));
            self.probability.update_excavate((x, y), v);
            // self.print_expected();
            if let Some(ans) = self.probability.solved_check() {
                self.io.submit(ans);
            }
        }

        self.honesty();
    }

    fn print_expected(&self) {
        if !DEBUG {
            return;
        }
        let ev = self.probability.expected_value();
        for (x, ev) in ev.iter().enumerate() {
            for (y, ev) in ev.iter().enumerate() {
                self.io.debug_color((x, y), *ev);
            }
        }
    }

    fn honesty(&mut self) {
        let mut island = vec![vec![false; self.n]; self.n];
        for (x, island) in island.iter_mut().enumerate() {
            for (y, island) in island.iter_mut().enumerate() {
                *island = self.excavate((x, y)) > 0;
            }
        }

        let ans = (0..self.n)
            .flat_map(|i| (0..self.n).map(move |j| (i, j)))
            .filter(|&(i, j)| island[i][j])
            .collect::<Vec<_>>();

        self.io.submit(ans);
    }
}
