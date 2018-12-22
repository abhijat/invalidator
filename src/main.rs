extern crate fasthash;
#[macro_use]
extern crate log;
extern crate rand;
extern crate simplelog;

use bit_vec::BitVec;
use fasthash::metro;
use rand::{
    distributions::Alphanumeric,
    Rng,
    thread_rng,
};
use simplelog::{
    Config,
    LevelFilter,
    TermLogger,
};

fn hash_pairs(payload: &str) -> (u128, u128) {
    let first_hash = metro::hash64(&payload);
    let second_hash = metro::hash64(&format!("{:x}", first_hash));
    (first_hash as u128, second_hash as u128)
}

struct BloomFilter {
    n_bits: u128,
    n_hashes: u128,
    backing_store: BitVec,
}

impl BloomFilter {
    fn new() -> Self {
        let n_bits = u128::pow(2, 32);
        BloomFilter {
            n_bits,
            n_hashes: 5,
            backing_store: BitVec::from_elem(n_bits as usize, false),
        }
    }

    fn add(&mut self, item: &str) {
        let (first, second) = hash_pairs(item);
        for i in 0..self.n_hashes {
            let index = first + (i * second);
            let index = index & (self.n_bits - 1);
            self.backing_store.set(index as usize, true);
        }
    }

    fn get(&self, item: &str) -> bool {
        let (first, second) = hash_pairs(item);
        for i in 0..self.n_hashes {
            let index = first + (i * second);
            let index = index & (self.n_bits - 1);
            if !self.backing_store.get(index as usize).unwrap() {
                return false;
            }
        }
        true
    }
}

pub fn exercise_filter() {

    let mut filter = BloomFilter::new();
    info!("created filter");

    let data_size = 1000 * 1000 * 10;
    let mut words: Vec<String> = Vec::with_capacity(data_size);

    for _ in 0..data_size {
        let word: String = thread_rng().sample_iter(&Alphanumeric)
            .take(24)
            .collect();
        filter.add(&word);
        words.push(word);
    }

    info!("initialized data");
    for word in words.iter() {
        if !filter.get(word) {
            error!("uh oh -  a false negative");
            panic!()
        }
    }
    info!("false negative tests complete");
    let mut false_positives = 0;
    for word in words.iter() {
        let r = word.to_ascii_uppercase();
        if filter.get(&r) {
            false_positives += 1;
        }
    }
    info!("false_positives: {} of {} elements checked were missed", false_positives, words.len());
}

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default())
        .expect("failed to initialize logger");
    exercise_filter();
}
