
use super::interface::*;
use crate::error::Error;

use ureq;
use serde_json::Value;

pub struct Provider {
    key:    String,
    domain: String,
}


impl DnsProvider for Provider {

    fn build_provider(key: String, domain: String) -> Result<Box<dyn DnsProvider>, Error> {
        Ok(Box::new(Provider{key: key, domain: domain}))
    }
    fn add_record(&self, sub_domain: String, record_type: String, record_line: String, value: String) -> Result<i32, Error> {

        let mut easy = Easy::new();
        easy.url("https://dnsapi.cn/Record.Create")?;




        let record_line = if record_line.chars().all(char::is_numeric) {
                ("record_line_id", record_line.as_str())
            } else {
                ("record_line", record_line.as_str())
            };

        let result: Value = ureq::post()
            .set("User-Agent", "cao, Johann Li <me@qinka.pro>")
            .send_form(&[
                ("login_token", self.key.as_str()),
                ("domain",      self.domain.as_str()),
                ("sub_domain",  sub_domain.as_str()),
                ("record_type", record_type.as_str()),
                record_line,
                ("value",       value.as_str()),
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
            ("login_token", self.key.as_str()),
            ("domain",      self.domain.as_str()),
        ];
        let offset_s = offset.map_or(String::from(""), |i| i.to_string());
        let length_s = length.map_or(String::from(""), |i| i.to_string());
        let subdom_s = sub_domain.clone().unwrap_or(String::from(""));

        if offset.is_some() {
            form.push(("offset", offset_s.as_str()));
        }
        if length.is_some() {
            form.push(("length", length_s.as_str()));
        }
        if sub_domain.is_some() {
            form.push(("sub_domain", subdom_s.as_str()));
        }

        let result: Value = ureq::post("https://dnsapi.cn/Record.List")
            .set("User-Agent", "cao, Johann Li <me@qinka.pro>")
            .send_form(form.as_slice())?
            .into_json()?;
        eprintln!("{:?}",result);

        if result["status"]["code"].is_string() && result["status"]["code"].as_str().unwrap().eq("1") {
            if let Value::Array(list) = &result["records"] {
                Ok(list.into_iter().map(|v| {
                    let id = if let Value::String(id) = &v["id"] {
                            id.parse::<i32>().unwrap_or(-1)
                        } else { -2 };
                    super::interface::Record{
                        id: id,
                        sub_domain: v["name"].to_string(),
                        value: v["value"].to_string(),
                        r_type: v["type"].to_string(),
                        r_line: v["line"].to_string(),
                    }}
                ).collect())
            } else {
                Err(Error::InterfaceError(format!("Failed to parse {:?}", result["status"]["code"])))
            }
        } else {
            Err(Error::ProviderError(result.to_string()))
        }
    }
    fn modify_record(&self, id: i32, sub_domain: Option<String>, r_type: String, r_line: String, value: String) -> Result<(), Error>{

        let id = id.to_string();

        let result: Value = ureq::post("https://dnsapi.cn/Record.Info")
            .set("User-Agent", "cao, Johann Li <me@qinka.pro>")
            .send_form(&[
                ("login_token", self.key.as_str()),
                ("domain", self.domain.as_str()),
                ("record_id", id.as_str()),
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
            ("login_token", self.key.as_str()),
            ("domain", self.domain.as_str()),
            ("record_id", id.as_str()),
            ("record_type", r_type.as_str()),
            ("value", value.as_str()),
        ];

        let subdom_s = sub_domain.clone().unwrap_or(String::from(""));

        if sub_domain.is_some() {
            form.push(("sub_domain", subdom_s.as_str()));
        }
        if r_line.chars().all(char::is_numeric) {
            form.push(("record_line_id", r_line.as_str()));
        } else {
            form.push(("record_line", r_line.as_str()));
        }

        let result: Value = ureq::post("https://dnsapi.cn/Record.Modify")
            .set("User-Agent", "cao, Johann Li <me@qinka.pro>")
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

        let result: Value = ureq::post("https://dnsapi.cn/Record.Remove")
            .set("User-Agent", "cao, Johann Li <me@qinka.pro>")
            .send_form(&[
                ("login_token", self.key.as_str()),
                ("domain", self.domain.as_str()),
                ("record_id", id.as_str()),
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