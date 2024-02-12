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

    pub fn solved_check(&self) -> Option<Vec<(usize, usize)>> {
        let eps = 1e-6;
        if self.p.iter().all(|p| {
            p.iter()
                .any(|p| p.iter().any(|&p| relative_eq_eps(p, 1.0, eps)))
        }) {
            let mut r = vec![vec![false; self.n]; self.n];
            for (i, p) in self.p.iter().enumerate() {
                for (dx, p) in p.iter().enumerate() {
                    for (dy, &p) in p.iter().enumerate() {
                        if relative_eq_eps(p, 1.0, eps) {
                            for &(x, y) in &self.oilfields[i] {
                                r[x + dx][y + dy] = true;
                            }
                        }
                    }
                }
            }
            Some(
                (0..self.n)
                    .flat_map(|i| (0..self.n).map(move |j| (i, j)))
                    .filter(|&(i, j)| r[i][j])
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
            for dx in 0..(self.p[i].len()) {
                for dy in 0..(self.p[i][dx].len()) {
                    if self.oilfields[i]
                        .iter()
                        .any(|&(ox, oy)| x == ox + dx && y == oy + dy)
                    {
                        self.p[i][dx][dy] *= if v == 0 { 0.0 } else { dp[v - 1] };
                    } else {
                        self.p[i][dx][dy] *= dp[v];
                    }
                }
            }
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

    pub fn reupdate_excavate(&mut self) {
        let p = self.expected_value();
        let excavate_history = self.excavate_history.clone();
        let reupdate = excavate_history
            .iter()
            .filter(|&&((x, y), v)| (p[x][y] - v as f64).abs() > 0.5)
            .collect::<Vec<_>>();

        for &((x, y), v) in &reupdate {
            self.update_excavate((*x, *y), *v);
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
