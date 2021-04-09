

pub mod interface;

#[cfg(feature = "dnspod")]
mod dnspod;

use self::interface::{DnsProvider};
use crate::error::{Error};

type BoxDnsProvider = Box<dyn DnsProvider>;
type DnsProviderMaker<'a> = &'a dyn Fn(String, String) -> Result<BoxDnsProvider, Error>;

fn provider_register(provider: &str)
-> Result<DnsProviderMaker, Error> {
    match provider as &str {
        #[cfg(feature = "dnspod")]
        "dnspod" => Ok(&dnspod::Provider::build_provider),
        _ => Err(Error::UnimplementedProvider(String::from(provider))),
    }
}

pub fn build_dns_provider(provider: String, key: String, domain: String)
-> Result<Box<dyn DnsProvider>, Error> {
    let func = provider_register(&provider)?;
    func(key, domain)
}