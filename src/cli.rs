use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
)]
pub struct Args {
    #[arg(short = 'd', required = true)]
    pub domain: String,

    #[arg(short = 'w', required = true)]
    pub wordlist: String,

    #[arg(short = 'o')]
    pub output: Option<String>,

    #[arg(short = 'D', long = "doh")]
    pub dns_over_https: bool,

    #[arg(short = 'v')]
    pub verbose: bool,

    #[arg()]
    pub ip_version: Option<IpVersion>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum IpVersion {
    #[value(name = "4")]
    IPv4,
    #[value(name = "6")]
    IPv6,
}
