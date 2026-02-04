use trust_dns_resolver::TokioAsyncResolver;
use std::fs;
use std::path::Path;

use crate::cli::IpVersion;
use crate::error::SubsnifferError;

pub async fn run(
    resolver: &TokioAsyncResolver,
    domain: &str,
    wordlist_path: &str,
    ip_version: &Option<IpVersion>,
    verbose: bool,
) -> Result<(), SubsnifferError> {

    if !Path::new(wordlist_path).exists() {
        return Err(SubsnifferError::WordlistNotFound(wordlist_path.into()));
    }

    let wordlist = fs::read_to_string(wordlist_path)?;

    for line in wordlist.lines() {
        let sub = line.trim();
        if sub.is_empty() {
            continue;
        }

        let fqdn = format!("{}.{}", sub, domain);

        match ip_version {
            None => {
                if let Ok(lookup) = resolver.lookup_ip(fqdn.as_str()).await {
                    if verbose {
                        for ip in lookup.iter() {
                            println!("{fqdn} -> {ip}");
                        }
                    } else {
                        println!("{fqdn}");
                    }
                }
            }
            Some(IpVersion::IPv4) => {
                if let Ok(lookup) = resolver.ipv4_lookup(fqdn.as_str()).await {
                    if verbose {
                        for ip in lookup.iter() {
                            println!("{fqdn} -> {ip}");
                        }
                    } else {
                        println!("{fqdn}");
                    }
                }
            }
            Some(IpVersion::IPv6) => {
                if let Ok(lookup) = resolver.ipv6_lookup(fqdn.as_str()).await {
                    if verbose {
                        for ip in lookup.iter() {
                            println!("{fqdn} -> {ip}");
                        }
                    } else {
                        println!("{fqdn}");
                    }
                }
            }
        }
    }

    Ok(())
}
