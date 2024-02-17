use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::{io::BufRead, process::exit};

pub struct IO<R: BufRead> {
    source: LineSource<R>,
    cost: f64,
    pub query_cnt: usize,
    excavate_history: Vec<((usize, usize), usize)>,
    submit_history: Vec<Vec<(usize, usize)>>,
}

impl<R: BufRead> IO<R> {
    pub fn new(mut source: LineSource<R>) -> Self {
        input! {
            from &mut source,
        }

        Self {
            source,
            cost: 0.0,
            query_cnt: 0,
            excavate_history: vec![],
            submit_history: vec![],
        }
    }

    pub fn init(&mut self) -> (usize, usize, f64, Vec<(usize, Vec<(usize, usize)>)>) {
        input! {
            from &mut self.source,
            n: usize,
            m: usize,
            e: f64,
        }

        let oilfields = {
            let mut oilfields = vec![];
            for _ in 0..m {
                input! {
                    from &mut self.source,
                    d: usize,
                    oilfield: [(usize, usize); d],
                }
                if let Some((idx, _)) = oilfields.iter_mut().find(|(_, v)| v == &oilfield) {
                    *idx += 1;
                } else {
                    oilfields.push((1, oilfield));
                }
            }
            oilfields
        };
        (n, m, e, oilfields)
    }

    pub fn excavate(&mut self, (x, y): (usize, usize)) -> usize {
        if let Some(&(_, v)) = self
            .excavate_history
            .iter()
            .find(|&&((hx, hy), _)| hx == x && hy == y)
        {
            return v;
        }
        self.cost += 1.0;
        self.query_cnt += 1;
        println!("q 1 {} {}", x, y);
        input! {
            from &mut self.source,
            res: usize,
        }
        self.excavate_history.push(((x, y), res));
        res
    }

    pub fn predict(&mut self, s: Vec<(usize, usize)>) -> f64 {
        let d = s.len();
        self.cost += 1.0 / (d as f64).sqrt();
        self.query_cnt += 1;
        let s = s.iter().map(|(x, y)| format!("{} {}", x, y)).join(" ");
        println!("q {} {}", d, s);
        input! {
            from &mut self.source,
            res: f64,
        }
        res
    }

    pub fn submit(&mut self, ans: Vec<(usize, usize)>) {
        if self.submit_history.contains(&ans) {
            return;
        }
        self.cost += 1.0;
        self.query_cnt += 1;
        self.submit_history.push(ans.clone());
        let d = ans.len();
        let ans = ans.iter().map(|(x, y)| format!("{} {}", x, y)).join(" ");
        println!("a {} {}", d, ans);
        input! {
            from &mut self.source,
            res: usize,
        }
        if res == 1 {
            println!("# submit success! cost: {}", self.cost);
            exit(0);
        }
        println!("# submit failed! {} {}", d, ans);
    }

    pub fn debug(&self, disp: bool, line: &str) {
        if !disp {
            return;
        }
        println!("# {}", line);
    }

    pub fn debug_color(&self, (x, y): (usize, usize), v: f64) {
        let mut lines = vec![];
        if v < 1.0 {
            let v = ((v * 255.0) as usize).min(255);
            lines.push(format!("#c {} {} #{:02x}ff{:02x}", x, y, 255 - v, 255 - v));
        } else {
            lines.push(format!("#c {} {} #ff00ff", x, y));
        }
        println!("{}", lines.iter().join("\n"));
    }
}
