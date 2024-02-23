use clap::Parser;
use memchr::memmem::find;
use sz_rust_bench::Args;

fn main() {
    let args = Args::parse();
    let file = std::fs::read(args.path).unwrap();
    let _ = find(&file, args.token.as_bytes());
}
