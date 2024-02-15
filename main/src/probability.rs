// --- bandle on ---
use crate::IO;
// --- bandle off ---

use rand::seq::SliceRandom;

pub struct Probability {
    n: usize,
    m: usize,
    e: f64,
    oilfields: Vec<Vec<(usize, usize)>>,
    pub p: Vec<Vec<Vec<f64>>>,
    excavate_history: Vec<((usize, usize), usize)>,
}

impl Probability {
    pub fn new(n: usize, m: usize, e: f64, oilfields: Vec<Vec<(usize, usize)>>) -> Self {
        let p = (0..m)
            .map(|i| {
                let (mx, my) = oilfields[i]
                    .iter()
                    .fold((0, 0), |(mx, my), &(x, y)| (mx.max(x), my.max(y)));
                vec![vec![1.0 / ((n - mx) as f64 * (n - my) as f64); n - my]; n - mx]
            })
            .collect::<Vec<_>>();
        Self {
            n,
            m,
            e,
            oilfields: oilfields.clone(),
            p,
            excavate_history: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.p = (0..self.m)
            .map(|i| {
                let (mx, my) = self.oilfields[i]
                    .iter()
                    .fold((0, 0), |(mx, my), &(x, y)| (mx.max(x), my.max(y)));
                vec![
                    vec![1.0 / ((self.n - mx) as f64 * (self.n - my) as f64); self.n - my];
                    self.n - mx
                ]
            })
            .collect::<Vec<_>>();
        let mut excavate_history = self.excavate_history.clone();
        // excavate_history.shuffle(&mut rand::thread_rng());
        excavate_history.sort_by(|a, b| a.1.cmp(&b.1));
        for ((x, y), v) in excavate_history {
            self.update_excavate((x, y), v);
        }
    }

    fn invalid(&self) -> bool {
        self.p.iter().any(|p| {
            p.iter()
                .any(|p| p.iter().any(|&p| !(0.0..=1.0).contains(&p)))
        })
    }

    pub fn solved_check<R: std::io::BufRead>(&mut self, io: &IO<R>) -> Option<Vec<(usize, usize)>> {
        if self.invalid() {
            self.reset();
        }

        // reset 後がもうinvalidなケースがあるらしい
        if self.invalid() {
            return None;
        }

        let ac_per = self
            .p
            .iter()
            .map(|p| {
                p.iter()
                    .map(|p| *p.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|p| *p.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
            .fold(1.0, |acc, x| acc * x);

        io.debug(true, &format!("ac_per: {}", ac_per));

        if ac_per > (0.5_f64).powf(self.m as f64) {
            let mut r = vec![vec![0; self.n]; self.n];
            for (i, p) in self.p.iter().enumerate() {
                let (dx, (dy, _)) = p
                    .iter()
                    .map(|p| {
                        p.iter()
                            .enumerate()
                            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                            .unwrap()
                    })
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.1.partial_cmp(b.1).unwrap())
                    .unwrap();
                for &(x, y) in &self.oilfields[i] {
                    r[x + dx][y + dy] += 1;
                }
            }
            // excavate_history check
            if self
                .excavate_history
                .iter()
                .any(|((x, y), v)| r[*x][*y] != *v)
            {
                // self.update_submit_failed();
                self.reset();
                return None;
            }
            Some(
                (0..self.n)
                    .flat_map(|i| (0..self.n).map(move |j| (i, j)))
                    .filter(|&(i, j)| r[i][j] > 0)
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    }

    pub fn update_excavate(&mut self, (x, y): (usize, usize), v: usize) {
        if !self.excavate_history.contains(&((x, y), v)) {
            self.excavate_history.push(((x, y), v));
        }
        // 各ピースについて、 (x, y) が 1 になる確率を求めておく
        let pick_p = (0..self.m)
            .map(|i| {
                let mut res = 0.0;
                for dx in 0..(self.p[i].len()) {
                    for dy in 0..(self.p[i][dx].len()) {
                        // if self.oilfields[i].contains(&(x - dx, y - dy)) {
                        if self.oilfields[i]
                            .iter()
                            .any(|&(ox, oy)| x == ox + dx && y == oy + dy)
                        {
                            res += self.p[i][dx][dy];
                        }
                    }
                }
                res
            })
            .collect::<Vec<_>>();

        for i in 0..self.m {
            // dp[k] := ピース i 以外のピースを使って、合計 k になる確率
            let dp = {
                let mut dp = vec![0.0; v + 2];
                dp[0] = 1.0;
                for (j, &p) in pick_p.iter().enumerate() {
                    if i == j {
                        continue;
                    }
                    let mut pd = vec![0.0; v + 2];
                    for k in 0..(v + 1) {
                        pd[k + 1] += dp[k] * p;
                        pd[k] += dp[k] * (1.0 - p);
                    }
                    std::mem::swap(&mut dp, &mut pd);
                }
                dp
            };
            let (a, b) = {
                let a = if v == 0 { 0.0 } else { dp[v - 1] };
                let b = dp[v];
                let su = a + b;
                (a / su, b / su)
            };
            for dx in 0..(self.p[i].len()) {
                for dy in 0..(self.p[i][dx].len()) {
                    if self.oilfields[i]
                        .iter()
                        .any(|&(ox, oy)| x == ox + dx && y == oy + dy)
                    {
                        self.p[i][dx][dy] *= a;
                    } else {
                        self.p[i][dx][dy] *= b;
                    }
                }
            }
        }
        self.normalize();
    }

    fn normal_distribution(mu: f64, sig2: f64, x: f64) -> f64 {
        let a = 1. / (2. * std::f64::consts::PI * sig2).sqrt();
        let b = (x - mu).powi(2) / (2. * sig2);
        a * (-b).exp()
    }

    // 推測が面倒なので、殆ど 0 のやつについてpredictし、やる
    pub fn update_predict(&mut self, s: Vec<(usize, usize)>, v: f64) {
        let k = s.len() as f64;
        let per = (0..(s.len() + 1))
            .map(|vs| {
                let mu = (k - vs as f64) * self.e + vs as f64 * (1. - self.e);
                let sig2 = k * self.e * (1. - self.e);
                Probability::normal_distribution(mu, sig2, v)
            })
            .collect::<Vec<_>>();

        let num = per
            .iter()
            .enumerate()
            .map(|(i, p)| p * i as f64)
            .sum::<f64>();
        let sum = per.iter().sum::<f64>();

        // s の中の油田量総和の期待値...？
        // update
        let nk = num / sum;
        for (i, p) in self.p.iter_mut().enumerate() {
            for dx in 0..(p.len()) {
                for dy in 0..(p[dx].len()) {
                    let mut res = 0.0;
                    for (j, &(x, y)) in s.iter().enumerate() {
                        todo!();
                    }
                    todo!();
                }
            }
        }
        self.normalize();
    }

    pub fn update_submit_failed(&mut self) {
        for p in self.p.iter_mut() {
            let (dx, (dy, _)) = p
                .iter()
                .map(|p| {
                    p.iter()
                        .enumerate()
                        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                        .unwrap()
                })
                .enumerate()
                .max_by(|(_, a), (_, b)| a.1.partial_cmp(b.1).unwrap())
                .unwrap();
            p[dx][dy] *= 0.05; // 失敗したので確率を落とす 100%これなら正規化で元に戻るので問題なし
        }
        self.normalize();
    }

    // 正規化 各ピース i について、p[i] の合計が 1 になるようにする
    fn normalize(&mut self) {
        for i in 0..self.m {
            let sum = self.p[i].iter().map(|v| v.iter().sum::<f64>()).sum::<f64>();
            for dx in 0..(self.p[i].len()) {
                for dy in 0..(self.p[i][dx].len()) {
                    self.p[i][dx][dy] /= sum;
                }
            }
        }
    }

    // セル (i, j) の油田量の期待値
    pub fn expected_value(&self) -> Vec<Vec<f64>> {
        let mut ev = vec![vec![0.0; self.n]; self.n];
        for (i, oilfield) in self.oilfields.iter().enumerate() {
            for dx in 0..(self.p[i].len()) {
                for dy in 0..(self.p[i][dx].len()) {
                    let p = self.p[i][dx][dy];
                    for &(x, y) in oilfield {
                        ev[x + dx][y + dy] += p;
                    }
                }
            }
        }
        ev
    }
}

fn relative_eq_eps(a: f64, b: f64, epsilon: f64) -> bool {
    // 0.0の場合は特別扱いする
    if a == 0.0 || b == 0.0 {
        return (a - b).abs() < epsilon;
    }

    // 相対誤差を計算
    let relative_difference = (a - b).abs() / a.abs().max(b.abs());

    // 相対誤差がepsilon以下ならtrueを返す
    relative_difference < epsilon
}

fn relative_eq(a: f64, b: f64) -> bool {
    relative_eq_eps(a, b, 1.0e-6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probability() {
        let n = 4;
        let m = 2;
        let e = 0.01;

        // piece 1    piece 2
        //   .#.       ..#.
        //   ###       ####
        //   .#.

        let oilfields = vec![
            vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            vec![(1, 0), (1, 1), (1, 2), (1, 3), (0, 2)],
        ];
        let mut probability = Probability::new(n, m, e, oilfields);
        assert_eq!(
            probability.p,
            vec![
                vec![vec![1. / 4., 1. / 4.], vec![1. / 4., 1. / 4.]],
                vec![vec![1. / 3.], vec![1. / 3.], vec![1. / 3.]]
            ]
        );

        // answer
        // 0 0 1 0
        // 0 1 2 1
        // 2 2 2 1
        // 0 0 0 0

        probability.update_excavate((1, 1), 1);

        assert!(relative_eq(probability.p[0][0][0], 2. / 7.));
        assert!(relative_eq(probability.p[0][0][1], 2. / 7.));
        assert!(relative_eq(probability.p[0][1][0], 2. / 7.));
        assert!(relative_eq(probability.p[0][1][1], 1. / 7.));
        assert!(relative_eq(probability.p[1][0][0], 1. / 7.));
        assert!(relative_eq(probability.p[1][1][0], 3. / 7.));
        assert!(relative_eq(probability.p[1][2][0], 3. / 7.));
    }
}
