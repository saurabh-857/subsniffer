use crate::{
    cli::Args,
    resolver::async_resolver::create_resolver,
    sources::dns_bruteforce,
};

pub async fn run(args: Args) -> Result<(), crate::error::SubsnifferError> {
    let resolver = create_resolver(args.dns_over_https);

    dns_bruteforce::run(
        &resolver,
        &args.domain,
        &args.wordlist,
        &args.ip_version,
        args.verbose,
    ).await?;

    Ok(())
}
