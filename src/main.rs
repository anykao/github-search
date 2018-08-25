#[macro_use]
extern crate structopt;
extern crate opener;
extern crate url;

use structopt::StructOpt;
use url::Url;

#[derive(StructOpt, Debug)]
#[structopt(name = "github search")]
struct Opt {
    /// repository
    #[structopt(short = "r", long = "repo")]
    repo: bool,

    /// crates.io
    #[structopt(short = "c", long = "crates")]
    crates: bool,

    /// docs.rs
    #[structopt(short = "d", long = "docs")]
    docs: bool,

    /// language
    #[structopt(short = "l")]
    lang: Option<String>,

    /// extension
    #[structopt(short = "e")]
    extension: Option<String>,

    /// filename
    #[structopt(short = "f")]
    filename: Option<String>,

    /// query string
    query: String,
}

fn main() {
    let opt = Opt::from_args();
    let url;
    if opt.repo {
        url = Url::parse_with_params("https://github.com/search", &[("q", opt.query)]).unwrap();
    } else if opt.crates {
        url = Url::parse_with_params("https://crates.io/search", &[("q", opt.query)]).unwrap();
    } else if opt.docs {
        url = Url::parse_with_params("https://docs.rs/releases/search", &[("query", opt.query)])
            .unwrap();
    } else {
        let mut parts = Vec::new();
        parts.push(opt.query);
        if let Some(extension) = opt.extension {
            parts.push(format!("extension:{}", extension));
        }
        if let Some(filename) = opt.filename {
            parts.push(format!("filename:{}", filename));
        }
        if let Some(lang) = opt.lang {
            parts.push(format!("language:{}", lang));
        }
        url = Url::parse_with_params(
            "https://github.com/search",
            &[
                ("q", parts.join(" ").as_str()),
                ("type", "Code"),
                ("o", "desc"),
                ("s", "indexed"),
            ],
        ).unwrap();
    }
    opener::open(url.as_str()).unwrap();
}
