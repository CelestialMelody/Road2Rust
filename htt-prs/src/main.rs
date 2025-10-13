mod cmd;
mod utils;

use clap::Parser;
use once_cell::sync::Lazy;
use reqwest::header::USER_AGENT;
use reqwest::{Client, header};
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

use crate::cmd::{Opts, SubCommand};
use crate::cmd::{get, post};

pub static GLOBAL_SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
pub static GLOBAL_THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(USER_AGENT, "HTT-PRS".parse()?);

    let client = Client::builder().default_headers(headers).build()?;

    let result = match opts.sub_cmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{
        cmd::KVPair,
        utils::{parse_kv_pair, parse_url},
    };

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }
    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KVPair {
                key: "a".into(),
                value: "1".into()
            }
        );
        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KVPair {
                key: "b".into(),
                value: "".into()
            }
        )
    }
}
