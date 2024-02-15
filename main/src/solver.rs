// --- bandle on ---
use crate::{io::*, probability::Probability, Timer, DEBUG, TL};
use rand::seq::SliceRandom;
use std::io::BufRead;
// --- bandle off ---

enum Query {
    Excavate((usize, usize)),
    Predict(Vec<(usize, usize)>),
}

pub struct Solver<R: BufRead> {
    timer: Timer,
    n: usize,
    m: usize,
    e: f64,
    oilfields: Vec<Vec<(usize, usize)>>,
    io: IO<R>,
    probability: Probability,
}

impl<R: BufRead> Solver<R> {
    pub fn new(
        timer: Timer,
        io: IO<R>,
        n: usize,
        m: usize,
        e: f64,
        oilfields: Vec<Vec<(usize, usize)>>,
    ) -> Self {
        Self {
            timer,
            n,
            m,
            e,
            oilfields: oilfields.clone(),
            io,
            probability: Probability::new(n, m, e, oilfields),
        }
    }

    fn excavate(&mut self, (x, y): (usize, usize)) -> usize {
        let v = self.io.excavate((x, y));
        self.probability.update_excavate((x, y), v);
        v
    }

    fn submit(&mut self, ans: Vec<(usize, usize)>) {
        self.io.submit(ans);
        // submit falled
        self.probability.update_submit_failed();
    }

    fn next_query(&self, is_excavated: &[Vec<bool>]) -> Query {
        if let Some(pair) =
            self.next_excavate_good_pos(is_excavated, &self.probability.expected_value())
        {
            return Query::Excavate(pair);
        }
        Query::Excavate(self.next_random_pos(is_excavated))
    }

    fn next_excavate_good_pos(
        &self,
        is_excavated: &[Vec<bool>],
        p: &[Vec<f64>],
    ) -> Option<(usize, usize)> {
        if let Some((_, (x, y))) = (0..self.n)
            .flat_map(|i| (0..self.n).map(move |j| (i, j)))
            .map(|(x, y)| (p[x][y], (x, y)))
            .filter(|&(_, (x, y))| !is_excavated[x][y])
            // これなに？謎
            .filter(|&(p, _)| 0.01 < p && p < 0.99)
            .min_by(|a, b| {
                let a = (a.0 - 0.5).abs();
                let b = (b.0 - 0.5).abs();
                a.partial_cmp(&b).unwrap()
            })
        {
            Some((x, y))
        } else {
            None
        }
    }

    fn next_random_pos(&self, is_excavated: &[Vec<bool>]) -> (usize, usize) {
        let points = (0..self.n)
            .flat_map(|i| (0..self.n).map(move |j| (i, j)))
            .filter(|&(x, y)| !is_excavated[x][y])
            .collect::<Vec<_>>();
        *points.choose(&mut rand::thread_rng()).unwrap_or(&(0, 0))
    }

    pub fn solve(&mut self) {
        self.print_expected();
        let mut is_excavated = vec![vec![false; self.n]; self.n];

        while self.timer.get_time() < TL {
            let query = self.next_query(&is_excavated);
            match query {
                Query::Excavate((x, y)) => {
                    self.excavate((x, y));
                    if !is_excavated[x][y] {
                        self.print_expected();
                        is_excavated[x][y] = true;
                    }
                }
                Query::Predict(set) => {
                    self.io.predict(set);
                }
            }

            if let Some(ans) = self.probability.solved_check(&self.io) {
                self.submit(ans);
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
                self.io.debug(DEBUG, "honesty");
            }
        }

        let ans = (0..self.n)
            .flat_map(|i| (0..self.n).map(move |j| (i, j)))
            .filter(|&(i, j)| island[i][j])
            .collect::<Vec<_>>();

        self.io.submit(ans);
    }
}
