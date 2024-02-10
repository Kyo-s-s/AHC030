pub struct Island {
    n: usize,
    m: usize,
    e: f64,
    oilfields: Vec<Vec<(usize, usize)>>,
    pub field: Vec<Vec<State>>,
}

const M: usize = 21;

pub enum State {
    Decision(usize),
    Undecision([f64; M]),
}

impl Island {
    pub fn new(n: usize, m: usize, e: f64, oilfields: Vec<Vec<(usize, usize)>>) -> Self {
        let field = (0..n)
            .map(|_| (0..n).map(|_| State::Undecision([0.0; M])).collect())
            .collect();

        Self {
            n,
            m,
            e,
            oilfields,
            field,
        }
    }

    pub fn excavate(&mut self, x: usize, y: usize, v: usize) {
        self.field[x][y] = State::Decision(v);
    }
}
