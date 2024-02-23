#[derive(clap::Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    pub path: std::path::PathBuf,

    #[arg(short, long)]
    pub token: String,
}
