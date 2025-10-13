use colored::Colorize;
// use jsonxf::pretty_print;
use mime::Mime;
use reqwest::{Response, Url, header::CONTENT_TYPE};
use syntect::{
    easy::HighlightLines,
    highlighting::Style,
    util::{LinesWithEndings, as_24_bit_terminal_escaped},
};

use crate::{GLOBAL_SYNTAX_SET, GLOBAL_THEME_SET, cmd::KVPair};

pub fn parse_url(s: &str) -> anyhow::Result<String> {
    if Url::parse(s).is_ok() {
        Ok(s.to_string())
    } else {
        Err(anyhow::anyhow!("Not a valid URL"))
    }
}

pub fn parse_kv_pair(s: &str) -> anyhow::Result<KVPair, anyhow::Error> {
    Ok(s.parse()?)
}

pub fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(CONTENT_TYPE)?
        .to_str()
        .ok()
        .and_then(|s| s.parse::<Mime>().ok())
}

pub fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}", status);
}

pub fn print_headers(resp: &Response) {
    for (header_name, header_value) in resp.headers() {
        println!("{}: {:?}", header_name.to_string().green(), header_value);
    }
    print!("\n");
}

pub fn print_body(m: Option<Mime>, body: String) {
    match m {
        Some(_m) => {
            if _m == "application/json" {
                // println!("{}", pretty_print(&body).unwrap().cyan());
                print_syntect(&body);
            } else {
                println!("{}", body.green());
            }
        }
        None => println!("{}", body.green()),
    }
}

pub fn print_syntect(s: &str) {
    let syntax = GLOBAL_SYNTAX_SET.find_syntax_by_extension("json").unwrap();
    let mut h = HighlightLines::new(syntax, &GLOBAL_THEME_SET.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &GLOBAL_SYNTAX_SET).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        println!("{}", escaped);
    }
}
