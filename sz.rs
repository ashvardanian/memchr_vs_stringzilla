use criterion::{criterion_group, criterion_main, Criterion};
use sz_rust_bench::{open, random_token};

fn sz_benchmark(c: &mut Criterion) {
    use stringzilla::StringZilla;

    let file = &open();
    let mut rng = rand::thread_rng();
    let token = random_token(&mut rng, file);

    c.bench_function("stringzilla::Stringzilla::find", move |b| {
        b.iter(|| file.sz_find(token))
    });
}

criterion_group!(sz_bench, sz_benchmark);
criterion_main!(sz_bench);
