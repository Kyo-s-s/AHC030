extern crate rand;

use rand::{seq::SliceRandom, Rng};

pub struct Random {}

impl Random {
    pub fn _get_f() -> f64 {
        rand::thread_rng().gen_range(0.0..1.0)
    }

    pub fn get(range: std::ops::Range<usize>) -> usize {
        rand::thread_rng().gen_range(range)
    }

    pub fn _get_2d(range: std::ops::Range<usize>) -> (usize, usize) {
        (Self::get(range.clone()), Self::get(range))
    }

    pub fn get_item<T>(items: &[T]) -> &T {
        &items[Self::get(0..items.len())]
    }

    pub fn shuffle<T>(items: &mut [T]) {
        items.shuffle(&mut rand::thread_rng());
    }
}
