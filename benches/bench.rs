use criterion::{criterion_group, criterion_main, Criterion};
use binary_search::{find, find_loop};
use rand::Rng;


// bench does not run. why?

pub fn generate_random_array(size: usize, range: std::ops::Range<i32>) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut array: Vec<i32> = (0..size).map(|_| rng.gen_range(range.clone())).collect();
    array.sort();
    array
}

fn benchmark(c: &mut Criterion) {
    let array = generate_random_array(1000, 0..1000);
    let key = 600;

    c.bench_function("find", |b| b.iter(|| find(&array, key)));
    c.bench_function("find_loop", |b| b.iter(|| find_loop(&array, key)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);