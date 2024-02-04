pub mod interface;

#[cfg(feature = "dnspod")]
mod dnspod;

use self::interface::{DnsProvider, DnsProviderBuild};
use crate::error::{Error, Result};

type BoxDnsProvider = Box<dyn DnsProvider>;

pub fn build_dns_provider(
  provider: &str,
  key: String,
  domain: String,
) -> Result<BoxDnsProvider> {
  match provider as &str {
    #[cfg(feature = "dnspod")]
    "dnspod" => Ok(Box::new(dnspod::Provider::build_provider(key, domain)?)),
    _ => Err(Error::Reason(String::from(provider))),
  }
}
