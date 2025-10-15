use std::fs;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "html2md")]
struct Ops {
    #[structopt(help = "input url", parse(try_from_str = url::Url::parse))]
    pub url: url::Url,
    #[structopt(help = "output file", parse(from_str = md_suffix))]
    pub output: String,
}

fn md_suffix(file: &str) -> String {
    if file.ends_with(".md") {
        file.to_string()
    } else {
        format!("{}.md", file)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ops = Ops::from_args();

    println!("Fetching url {}", ops.url);
    let body = reqwest::blocking::get(ops.url)?.text()?;

    println!("Converting html to markdown");
    let md = html2md::parse_html(&body);

    fs::write(&ops.output, md.as_bytes())?;
    println!("Saved to {}", &ops.output);

    Ok(())
}
