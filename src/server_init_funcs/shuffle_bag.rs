use rand::{seq::SliceRandom, thread_rng};
use tokio::sync::RwLock;

pub struct ShuffleBag {
    pub count: usize,
    pub shuffle_bag: Vec<u8>,
}

impl ShuffleBag {
    pub fn new() -> RwLock<ShuffleBag> {
        let mut bag: Vec<u8> = Vec::with_capacity(QUOTE_NUMBER);
        // If you want to initialize the vector with a sequence of numbers:
        for i in 0..QUOTE_NUMBER {
            bag.push(i as u8);
        }

        bag.shuffle(&mut thread_rng());
        RwLock::new(ShuffleBag {
            count: 0usize,
            shuffle_bag: bag,
        })
    }
    pub fn shuffle_bag(&mut self) {
        self.shuffle_bag.shuffle(&mut thread_rng());
    }
}

pub const QUOTE_NUMBER: usize = 5;

