use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::env;
use std::fs;
use std::str;


fn benchmarks(c: &mut Criterion) {
    use stringzilla::StringZilla;

    // Get the document path from the first command line argument.
    let document_path = env::var("DOCUMENT_PATH").expect("DOCUMENT_PATH environment variable not set");
    let document_content = fs::read_to_string(document_path).expect("Could not read file");

    // Tokenize the document content by white space.
    let tokens: Vec<&str> = document_content.split_whitespace().collect();
    if tokens.is_empty() {
        panic!("No tokens found in the document.");
    }

    let file = document_content.as_bytes();

    let mut group = c.benchmark_group("TokenSearch");
    for (i, token) in tokens.iter().enumerate() {
        let token_bytes = token.as_bytes();

        group.bench_function(BenchmarkId::new("stringzilla", i), |b| {
            b.iter(|| {
                let mut pos = 0;
                while let Some(found) = (&file[pos..]).sz_find(token_bytes) {
                    pos += found + token_bytes.len(); // Move past the found token for the next search.
                }
            })
        });

        group.bench_function(BenchmarkId::new("memchr", i), |b| {
            b.iter(|| {
                let mut pos = 0;
                while let Some(found) = memchr::memmem::find(&file[pos..], token_bytes) {
                    pos += found + token_bytes.len(); // Move past the found token for the next search.
                }
            })
        });
    }

    group.finish();
}

criterion_group!(sz_bench, benchmarks);
criterion_main!(sz_bench);
