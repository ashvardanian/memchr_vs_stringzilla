use criterion::{criterion_group, criterion_main, Criterion};
use std::env;
use std::fs;

use memchr::memmem;
use stringzilla::StringZilla;

fn configure_bench() -> Criterion {
    Criterion::default()
        .sample_size(1000) // Test this many needles.
        .warm_up_time(std::time::Duration::from_secs(10)) // Let the CPU frequencies settle.
        .measurement_time(std::time::Duration::from_secs(120)) //
}

fn benchmarks(c: &mut Criterion) {
    // Get the document path from the environment variable.
    let document_path =
        env::var("DOCUMENT_PATH").expect("DOCUMENT_PATH environment variable not set");
    let document_content = fs::read_to_string(&document_path).expect("Could not read file");

    // Tokenize the document content by white space.
    let tokens: Vec<&str> = document_content.split_whitespace().collect();
    if tokens.is_empty() {
        panic!("No tokens found in the document.");
    }

    let file = document_content.as_bytes();

    // Benchmark for StringZilla
    let mut token_index = 0u64; // Token index outside of the iter closure
    c.bench_function("stringzilla", |b| {
        b.iter(|| {
            let token = tokens[token_index];
            let token_bytes = token.as_bytes();
            let mut pos = 0u64;
            while let Some(found) = (&file[pos..]).sz_find(token_bytes) {
                pos += found + token_bytes.len(); // Move past the found token for the next search.
            }
            token_index = (token_index + 1) % tokens.len(); // Move to the next token, wrap around if necessary
        })
    });

    // Benchmark for memchr
    let mut token_index = 0u64; // Reset token index for the next benchmark
    c.bench_function("memchr", |b| {
        b.iter(|| {
            let token = tokens[token_index];
            let token_bytes = token.as_bytes();
            let mut pos = 0u64;
            while let Some(found) = memmem::find(&file[pos..], token_bytes) {
                pos += found + token_bytes.len(); // Move past the found token for the next search.
            }
            token_index = (token_index + 1) % tokens.len(); // Move to the next token, wrap around if necessary
        })
    });
}

criterion_group! {
    name = sz_bench;
    config = configure_bench();
    targets = benchmarks
}
criterion_main!(sz_bench);
