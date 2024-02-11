use itertools::Itertools;

// --- bandle on ---
use crate::DEBUG;
// --- bandle off ---

pub struct Island {
    n: usize,
    m: usize,
    e: f64,
    oilfields: Vec<Vec<(usize, usize)>>,
    pub field: Vec<Vec<State>>,
    excavate_history: Vec<((usize, usize), usize)>,
}

pub enum State {
    Decision(usize),
    Undecision(Vec<f64>),
}

impl Island {
    pub fn new(n: usize, m: usize, e: f64, oilfields: Vec<Vec<(usize, usize)>>) -> Self {
        let mut field: Vec<Vec<State>> = (0..n)
            .map(|_| {
                (0..n)
                    .map(|_| State::Undecision(vec![0.0; m + 1]))
                    .collect()
            })
            .collect();

        // がんばって推定する
        // すべて未検出の場合、初期状態の推測
        let layouts = (0..m)
            .map(|i| {
                let mut layout = vec![vec![0; n]; n];
                let (mx, my) = oilfields[i]
                    .iter()
                    .fold((0, 0), |(x, y), &(nx, ny)| (x.max(nx), y.max(ny)));
                for dx in 0..(n - mx) {
                    for dy in 0..(n - my) {
                        for &(x, y) in &oilfields[i] {
                            layout[dx + x][dy + y] += 1;
                        }
                    }
                }
                layout
            })
            .collect::<Vec<_>>();

        let layout_counts = (0..m)
            .map(|i| {
                let (mx, my) = oilfields[i]
                    .iter()
                    .fold((0, 0), |(x, y), &(nx, ny)| (x.max(nx), y.max(ny)));
                (n - mx) * (n - my)
            })
            .collect::<Vec<_>>();

        for x in 0..n {
            for y in 0..n {
                let mut dp = vec![0.0; m + 1];
                dp[0] = 1.0;
                for i in 0..m {
                    let mut pd = vec![0.0; m + 1];
                    // layouts[x][y][i] / layout_counts[i] の確率で置かれる
                    // 1.0 - ... の確率で置かれない
                    for j in 0..=i {
                        pd[j] += dp[j] * (1.0 - layouts[i][x][y] as f64 / layout_counts[i] as f64);
                        pd[j + 1] += dp[j] * (layouts[i][x][y] as f64 / layout_counts[i] as f64);
                    }
                    std::mem::swap(&mut dp, &mut pd);
                }
                field[x][y] = State::Undecision(dp);
            }
        }

        Self {
            n,
            m,
            e,
            oilfields,
            field,
            excavate_history: vec![],
        }
    }

    pub fn excavate(&mut self, x: usize, y: usize, v: usize) {
        self.field[x][y] = State::Decision(v);
        self.excavate_history.push(((x, y), v));

        // TODO: refactor
        // 厳密にやるのつらい
        // oilfieldsシャッフル -> ランダムに置く -> その結果を反映～ みたいな
        // モンテカルロっぽく更新
        // 嘘かも
    }

    pub fn disp(&self, (x, y): (usize, usize)) {
        if !DEBUG {
            return;
        }
        let val = match self.field[x][y] {
            State::Decision(v) => v as f64,
            State::Undecision(ref p) => p.iter().enumerate().map(|(i, &p)| i as f64 * p).sum(),
        };
        let val = ((val / 0.5).min(1.0) * 255.0) as usize;
        println!(
            "#c {} {} #{:02x}{:02x}{:02x}",
            x,
            y,
            255,
            255 - val,
            255 - val
        );
    }
}
