#[macro_use]
extern crate criterion;

use std::collections::HashSet;

use criterion::Criterion;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::thread_rng;

use bloom_filter::BloomFilter;

fn data_set_of_size(size: usize, word_size: usize) -> HashSet<String> {
    let mut data = HashSet::with_capacity(size);
    for _ in 0..size {
        let s: String = thread_rng().sample_iter(&Alphanumeric).take(word_size).collect();
        data.insert(s);
    }
    data
}

fn false_negative_benchmark(c: &mut Criterion) {
    let data = data_set_of_size(1 * 100 * 100, 16);
    let mut filter = BloomFilter::new();

    data.iter().for_each(|w| filter.put(w));

    c.bench_function(
        "false-negatives 100 * 100 problem size, 16 char words",
        move |b| b.iter(|| {
            data.iter().for_each(|w| if !filter.get(w) { panic!(); });
        }));
}

criterion_group!(benches, false_negative_benchmark);
criterion_main!(benches);
