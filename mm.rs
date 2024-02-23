use criterion::{criterion_group, criterion_main, Criterion};
use sz_rust_bench::{open, random_token};

fn mm_benchmark(c: &mut Criterion) {
    let file = &open();
    let mut rng = rand::thread_rng();
    let token = random_token(&mut rng, file);

    c.bench_function("memchr::memmem::find", |b| {
        b.iter(|| memchr::memmem::find(&file, token))
    });
}

criterion_group!(sz_bench, mm_benchmark);
criterion_main!(sz_bench);
