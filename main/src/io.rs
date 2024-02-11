use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::{io::BufRead, process::exit};

pub struct IO<R: BufRead> {
    source: LineSource<R>,
    cost: f64,
    excavate_history: Vec<((usize, usize), usize)>,
}

impl<R: BufRead> IO<R> {
    pub fn new(mut source: LineSource<R>) -> Self {
        input! {
            from &mut source,
        }

        Self {
            source,
            cost: 0.0,
            excavate_history: vec![],
        }
    }

    pub fn init(&mut self) -> (usize, usize, f64, Vec<Vec<(usize, usize)>>) {
        input! {
            from &mut self.source,
            n: usize,
            m: usize,
            e: f64,
        }
        let oilfields = (0..m)
            .map(|_| {
                input! {
                    from &mut self.source,
                    d: usize,
                    oilfield: [(usize, usize); d],
                }
                oilfield
            })
            .collect::<Vec<_>>();
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
        let s = s.iter().map(|(x, y)| format!("{} {}", x, y)).join(" ");
        println!("q {} {}", d, s);
        input! {
            from &mut self.source,
            res: f64,
        }
        res
    }

    pub fn submit(&mut self, ans: Vec<(usize, usize)>) {
        self.cost += 1.0;
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
        } else {
            println!("# submit failed! {} {}", d, ans);
        }
    }

    pub fn debug_color(&self, (x, y): (usize, usize), v: f64) {
        // let v = ((v * 255.0) as usize).min(255);
        // println!("#c {} {} #{:02x}{:02x}{:02x}", x, y, 255, 255 - v, 255 - v);
        if v < 1.0 {
            let v = ((v * 255.0) as usize).min(255);
            println!("#c {} {} #ff{:02x}{:02x}", x, y, 255 - v, 255 - v);
        } else {
            println!("#c {} {} #ff00ff", x, y);
        }
    }
}
