use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use clap::{ColorChoice, Parser};
use reqwest::Client;

use crate::utils::{
    get_content_type, parse_kv_pair, parse_url, print_body, print_headers, print_status,
};

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    author = "Clstilmldy <celestialmelodygo@gmail.com>",
    help_template = "version: {version}\nauthor: {author}\n\n{usage}\n\n{all-args}"
)]
#[command(color = ColorChoice::Always)]
pub struct Opts {
    #[command(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    Post(Post),
    Get(Get),
}

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
pub struct Post {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KVPair>,
}

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
pub struct Get {
    #[arg(value_parser = parse_url)]
    pub url: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KVPair {
    pub key: String,
    pub value: String,
}

impl FromStr for KVPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.splitn(2, "=");
        let err = || anyhow!("invalid key-value pair: {}", s);
        Ok(Self {
            key: splits.next().ok_or(err())?.to_string(),
            value: splits.next().ok_or(err())?.to_string(),
        })
    }
}

pub async fn get(client: Client, args: &Get) -> anyhow::Result<()> {
    let resp = client.get(&args.url).send().await?;
    print_status(&resp);
    print_headers(&resp);

    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, body);
    Ok(())
}

pub async fn post(client: Client, args: &Post) -> anyhow::Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.key, &pair.value);
    }

    let resp = client.post(&args.url).json(&body).send().await?;
    print_status(&resp);
    print_headers(&resp);

    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, body);

    Ok(())
}
