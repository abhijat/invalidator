use bit_vec::BitVec;
use fasthash::metro;
use std::sync::Arc;
use std::sync::Mutex;

pub struct BloomFilter {
    n_bits: u128,
    n_hashes: u128,
    backing_store: BitVec,
}

impl BloomFilter {
    pub fn new() -> Self {
        let n_bits = u128::pow(2, 32);

        BloomFilter {
            n_bits,
            n_hashes: 5,
            backing_store: BitVec::from_elem(n_bits as usize, false),
        }
    }

    pub fn add(&mut self, item: &str) {
        let (first, second) = hash_pairs(item);
        for i in 0..self.n_hashes {
            let index = first + (i * second);
            let index = index & (self.n_bits - 1);
            self.backing_store.set(index as usize, true);
        }
    }

    pub fn get(&self, item: &str) -> bool {
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


pub struct AppState {
    pub filter: Arc<Mutex<BloomFilter>>,
}

pub fn hash_pairs(payload: &str) -> (u128, u128) {
    let first_hash = metro::hash64(&payload);
    let second_hash = metro::hash64(&format!("{:x}", first_hash));
    (first_hash as u128, second_hash as u128)
}
