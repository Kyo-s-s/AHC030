// --- bandle on ---
use crate::{io::*, probability::Probability, random::Random, Timer, DEBUG, TL};
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

    fn predict(&mut self, set: &Vec<(usize, usize)>) -> f64 {
        let v = self.io.predict(&set);
        self.probability.update_predict(&set, v);
        v
    }

    fn submit(&mut self, ans: Vec<(usize, usize)>) {
        self.io.submit(ans);
    }

    fn next_query(&self, is_excavated: &[Vec<bool>]) -> Query {
        let p = self.probability.expected_value();
        // 期待値 0.25 < p < 0.75 のセルから、 0.5に一番近い掘っていないものを選ぶ
        if let Some(pair) = self.next_excavate_good_pos(is_excavated, &p) {
            return Query::Excavate(pair);
        }

        // Predict
        if let Some(set) = self.next_predict(is_excavated, &p) {
            return Query::Predict(set);
        }

        // まだ掘っていないセルからランダムに選ぶ
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
            // ここの制約を厳しくして、Predictへ誘導？
            .filter(|&(p, _)| 0.05 < p && p < 0.95) // こっちのほうがスコアは良い(それはそう、出ちゃうとRandomなので)
            .min_by(|a, b| {
                let f = |x: f64| (x - 0.5).abs();
                let a = f(a.0);
                let b = f(b.0);
                a.partial_cmp(&b).unwrap()
            })
        {
            Some((x, y))
        } else {
            None
        }
    }

    fn next_predict(
        &self,
        is_excavated: &[Vec<bool>],
        p: &[Vec<f64>],
    ) -> Option<Vec<(usize, usize)>> {
        // このままだとTLまでずっとこれをやってしまう
        if Random::get_f() < 0.5 {
            return None;
        }
        let mut less = (0..self.n)
            .flat_map(|i| (0..self.n).map(move |j| (i, j)))
            .filter(|&(x, y)| !is_excavated[x][y] && p[x][y] < 0.25)
            .collect::<Vec<_>>();

        Random::shuffle(&mut less);
        // なんか近くのをいい感じに選ぶとよさげ
        let k = Random::get(10..21);
        if less.len() < k {
            None
        } else {
            Some(less.into_iter().take(k).collect())
        }
    }

    fn next_random_pos(&self, is_excavated: &[Vec<bool>]) -> (usize, usize) {
        let points = (0..self.n)
            .flat_map(|i| (0..self.n).map(move |j| (i, j)))
            .filter(|&(x, y)| !is_excavated[x][y])
            .collect::<Vec<_>>();
        if points.is_empty() {
            (0, 0)
        } else {
            *Random::get_item(&points)
        }
    }

    // TODO: self.e によってもかえる
    fn divide(&self) -> Vec<usize> {
        let oilfield_size_ave = self
            .oilfields
            .iter()
            .map(|oilfield| oilfield.len())
            .sum::<usize>() as f64
            / self.m as f64;

        if oilfield_size_ave < 6.0 {
            match self.n {
                10 => vec![2, 3, 2, 3],
                11 => vec![3, 3, 3, 2],
                12 => vec![3, 3, 3, 3],
                13 => vec![3, 3, 4, 3],
                14 => vec![3, 4, 4, 3],
                15 => vec![3, 3, 3, 3, 3],
                16 => vec![3, 3, 3, 4, 3],
                17 => vec![3, 4, 3, 4, 3],
                18 => vec![3, 3, 3, 3, 3, 3],
                19 => vec![3, 3, 3, 3, 4, 3],
                20 => vec![3, 4, 3, 3, 4, 3],
                _ => unimplemented!("n = {}", self.n),
            }
        } else if oilfield_size_ave < 20.0 {
            match self.n {
                10 => vec![3, 3, 4],
                11 => vec![3, 4, 4],
                12 => vec![4, 4, 4],
                13 => vec![4, 4, 5],
                14 => vec![4, 5, 5],
                15 => vec![5, 5, 5],
                16 => vec![4, 4, 4, 4],
                17 => vec![4, 4, 4, 5],
                18 => vec![4, 4, 5, 5],
                19 => vec![4, 5, 5, 5],
                20 => vec![5, 5, 5, 5],
                _ => unimplemented!("n = {}", self.n),
            }
        } else {
            match self.n {
                10 => vec![5, 5],
                11 => vec![5, 6],
                12 => vec![6, 6],
                13 => vec![6, 7],
                14 => vec![7, 7],
                15 => vec![5, 5, 5],
                16 => vec![5, 6, 5],
                17 => vec![6, 5, 6],
                18 => vec![6, 6, 6],
                19 => vec![6, 7, 6],
                20 => vec![7, 6, 7],
                _ => unimplemented!("n = {}", self.n),
            }
        }
    }

    fn preprocess_predict(&self) -> Vec<Vec<(usize, usize)>> {
        let div = self.divide();
        let mut ret = vec![];
        let mut x = 0;
        for &dx in &div {
            let mut y = 0;
            for &dy in &div {
                let mut set = vec![];
                for i in x..x + dx {
                    for j in y..y + dy {
                        set.push((i, j));
                    }
                }
                ret.push(set);
                y += dy;
            }
            x += dx;
        }
        ret
    }

    pub fn solve(&mut self) {
        self.print_expected();
        let mut is_excavated = vec![vec![false; self.n]; self.n];

        for set in self.preprocess_predict() {
            self.predict(&set);
            self.print_expected();
        }

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
                    self.predict(&set);
                }
            }
            if let Some(ans) = self.probability.solved_check(&self.io) {
                self.submit(ans);
            }
            if is_excavated.iter().flatten().all(|&b| b) {
                break;
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
