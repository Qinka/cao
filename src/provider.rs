

pub mod interface;

#[cfg(feature = "dnspod")]
mod dnspod;

use self::interface::{DnsProvider};
use crate::error::{Error};

fn provider_register(provider: &String)
-> Result<&dyn Fn(String, String) -> Result<Box<dyn DnsProvider>, Error>, Error> {
    match provider as &str {
        #[cfg(feature = "dnspod")]
        "dnspod" => Ok(&dnspod::Provider::build_provider),
        _ => Err(Error::UnimplementedProvider(provider.clone())),
    }
}

pub fn build_dns_provider(provider: String, key: String, domain: String)
-> Result<Box<dyn DnsProvider>, Error> {
    let func = provider_register(&provider)?;
    func(key, domain)
}