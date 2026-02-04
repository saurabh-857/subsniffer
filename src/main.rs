mod cli;
mod core;
mod error;
mod resolver;
mod sources;
mod output;
mod utils;

use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = core::engine::run(args).await {
        eprintln!("[!] {}", e);
        std::process::exit(1);
    }
}





// use clap::{Parser, ValueEnum};
// use std::fs::{self, File};
// use std::io::Write;
// use std::path::Path;
// use trust_dns_resolver::{
//     TokioAsyncResolver,
//     config::{ResolverConfig, ResolverOpts},
// };

// #[derive(Parser)]
// #[command(
//     author,
//     version,
//     about,
//     long_about = None,
//     help_template = "\n\n{bin} {version}\n{author}\n{about}\n\n{usage}\n\n{all-args}\n",
//     override_usage = "subsniffer -d <DOMAIN> -w <WORDLIST> [OPTIONS] [IP_VERSION]"
// )]
// struct Args {
//     #[arg(
//         short = 'd',
//         required = true,
//         help = "Target domain to enumerate (e.g., 'example.com')"
//     )]
//     domain: String,

//     #[arg(
//         short = 'w',
//         required = true,
//         help = "Path to wordlist file for brute-forcing subdomains"
//     )]
//     wordlist: String,

//     #[arg(short = 'o', help = "Output file for subdomain results")]
//     output: Option<String>,

//     /// Use encryption like DNS-over-HTTPS (Cloudflare resolver)
//     #[arg(short = 'D', long = "doh")]
//     dns_protocol: bool,

//     #[arg(short = 'v', help = "Show verbose output including IP addresses")]
//     verbose: bool,

//     #[arg(help = "IP version to query ('4' for IPv4, '6' for IPv6, omit for both)")]
//     ip_version: Option<IPversion>,
// }

// #[derive(ValueEnum, Clone)]
// enum IPversion {
//     #[value(name = "4")]
//     IPv4,
//     #[value(name = "6")]
//     IPv6,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args = Args::parse();

//     let mut output_file = if let Some(output_path) = &args.output {
//         Some(File::create(output_path)?)
//     } else {
//         None
//     };

//     println!("\n[+] Target domain: {0}", args.domain);
//     if args.verbose {
//         println!("[+] Verbose mode enabled - showing IP addresses");
//     }
//     if args.dns_protocol {
//         println!("[+] Using DNS-over-HTTPS (Cloudflare resolver)");
//     }

//     let resolver = if args.dns_protocol {
//         TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default())?
//     } else {
//         TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())?
//     };

//     let root_domain = format!("{}", args.domain.trim());
//     resolve_subdomain(
//         &resolver,
//         &root_domain,
//         &mut output_file,
//         &args.ip_version,
//         args.verbose,
//     )
//     .await?;

//     let wordlist_path = args.wordlist;
//     if !Path::new(&wordlist_path).exists() {
//         return Err(format!("[!] Wordlist file '{wordlist_path}' not found").into());
//     }
//     let wordlist = fs::read_to_string(wordlist_path)?;
//     for line in wordlist.lines() {
//         let subdomain = line.trim();
//         if subdomain.is_empty() {
//             continue;
//         }
//         let fqdn = format!("{}.{}", subdomain, root_domain);
//         resolve_subdomain(
//             &resolver,
//             &fqdn,
//             &mut output_file,
//             &args.ip_version,
//             args.verbose,
//         )
//         .await?;
//     }

//     Ok(())
// }

// async fn resolve_subdomain(
//     resolver: &TokioAsyncResolver,
//     fqdn: &str,
//     output_file: &mut Option<File>,
//     ip_version: &Option<IPversion>,
//     verbose: bool,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let mut found = false;

//     match ip_version {
//         None => {
//             if let Ok(lookup) = resolver.lookup_ip(fqdn).await {
//                 found = true;
//                 if verbose {
//                     for ip in lookup.iter() {
//                         println!("{fqdn} -> {ip}");
//                         if let Some(file) = output_file {
//                             writeln!(file, "{fqdn} -> {ip}")?;
//                         }
//                     }
//                 } else {
//                     println!("{fqdn}");
//                     if let Some(file) = output_file {
//                         writeln!(file, "{fqdn}")?;
//                     }
//                 }
//             }
//         }
//         Some(IPversion::IPv4) => {
//             if let Ok(lookup) = resolver.ipv4_lookup(fqdn).await {
//                 found = true;
//                 if verbose {
//                     for ip in lookup.iter() {
//                         println!("{fqdn} -> {ip}");
//                         if let Some(file) = output_file {
//                             writeln!(file, "{fqdn} -> {ip}")?;
//                         }
//                     }
//                 } else {
//                     println!("{fqdn}");
//                     if let Some(file) = output_file {
//                         writeln!(file, "{fqdn}")?;
//                     }
//                 }
//             }
//         }
//         Some(IPversion::IPv6) => {
//             if let Ok(lookup) = resolver.ipv6_lookup(fqdn).await {
//                 found = true;
//                 if verbose {
//                     for ip in lookup.iter() {
//                         println!("{fqdn} -> {ip}");
//                         if let Some(file) = output_file {
//                             writeln!(file, "{fqdn} -> {ip}")?;
//                         }
//                     }
//                 } else {
//                     println!("{fqdn}");
//                     if let Some(file) = output_file {
//                         writeln!(file, "{fqdn}")?;
//                     }
//                 }
//             }
//         }
//     }

//     if !found && verbose {
//         println!("{fqdn} -> Not resolved");
//         if let Some(file) = output_file {
//             writeln!(file, "{fqdn} -> Not resolved")?;
//         }
//     }

//     Ok(())
// }
