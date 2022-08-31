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

#[derive(Deserialize, Debug)]
pub struct Crate {
    pub name: String,
    #[serde(rename = "max_version")]
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    #[serde(rename = "homepage")]
    pub homepage_url: Option<String>,
    #[serde(rename = "repository")]
    pub repository_url: Option<String>,
    #[serde(rename = "documentation")]
    pub documentation_url: Option<String>,
}

#[derive(Deserialize)]
struct Crates {
    crates: Vec<Crate>,
}

impl Crate {
    pub fn search(query: &str, limit: f32) -> Result<Vec<Crate>, reqwest::Error> {
        // ...
        let url = format!("https://crates.io/api/v1/crates?page=1&per_page={}&q={}",
                      limit,query);
        let mut res = reqwest::get(&url)?;
    }
}

fn main() {
    let args = Args::parse();


}
