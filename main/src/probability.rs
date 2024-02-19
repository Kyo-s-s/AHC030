// --- bandle on ---
use crate::{random::Random, IO};
// --- bandle off ---

pub struct Probability {
    n: usize,
    m: usize,
    e: f64,
    oilfields: Vec<Vec<(usize, usize)>>,
    oilfields_count: Vec<(usize, usize)>, // idx, sum
    pub p: Vec<Vec<Vec<f64>>>,
    excavate_history: Vec<((usize, usize), usize)>,
    predict_history: Vec<(Vec<(usize, usize)>, f64)>,
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

        let oilfields_count = (0..m)
            .map(|i| {
                let mut idx = 1;
                let mut cnt = 1;
                for j in 0..m {
                    if oilfields[i] == oilfields[j] {
                        if j < i {
                            idx += 1;
                        }
                        cnt += 1;
                    }
                }
                (idx, cnt)
            })
            .collect::<Vec<_>>();

        Self {
            n,
            m,
            e,
            oilfields: oilfields.clone(),
            oilfields_count,
            p,
            excavate_history: vec![],
            predict_history: vec![],
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
        excavate_history.sort_by(|a, b| b.1.cmp(&a.1));
        for ((x, y), v) in excavate_history {
            self.update_excavate((x, y), v);
        }
        let mut predict_history = self.predict_history.clone();
        predict_history.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for (s, v) in predict_history {
            self.update_predict(&s, v);
        }
    }

    fn invalid(&self) -> bool {
        self.p.iter().any(|p| {
            p.iter()
                .any(|p| p.iter().any(|&p| !(0.0..=1.0).contains(&p)))
        })
    }

    pub fn all_excavate<R: std::io::BufRead>(&mut self, io: &IO<R>) -> Option<Vec<(usize, usize)>> {
        let a = self.excavate_history.iter().map(|(_, v)| v).sum::<usize>();
        let b = self.oilfields.iter().map(|v| v.len()).sum::<usize>();

        if a == b {
            Some(
                self.excavate_history
                    .iter()
                    .filter(|(_, v)| *v > 0)
                    .map(|((x, y), _)| (*x, *y))
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn solved_check<R: std::io::BufRead>(&mut self, io: &IO<R>) -> Option<Vec<(usize, usize)>> {
        if let Some(set) = self.all_excavate(io) {
            return Some(set);
        }

        if self.invalid() {
            self.reset();
        }

        // reset 後がもうinvalidなケースがあるらしい
        if self.invalid() {
            return None;
        }

        // max_by 使うんじゃなく、ピース i の位置 (dx, dy) を前処理で求めるべき
        // 同じピース同士はdx, dy の**filter**ではずせばよろしい
        // よしなに個数を掛ければよい？
        let positions = self
            .p
            .iter()
            .enumerate()
            .map(|(i, p)| {
                if self.oilfields_count[i].1 == 1 {
                    let mut ma = (f64::MIN, (0, 0));
                    for (dx, p) in p.iter().enumerate() {
                        for (dy, p) in p.iter().enumerate() {
                            if ma.0 < *p {
                                ma = (*p, (dx, dy));
                            }
                        }
                    }
                    ma
                } else {
                    let mut s = vec![];
                    for (dx, p) in p.iter().enumerate() {
                        for (dy, p) in p.iter().enumerate() {
                            let per = (p * self.oilfields_count[i].1 as f64).min(1.);
                            s.push((per, (dx, dy)));
                        }
                    }
                    s.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                    s[self.oilfields_count[i].0 - 1]
                }
            })
            .collect::<Vec<_>>();

        let ac_per = positions
            .iter()
            .map(|(per, _)| *per)
            .fold(1.0, |acc, x| acc * x);

        io.debug(true, &format!("ac_per: {}", ac_per));

        if ac_per > (0.7_f64).powf(self.m as f64) {
            let mut r = vec![vec![0; self.n]; self.n];
            for (i, (_, (dx, dy))) in positions.iter().enumerate() {
                for (x, y) in &self.oilfields[i] {
                    r[x + dx][y + dy] += 1;
                }
            }
            // excavate_history check
            let invalid_pos = self
                .excavate_history
                .iter()
                .filter(|((x, y), v)| r[*x][*y] != *v)
                .map(|((x, y), v)| ((*x, *y), *v))
                .collect::<Vec<_>>();

            // こういうことをするとダブりで困りそうではある
            if !invalid_pos.is_empty() {
                for &((x, y), v) in &invalid_pos {
                    io.debug(
                        true,
                        &format!(
                            "excavate_history check failed, x: {}, y: {}, v: {}, r: {}",
                            x, y, v, r[x][y]
                        ),
                    );
                    self.update_excavate((x, y), v)
                }
                return self.submit_expected(io);
            }
            Some(
                (0..self.n)
                    .flat_map(|i| (0..self.n).map(move |j| (i, j)))
                    .filter(|&(i, j)| r[i][j] > 0)
                    .collect::<Vec<_>>(),
            )
        } else {
            self.submit_expected(io)
        }
    }

    fn submit_expected<R: std::io::BufRead>(&mut self, io: &IO<R>) -> Option<Vec<(usize, usize)>> {
        if io.query_cnt > self.n * self.n && Random::get_f() < 0.2 {
            let p = self.expected_value();
            let set = (0..self.n)
                .flat_map(|i| (0..self.n).map(move |j| (i, j)))
                .filter(|&(i, j)| p[i][j] > 0.1)
                .collect::<Vec<_>>();

            if self.excavate_history.iter().all(|((x, y), v)| {
                if *v > 0 {
                    set.contains(&(*x, *y))
                } else {
                    !set.contains(&(*x, *y))
                }
            }) {
                return Some(set);
            }
        }
        None
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

    fn normal_distribution(mu: f64, sig2: f64) -> impl Fn(f64) -> f64 {
        move |x: f64| {
            let a = 1. / (2. * std::f64::consts::PI * sig2).sqrt();
            let b = (x - mu).powi(2) / (2. * sig2);
            a * (-b).exp()
        }
    }

    fn integrate(f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
        let n = 10;
        let h = (b - a) / n as f64;
        let s = (0..n)
            .map(|i| {
                let x = a + h * i as f64;
                f(x) + 4. * f(x + h / 2.) + f(x + h)
            })
            .sum::<f64>();
        (h / 6.) * s
    }

    pub fn update_predict(&mut self, set: &Vec<(usize, usize)>, v: f64) {
        let k = set.len() as f64;
        let per = (0..(3 * set.len()))
            .map(|vs| {
                let mu = (k - vs as f64) * self.e + vs as f64 * (1. - self.e);
                let sig2 = k * self.e * (1. - self.e);
                // Probability::normal_distribution(mu, sig2)(v)
                Probability::integrate(
                    Probability::normal_distribution(mu, sig2),
                    if v != 0. { v } else { -10.0 },
                    v + 1.,
                )
            })
            .collect::<Vec<_>>();

        let s = per.iter().sum::<f64>();

        for (i, p) in self.p.iter_mut().enumerate() {
            for (dx, p) in p.iter_mut().enumerate() {
                for (dy, p) in p.iter_mut().enumerate() {
                    // P(i, dx, dy) が正当である確率 -> (i, dx, dy) でset上に置かれる個数以上になる確率
                    let dub = set
                        .iter()
                        .filter(|&&(x, y)| {
                            self.oilfields[i]
                                .iter()
                                .any(|&(ox, oy)| x == ox + dx && y == oy + dy)
                        })
                        .count();
                    // 全部 s で割ってるので、やらないで後で正規化パートに任せる
                    let u = s - (0..dub).map(|dub| per[dub]).sum::<f64>();
                    *p *= u;
                }
            }
        }
        self.normalize();
    }

    // 正規化 各ピース i について、p[i] の合計が 1 になるようにする
    fn normalize(&mut self) {
        for i in 0..self.m {
            let sum = self.p[i].iter().map(|v| v.iter().sum::<f64>()).sum::<f64>();
            for p in self.p[i].iter_mut().flatten() {
                *p /= sum;
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
