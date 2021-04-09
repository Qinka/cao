
use serde_json::Value;

use super::interface::*;
use crate::error::Error;

#[cfg(feature = "curl")]
#[path = "_providers/dnspod_curl.rs"]
mod intra;

#[cfg(feature = "ureq")]
#[path = "_providers/dnspod_ureq.rs"]
mod intra;

pub struct Provider {
    key:    String,
    domain: String,
}


impl DnsProvider for Provider {
    fn build_provider(
        key:    String,
        domain: String,
    ) -> Result<Box<dyn DnsProvider>, Error> {
        Ok(Box::new(Provider{key, domain}))
    }

    fn add_record(
        &self,
        sub_domain:  &str,
        record_type: &str,
        record_line: &str,
        value:       &str,
    ) -> Result<i32, Error> {
        let result = intra::add_record(
            &self.domain,
            &self.key,
            sub_domain,
            record_type,
            record_line,
            value,
        )?;

        #[cfg(debug_assertions)]
        eprintln!("{:?}",result);

        if  result["status"]["code"].is_string()
        && result["status"]["code"].as_str().unwrap().eq("1") {
            if let Value::String(id) = &result["record"]["id"] {
                id.parse::<i32>()
                .map_err(|e| Error::InterfaceError(
                    format!("Failed to parse {}, because {}", id, e)
                ))
            } else {
                Err(Error::InterfaceError(
                    format!("Failed to parse {:?}", result["status"]["code"])
                ))
            }
        } else {
            Err(Error::ProviderError(result.to_string()))
        }
    }

    fn list_record(
        &self,
        offset:     Option<i32>,
        length:     Option<i32>,
        sub_domain: Option<&str>,
    ) -> Result<Vec<Record>, Error> {
        let offset = offset.map(|i| i.to_string());
        let length = length.map(|i| i.to_string());
        let result = intra::list_record(
            &self.domain,
            &self.key,
            offset.as_deref(),
            length.as_deref(),
            sub_domain,
        )?;

        #[cfg(debug_assertions)]
        eprintln!("{:?}",result);

        if result["status"]["code"].is_string()
        && result["status"]["code"].as_str().unwrap().eq("1") {
            if let Value::Array(list) = &result["records"] {
                Ok(list.iter().filter_map(|v| {
                    let id = if let Value::String(id) = &v["id"] {
                            id.parse::<i32>().unwrap_or(-1)
                        } else { -2 };
                    Some(super::interface::Record{
                        id,
                        sub_domain: String::from(v["name"].as_str()?),
                        value:  String::from(v["value"].as_str()?),
                        r_type: String::from(v["type"] .as_str()?),
                        r_line: String::from(v["line"] .as_str()?),
                    })
                    }).collect()
                )
            } else {
                Err(Error::InterfaceError(
                    format!("Failed to parse {:?}", result["status"]["code"])
                ))
            }
        } else {
            Err(Error::ProviderError(result.to_string()))
        }
    }

    fn modify_record(
        &self,
        id:         i32,
        sub_domain: Option<&str>,
        r_type:     &str,
        r_line:     &str,
        value:      &str,
    ) -> Result<(), Error>{
        let id = id.to_string();

        // Check the record, and update record, unless not same.
        let result = intra::info_record(
            &self.domain,
            &self.key,
            &id,
        )?;

        #[cfg(debug_assertions)]
        eprintln!("{:?}",result);

        if result["status"]["code"].is_string()
        && result["status"]["code"].as_str().unwrap().eq("1") {
            if let Value::String(old_value) = &result["record"]["value"] {
                if old_value.eq(value) {
                    #[cfg(debug_assertions)]
                    eprintln!("Same record value");

                    return Ok(());
                }
            }
        } else {
            return Err(Error::ProviderError(result.to_string()));
        }

        let result = intra::modify_record(
            &self.domain,
            &self.key,
            &id,
            sub_domain,
            &r_type,
            &r_line,
            &value,
        )?;

        #[cfg(debug_assertions)]
        eprintln!("{:?}",result);

        if result["status"]["code"].is_string()
        && result["status"]["code"].as_str().unwrap().eq("1") {
            Ok(())
        } else {
            Err(Error::ProviderError(result.to_string()))
        }
    }

    fn delete_record(
        &self,
        id:   i32,
    ) -> Result<(), Error> {
        let id = id.to_string();


        let result = intra::delete_record(
            &self.domain,
            &self.key,
            &id,
        )?;

        #[cfg(debug_assertions)]
        eprintln!("{:?}",result);

        if result["status"]["code"].is_string()
        && result["status"]["code"].as_str().unwrap().eq("1") {
            Ok(())
        } else {
            Err(Error::ProviderError(result.to_string()))
        }
    }
}


#[cfg(test)]
mod test {
    use super::Provider;
    use crate::provider::interface::{DnsProvider, Record};
    use trust_dns_resolver::Resolver;
    use trust_dns_resolver::config::*;

    #[test]
    fn test_dnspod_record_actions() {
        use std::env::var;
        use std::net::*;
        use std::thread::sleep;
        use std::time;

        let sleep_time = time::Duration::from_secs(150);

        let domain = var("DNSPOD_TEST_DOMAIN")
            .expect("Need environment variable: DNSPOD_TEST_DOMAIN");
        let sub_domain = var("DNSPOD_TEST_SUB_DOMAIN")
            .expect("Need environment variable: DNSPOD_TEST_SUB_DOMAIN");
        let full_domain = format!("{}.{}", sub_domain.clone(), domain.clone());

        let type_a      = String::from("A");
        let r_line_id   = String::from("0");
        let value_a     = String::from("1.2.3.4");
        let value_b     = String::from("2.3.4.5");

        let mut resolver_config = ResolverConfig::new();
        resolver_config.add_name_server(NameServerConfig{
            socket_addr: "119.29.29.29:53".parse().unwrap(),
            protocol: Protocol::Udp,
            tls_dns_name: None,
            trust_nx_responses: true,
        });
        let mut resolver_opt = ResolverOpts::default();
        resolver_opt.attempts = 10;
        resolver_opt.negative_max_ttl = Some(time::Duration::from_micros(1000));
        let resolver = Resolver::new(ResolverConfig::default(), resolver_opt).unwrap();

        let provider = Provider{
            key: var("DNSPOD_TEST_TOKEN").expect("Need environment variable: DNSPOD_TEST_TOKEN"),
            domain: domain.clone()
        };

        sleep(sleep_time);

        let response = resolver.lookup_ip(full_domain.as_str());
        println!("1. {:?}", response);
        if response.is_ok() {
            // let address = response.iter().next().expect("no addresses returned!");
            panic!("Should clean test domain before the test! {:?}", response);
        }

        let id = provider
            .add_record(
                &sub_domain,
                &type_a,
                &r_line_id,
                &value_a,
            )
            .unwrap();

        sleep(sleep_time);

        let response = resolver.lookup_ip(full_domain.as_str()).unwrap();
        println!("2. {:?}", response);
        let address = response.iter().next().expect("no addresses returned!");
        assert_eq!(address, IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)));

        let records = provider
            .list_record(Some(0), Some(1), Some(&sub_domain))
            .unwrap();

        assert_eq!(
            records[0],
            Record{
                id: id,
                sub_domain: sub_domain.clone(),
                value: String::from("1.2.3.4"),
                r_type: String::from("A"),
                r_line: String::from("默认"),
            });

        sleep(sleep_time);

        provider
            .modify_record(
                id,
                Some(&sub_domain),
                &type_a,
                &r_line_id,
                &value_b,
            )
            .unwrap();

        sleep(sleep_time);

        let response = resolver.lookup_ip(full_domain.as_str()).unwrap();
        let address = response.iter().next().expect("no addresses returned!");
        assert_eq!(address, IpAddr::V4(Ipv4Addr::new(2, 3, 4, 5)));

        provider
            .delete_record(id)
            .unwrap();

        sleep(sleep_time);
        sleep(sleep_time);

        let response = resolver.lookup_ip(full_domain.as_str());
        if response.is_ok() {
            panic!("Record should be deleted the test!");
        }
    }
}