use clap::Parser;

#[derive(Parser)]
#[clap(author,
version,
about,
long_about = "https://github.com/brootware/crate-search")]
struct Args {
    // Crate name to search for
    name: String,

    // Limit the number of results
    #[clap(short='l', long="limit")]
    limit: f32,

    // Search for an exact match
    #[clap(short='e', long="exact")]
    exact: bool,
}

fn main() {
    let args = Args::parse();

    
}
