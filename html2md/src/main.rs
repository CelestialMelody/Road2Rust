use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
struct Ops {
    #[arg(value_parser = url::Url::parse)]
    pub url: url::Url,
    #[arg(value_parser = md_suffix)]
    pub output: String,
}

fn md_suffix(file: &str) -> anyhow::Result<String> {
    if file.ends_with(".md") {
        Ok(file.to_string())
    } else {
        Ok(format!("{}.md", file))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ops = Ops::parse();

    println!("Fetching url {}", ops.url);
    let body = reqwest::blocking::get(ops.url)?.text()?;

    println!("Converting html to markdown");
    let md = html2md::parse_html(&body);

    fs::write(&ops.output, md.as_bytes())?;
    println!("Saved to {}", &ops.output);

    Ok(())
}
