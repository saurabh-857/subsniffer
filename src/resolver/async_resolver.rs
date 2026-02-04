use trust_dns_resolver::{
    TokioAsyncResolver,
    config::{ResolverConfig, ResolverOpts},
};

pub fn create_resolver(doh: bool) -> TokioAsyncResolver {
    if doh {
        TokioAsyncResolver::tokio(
            ResolverConfig::cloudflare(),
            ResolverOpts::default(),
        ).expect("resolver init failed")
    } else {
        TokioAsyncResolver::tokio(
            ResolverConfig::default(),
            ResolverOpts::default(),
        ).expect("resolver init failed")
    }
}
