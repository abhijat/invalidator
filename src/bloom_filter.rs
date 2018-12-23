use std::sync::Arc;
use std::sync::Mutex;

use bit_vec::BitVec;
use fasthash::metro;

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

    pub fn put(&mut self, item: &str) {
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rand::distributions::Alphanumeric;
    use rand::Rng;
    use rand::thread_rng;

    use super::*;

    fn data_set_of_size(size: usize, word_size: usize) -> HashSet<String> {
        let mut data = HashSet::with_capacity(size);
        for _ in 0..size {
            let s: String = thread_rng().sample_iter(&Alphanumeric).take(word_size).collect();
            data.insert(s);
        }
        data
    }

    #[test]
    fn test_false_negatives() {
        let data = data_set_of_size(1 * 1000 * 1000, 16);
        let mut filter = BloomFilter::new();
        data.iter().for_each(|w| filter.put(w));
        data.iter().for_each(|w| {
            if !filter.get(w) {
                panic!("unexpected false negative!");
            }
        });
    }

    #[test]
    fn test_false_positives() {
        let data = data_set_of_size(1 * 1000 * 1000, 16);
        let mut filter = BloomFilter::new();

        data.iter().for_each(|w| filter.put(w));

        let num_invalid = data.iter()
            .map(|w| format!("{}{}{}", w, w, w))
            .filter(|w| filter.get(w))
            .count();

        assert!(num_invalid < 100);
    }

}