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

    fn next_excavate_pos(&self, is_excavated: &[Vec<bool>]) -> (usize, usize) {
        let p = self.probability.expected_value();
        let candidate = (0..self.n)
            .flat_map(|i| (0..self.n).map(move |j| (i, j)))
            .filter(|&(x, y)| !is_excavated[x][y])
            .filter(|&(x, y)| p[x][y] < 1.0)
            .map(|(x, y)| (p[x][y], (x, y)))
            .collect::<Vec<_>>();

        if let Some(&(_, (x, y))) = candidate
            .iter()
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        {
            (x, y)
        } else {
            let points = (0..self.n)
                .flat_map(|i| (0..self.n).map(move |j| (i, j)))
                .filter(|&(x, y)| !is_excavated[x][y])
                .collect::<Vec<_>>();
            *points.choose(&mut rand::thread_rng()).unwrap()
        }
    }

    pub fn solve(&mut self) {
        self.print_expected();
        let mut is_excavated = vec![vec![false; self.n]; self.n];

        for _ in 0..(self.n * self.n) {
            let (x, y) = self.next_excavate_pos(&is_excavated);
            let v = self.excavate((x, y));
            self.probability.update_excavate((x, y), v);
            is_excavated[x][y] = true;
            self.print_expected();
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
