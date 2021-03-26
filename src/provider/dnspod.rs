
use super::interface::*;
use crate::error::Error;

use ureq;
use serde_json::Value;

pub struct Provider {
    key:    String,
    domain: String,
}

const CAO_USER_AGENT:         &str = concat!("cao ", env!("CARGO_PKG_VERSION"),", Johann Li <me@qinka.pro>") ;
const HTTP_HEADER_USER_AGENT: &str = "User-Agent";

const CAO_FORM_TOKEN:    &str = "login_token";
const CAO_FORM_DOMAIN:   &str = "domain";
const CAO_FORM_SDOMAIN:  &str = "sub_domain";
const CAO_FORM_RID:      &str = "record_id";
const CAO_FORM_RTYPE:    &str = "record_type";
const CAO_FORM_VALUE:    &str = "value";
const CAO_FORM_RLINE:    &str = "record_line";
const CAO_FORM_RLINE_ID: &str = "record_line_id";
const CAO_FORM_OFFSET:   &str = "offset";
const CAO_FORM_LENGTH:   &str = "length";


const DNSPOD_ADD_RECORD:    &str = "https://dnsapi.cn/Record.Create";
const DNSPOD_LIST_RECORDS:  &str = "https://dnsapi.cn/Record.List";
const DNSPOD_MODIFY_RECORD: &str = "https://dnsapi.cn/Record.Modify";
const DNSPOD_DELETE_RECORD: &str = "https://dnsapi.cn/Record.Remove";
const DNSPOD_INFO_RECORD:   &str = "https://dnsapi.cn/Record.Info";


impl DnsProvider for Provider {

    fn build_provider(key: String, domain: String) -> Result<Box<dyn DnsProvider>, Error> {
        Ok(Box::new(Provider{key: key, domain: domain}))
    }
    fn add_record(&self, sub_domain: String, record_type: String, record_line: String, value: String) -> Result<i32, Error> {

        let record_line = if record_line.chars().all(char::is_numeric) {
                (CAO_FORM_RLINE_ID, record_line.as_str())
            } else {
                (CAO_FORM_RLINE,    record_line.as_str())
            };

        let result: Value = ureq::post(DNSPOD_ADD_RECORD)
            .set(HTTP_HEADER_USER_AGENT, CAO_USER_AGENT)
            .send_form(&[
                (CAO_FORM_TOKEN,   self.key.as_str()),
                (CAO_FORM_DOMAIN,  self.domain.as_str()),
                (CAO_FORM_SDOMAIN, sub_domain.as_str()),
                (CAO_FORM_RTYPE,   record_type.as_str()),
                record_line,
                (CAO_FORM_VALUE,   value.as_str()),
            ])?
            .into_json()?;
        eprintln!("{:?}",result);

        if result["status"]["code"].is_string() && result["status"]["code"].as_str().unwrap().eq("1") {
            if let Value::String(id) = &result["record"]["id"] {
                id.parse::<i32>().map_err(|e| Error::InterfaceError(format!("Failed to parse {}, because {}", id, e)))
            } else {
                Err(Error::InterfaceError(format!("Failed to parse {:?}", result["status"]["code"])))
            }
        } else {
            Err(Error::ProviderError(result.to_string()))
        }
    }
    fn list_record(&self, offset: Option<i32>, length: Option<i32>, sub_domain: Option<String>) -> Result<Vec<Record>, Error> {

        let mut form = vec![
            (CAO_FORM_TOKEN,  self.key.as_str()),
            (CAO_FORM_DOMAIN, self.domain.as_str()),
        ];
        let offset_s = offset.map_or(String::from(""), |i| i.to_string());
        let length_s = length.map_or(String::from(""), |i| i.to_string());
        let subdom_s = sub_domain.clone().unwrap_or(String::from(""));

        if offset.is_some() {
            form.push((CAO_FORM_OFFSET, offset_s.as_str()));
        }
        if length.is_some() {
            form.push((CAO_FORM_LENGTH, length_s.as_str()));
        }
        if sub_domain.is_some() {
            form.push((CAO_FORM_SDOMAIN, subdom_s.as_str()));
        }

        let result: Value = ureq::post(DNSPOD_LIST_RECORDS)
            .set(HTTP_HEADER_USER_AGENT, CAO_USER_AGENT)
            .send_form(form.as_slice())?
            .into_json()?;
        eprintln!("{:?}",result);

        if result["status"]["code"].is_string() && result["status"]["code"].as_str().unwrap().eq("1") {
            if let Value::Array(list) = &result["records"] {
                Ok(list.into_iter().filter_map(|v| {
                    let id = if let Value::String(id) = &v["id"] {
                            id.parse::<i32>().unwrap_or(-1)
                        } else { -2 };
                    let sub_domain = v["name"].as_str()?;

                    Some(super::interface::Record{
                        id: id,
                        sub_domain: String::from(v["name"].as_str()?),
                        value:  String::from(v["value"].as_str()?),
                        r_type: String::from(v["type"] .as_str()?),
                        r_line: String::from(v["line"] .as_str()?),
                    })
                }).collect())
            } else {
                Err(Error::InterfaceError(format!("Failed to parse {:?}", result["status"]["code"])))
            }
        } else {
            Err(Error::ProviderError(result.to_string()))
        }
    }
    fn modify_record(&self, id: i32, sub_domain: Option<String>, r_type: String, r_line: String, value: String) -> Result<(), Error>{

        let id = id.to_string();

        let result: Value = ureq::post(DNSPOD_INFO_RECORD)
            .set(HTTP_HEADER_USER_AGENT, CAO_USER_AGENT)
            .send_form(&[
                (CAO_FORM_TOKEN, self.key.as_str()),
                (CAO_FORM_DOMAIN, self.domain.as_str()),
                (CAO_FORM_RID, id.as_str()),
            ])?
            .into_json()?;
        eprintln!("{:?}",result);

        if result["status"]["code"].is_string() && result["status"]["code"].as_str().unwrap().eq("1") {
            if let Value::String(old_value) = &result["record"]["value"] {
                if old_value.eq(&value) {
                    eprintln!("Same record value");
                    return Ok(());
                }
            }
        } else {
            return Err(Error::ProviderError(result.to_string()));
        }

        let mut form = vec![
            (CAO_FORM_TOKEN, self.key.as_str()),
            (CAO_FORM_DOMAIN, self.domain.as_str()),
            (CAO_FORM_RID, id.as_str()),
            (CAO_FORM_RTYPE, r_type.as_str()),
            (CAO_FORM_VALUE, value.as_str()),
        ];

        let subdom_s = sub_domain.clone().unwrap_or(String::from(""));

        if sub_domain.is_some() {
            form.push((CAO_FORM_SDOMAIN, subdom_s.as_str()));
        }
        if r_line.chars().all(char::is_numeric) {
            form.push((CAO_FORM_RLINE_ID, r_line.as_str()));
        } else {
            form.push((CAO_FORM_RLINE,    r_line.as_str()));
        }

        let result: Value = ureq::post(DNSPOD_MODIFY_RECORD)
            .set(HTTP_HEADER_USER_AGENT, CAO_USER_AGENT)
            .send_form(form.as_slice())?
            .into_json()?;
        eprintln!("{:?}",result);

        if result["status"]["code"].is_string() && result["status"]["code"].as_str().unwrap().eq("1") {
            Ok(())
        } else {
            Err(Error::ProviderError(result.to_string()))
        }
    }
    fn delete_record(&self, id: i32) -> Result<(), Error> {
        let id = id.to_string();

        let result: Value = ureq::post(DNSPOD_DELETE_RECORD)
            .set(HTTP_HEADER_USER_AGENT, CAO_USER_AGENT)
            .send_form(&[
                (CAO_FORM_TOKEN, self.key.as_str()),
                (CAO_FORM_DOMAIN, self.domain.as_str()),
                (CAO_FORM_RID, id.as_str()),
            ])?
            .into_json()?;
        eprintln!("{:?}",result);

        if result["status"]["code"].is_string() && result["status"]["code"].as_str().unwrap().eq("1") {
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

        let sleep_time = time::Duration::from_secs(20);

        let domain = var("DNSPOD_TEST_DOMAIN")
            .expect("Need environment variable: DNSPOD_TEST_DOMAIN");
        let sub_domain = var("DNSPOD_TEST_SUB_DOMAIN")
            .expect("Need environment variable: DNSPOD_TEST_SUB_DOMAIN");
        let full_domain = format!("{}.{}", sub_domain.clone(), domain.clone());

        let mut resolver_config = ResolverConfig::new();
        resolver_config.add_name_server(NameServerConfig{
            socket_addr: "119.29.29.29:53".parse().unwrap(),
            protocol: Protocol::Udp,
            tls_dns_name: None,
            trust_nx_responses: true,
        });
        let mut resolver_opt = ResolverOpts::default();
        resolver_opt.attempts = 10;
        resolver_opt.negative_max_ttl = Some(time::Duration::from_secs(1));
        let mut resolver = Resolver::new(ResolverConfig::default(), resolver_opt).unwrap();

        let provider = Provider{
            key: var("DNSPOD_TEST_TOKEN").expect("Need environment variable: DNSPOD_TEST_TOKEN"),
            domain: domain.clone()
        };

        sleep(sleep_time);

        let response = resolver.lookup_ip(full_domain.as_str());
        println!("1. {:?}", response);
        if response.is_ok() {
            let address = response.iter().next().expect("no addresses returned!");
            println!("1.1. {:?}", address);
            panic!("Should clean test domain before the test! {:?}", response);
        }

        let id = provider
            .add_record(sub_domain.clone(), String::from("A"), String::from("0"), String::from("1.2.3.4"))
            .unwrap();

        sleep(sleep_time);

        let mut response = resolver.lookup_ip(full_domain.as_str()).unwrap();
        println!("2. {:?}", response);
        let address = response.iter().next().expect("no addresses returned!");
        assert_eq!(address, IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)));

        let records = provider
            .list_record(Some(0), Some(1), Some(sub_domain.clone()))
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

        provider
            .modify_record(id, None, String::from("A"), String::from("0"), String::from("2.3.4.5"))
            .unwrap();

        sleep(sleep_time);

        let mut response = resolver.lookup_ip(full_domain.as_str()).unwrap();
        let address = response.iter().next().expect("no addresses returned!");
        assert_eq!(address, IpAddr::V4(Ipv4Addr::new(2, 3, 4, 5)));

        provider
            .delete_record(id)
            .unwrap();

        sleep(sleep_time);

        let response = resolver.lookup_ip(full_domain.as_str());
        if response.is_ok() {
            panic!("Record be deleted the test!");
        }
    }

}