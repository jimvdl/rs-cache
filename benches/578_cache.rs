use rscache::Cache;
use once_cell::sync::Lazy;
use criterion::{ Criterion, criterion_group, criterion_main };
use std::hint::black_box;
use rand::Rng;


static CACHE: Lazy<Cache> = Lazy::new(|| {
    Cache::new("578 cache").expect("You'll need to download your own 578 cache, 
        which you can find on OpenRS2 archive (the 2009 december variant)")
});

fn fetch_file_idx19_u32(id: u32) {
    let _ = CACHE.read(19, id >> 8).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("file_fetch_idx19_u32", |b| b.iter(
        || fetch_file_idx19_u32(black_box(rand::rng().random_range(0..=15000))))
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);