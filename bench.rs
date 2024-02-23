use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use sz_rust_bench::{open, random_token};

fn benchmarks(c: &mut Criterion) {
    use stringzilla::StringZilla;

    let file = &open();
    let num_of_tokens = std::env::var("TOKENS")
        .unwrap_or("10000".into())
        .parse::<usize>()
        .unwrap();
    let mut rng = rand::thread_rng();

    let mut group = c.benchmark_group("::find");
    for i in 0..num_of_tokens {
        let token = random_token(&mut rng, file);

        group.bench_function(BenchmarkId::new("StringZilla", i), |b| {
            b.iter(|| file.sz_find(token))
        });

        group.bench_function(BenchmarkId::new("memchr", i), |b| {
            b.iter(|| memchr::memmem::find(file, token))
        });
    }

    group.finish();
}

criterion_group!(sz_bench, benchmarks);
criterion_main!(sz_bench);
