use clap::Parser;
use stringzilla::StringZilla;
use sz_rust_bench::Args;

fn main() {
    let args = Args::parse();
    let file = std::fs::read(args.path).unwrap();
    let _ = file.sz_find(args.token);
}
