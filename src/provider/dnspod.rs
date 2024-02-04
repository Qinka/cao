use digest::{CtOutput, Digest};
use hmac::Hmac;
use hmac::Mac;
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde_json::{json, Value};
use sha2::Sha256;
use std::convert::TryInto;

use super::interface::*;
use crate::error::{Error, Result};

mod intra_common {

  pub const CAO_USER_AGENT: &str = concat!(
    "cao/",
    env!("CARGO_PKG_VERSION"),
    ", Johann Li <me@qinka.pro>"
  );

  // pub const CAO_FORM_TOKEN: &str = "login_token";
  pub const CAO_FORM_DOMAIN: &str = "Domain";
  pub const CAO_FORM_SDOMAIN: &str = "SubDomain";
  pub const CAO_FORM_RID: &str = "RecordId";
  pub const CAO_FORM_RTYPE: &str = "RecordType";
  pub const CAO_FORM_VALUE: &str = "Value";
  pub const CAO_FORM_RLINE: &str = "RecordLine";
  pub const CAO_FORM_RLINE_ID: &str = "RecordLineId";
  pub const CAO_FORM_OFFSET: &str = "Offset";
  pub const CAO_FORM_LENGTH: &str = "Limit";

  pub const DNSPOD_API: &str = "https://dnspod.tencentcloudapi.com";
  pub const DNSPOD_HOST: &str = "dnspod.tencentcloudapi.com";
  pub const DNSPOD_SERVICE: &str = "dnspod";
  pub const DNSPOD_REQUEST: &str = "tc3_request";
  pub const DNSPOD_ALGORITHM: &str = "TC3-HMAC-SHA256";
  pub const DNSPOD_API_VERSION: &str = "2021-03-23";
}

#[inline]
fn hmac_sha256_base(
  message: &[u8],
  key: &[u8],
) -> Result<CtOutput<Hmac<Sha256>>> {
  let mut mac = Hmac::<Sha256>::new_from_slice(key)?;
  mac.update(message);
  Ok(mac.finalize())
}

fn hmac_sha256(message: &[u8], key: &[u8]) -> Result<[u8; 32]> {
  let result = hmac_sha256_base(message, key)?;

  let code_bytes = result.into_bytes();
  let code_slice = code_bytes.as_slice();
  Ok(code_slice.try_into()?)
}

fn hmac_sha256_hex(message: &[u8], key: &[u8]) -> Result<String> {
  let result = hmac_sha256_base(message, key)?;
  let code_bytes = result.into_bytes();
  let code_slice = code_bytes.as_slice();
  Ok(hex::encode(code_slice))
}

fn hash_sha256_hex(data: &[u8]) -> Result<String> {
  // let mut hasher = sha2::Sha256::new();
  // hasher.update(data);
  Ok(hex::encode(Sha256::digest(data)))
}

fn record_parse(record: &Value) -> Option<Record> {
  let id = record.get("RecordId")?.as_u64()?;
  let sub_domain = String::from(record.get("Name")?.as_str()?);
  let value = String::from(record.get("Value")?.as_str()?);
  let r_type = String::from(record.get("Type")?.as_str()?);
  let r_line = String::from(record.get("Line")?.as_str()?);
  Some(Record {
    id,
    sub_domain,
    value,
    r_type,
    r_line,
  })
}

pub struct Provider {
  /// secret_id
  id: String,
  /// secret_key
  key: String,
  domain: String,
  client: reqwest::blocking::Client,
}

impl Provider {
  fn make_authorization(
    &self,
    action: &str,
    date: &str,
    timestamp: i64,
    payload: &str,
  ) -> Result<String> {
    //
    // step 1
    let canonical_headers = format!(
      "content-type:application/json; charset=utf-8\nhost:{}\nx-tc-action:{}\n",
      intra_common::DNSPOD_HOST,
      action.to_lowercase(),
    );
    let signed_headers = "content-type;host;x-tc-action";
    let hashed_request_payload = hash_sha256_hex(payload.as_bytes())?;
    let canonical_request = format!(
      "POST\n/\n\n{}\n{}\n{}",
      canonical_headers, signed_headers, hashed_request_payload
    );
    tracing::debug!("payload: {}", payload);
    tracing::debug!("canonical_request: {}", canonical_request);

    //
    // step 2
    let credential_scope = format!(
      "{}/{}/{}",
      &date,
      intra_common::DNSPOD_SERVICE,
      intra_common::DNSPOD_REQUEST
    );
    let hashed_canonical_request =
      hash_sha256_hex(canonical_request.as_bytes())?;
    let string_to_sign = format!(
      "{}\n{}\n{}\n{}",
      intra_common::DNSPOD_ALGORITHM,
      timestamp,
      credential_scope,
      hashed_canonical_request
    );

    // step 3
    let secret_date = hmac_sha256(
      date.as_bytes(),
      format!("TC3{}", self.key).as_str().as_bytes(),
    )?;
    let secret_service =
      hmac_sha256(intra_common::DNSPOD_SERVICE.as_bytes(), &secret_date)?;
    let secret_signing =
      hmac_sha256(intra_common::DNSPOD_REQUEST.as_bytes(), &secret_service)?;
    let signature =
      hmac_sha256_hex(string_to_sign.as_bytes(), &secret_signing)?;
    tracing::debug!("STRING_TO_SIGN: {}", string_to_sign);

    // step 4
    let authorization = format!(
      "{} Credential={}/{}, SignedHeaders={}, Signature={}",
      intra_common::DNSPOD_ALGORITHM,
      self.id,
      credential_scope,
      signed_headers,
      signature
    );

    Ok(authorization)
  }

  fn request<P>(&self, action: &str, payload: P) -> Result<Value>
  where
    P: Serialize,
  {
    // times
    let current_time = chrono::offset::Utc::now();
    let timestamp = current_time.timestamp();
    let date = current_time.format("%Y-%m-%d").to_string();

    // payload
    let payload_str = serde_json::to_string(&payload)?;
    // authorized
    let authorization =
      self.make_authorization(action, &date, timestamp, &payload_str)?;
    // headers
    let headers = {
      let mut headers = HeaderMap::new();
      headers.insert("Authorization", authorization.parse()?);
      headers
        .insert("Content-Type", "application/json; charset=utf-8".parse()?);
      headers.insert("Host", intra_common::DNSPOD_HOST.parse()?);
      headers.insert("X-TC-Action", action.parse()?);
      headers.insert("X-TC-Timestamp", timestamp.to_string().parse()?);
      headers.insert("X-TC-Version", intra_common::DNSPOD_API_VERSION.parse()?);
      headers.insert("User-Agent", intra_common::CAO_USER_AGENT.parse()?);
      headers
    };

    tracing::debug!("TIMESTAMP: {}", timestamp);
    tracing::debug!("KEY: {} {}", self.id, self.key);
    tracing::debug!("DOMAIN: {}", self.domain);
    tracing::debug!("URL: {:?}", intra_common::DNSPOD_API);
    tracing::debug!("HEADER: {:?}", &headers);
    tracing::debug!("BODY: {}", serde_json::to_string(&payload)?);

    let result = self
      .client
      .post(intra_common::DNSPOD_API)
      .headers(headers)
      .body(serde_json::to_string(&payload)?)
      .send()?
      .json()?;

    Ok(result)
  }
}

impl DnsProviderBuild for Provider {
  fn build_provider(token: String, domain: String) -> Result<Self> {
    let client = reqwest::blocking::Client::new();
    let mut split = token.split(',');
    let id = split.next().unwrap().to_string();
    let key = split.next().unwrap().to_string();
    Ok(Provider {
      id,
      key,
      domain,
      client,
    })
  }
}

impl DnsProvider for Provider {
  fn add_record(
    &self,
    sub_domain: &str,
    record_type: &str,
    record_line: &str,
    value: &str,
  ) -> Result<u64> {
    //
    // Build request
    // payload
    let (record_line_key, record_line_value) =
      if record_line.chars().all(char::is_numeric) {
        (
          intra_common::CAO_FORM_RLINE_ID,
          json!(record_line.parse::<i32>()?),
        )
      } else {
        (intra_common::CAO_FORM_RLINE, json!(record_line))
      };
    let payload = json!({
        intra_common::CAO_FORM_DOMAIN: &self.domain,
        intra_common::CAO_FORM_SDOMAIN: sub_domain,
        intra_common::CAO_FORM_RTYPE: record_type,
        intra_common::CAO_FORM_VALUE: value,
        record_line_key: record_line_value,
    });

    let result: Value = self.request("CreateRecord", payload)?;

    //
    // Result process
    #[cfg(debug_assertions)]
    eprintln!("{}", serde_json::to_string_pretty(&result)?);

    // response processing
    if let Some(response) = result.get("Response") {
      if let Some(id) = response.get("RecordId") {
        id.as_u64().ok_or(Error::http_failed(format!(
          "Record id parse failed {:?}",
          &result
        )))
      } else {
        Err(Error::http_failed(format!("Request failed {:?}", &result)))
      }
    } else {
      Err(Error::http_failed(format!(
        "Failed to parse result {:?}",
        &result
      )))
    }
  }

  fn list_record(
    &self,
    offset: Option<i32>,
    length: Option<i32>,
    sub_domain: Option<&str>,
  ) -> Result<Vec<Record>> {
    //
    // payload
    let payload = {
      let mut data = json!({
        intra_common::CAO_FORM_DOMAIN: &self.domain,
      });
      if let Some(offset) = offset {
        data[intra_common::CAO_FORM_OFFSET] = json!(offset);
      }
      if let Some(length) = length {
        data[intra_common::CAO_FORM_LENGTH] = json!(length);
      }
      if let Some(sub_domain) = sub_domain {
        data[intra_common::CAO_FORM_SDOMAIN] = json!(sub_domain);
      }
      data
    };

    let result: Value = self.request("DescribeRecordList", payload)?;

    //
    // Result process
    #[cfg(debug_assertions)]
    eprintln!("{}", serde_json::to_string_pretty(&result)?);

    // response processing
    if let Some(response) = result.get("Response") {
      if let Some(Value::Array(list)) = response.get("RecordList") {
        Ok(list.iter().filter_map(record_parse).collect())
      } else {
        Err(Error::http_failed(format!("Request failed {:?}", &result)))
      }
    } else {
      Err(Error::http_failed(format!(
        "Failed to parse result {:?}",
        &result
      )))
    }
  }

  fn modify_record(
    &self,
    record_id: u64,
    sub_domain: Option<&str>,
    record_type: &str,
    record_line: &str,
    value: &str,
  ) -> Result<u64> {
    //
    // Payload
    let (record_line_key, record_line_value) =
      if record_line.chars().all(char::is_numeric) {
        (
          intra_common::CAO_FORM_RLINE_ID,
          json!(record_line.parse::<i32>()?),
        )
      } else {
        (intra_common::CAO_FORM_RLINE, json!(record_line))
      };
    let payload: Value = json!({
      intra_common::CAO_FORM_RID: record_id,
      intra_common::CAO_FORM_DOMAIN: &self.domain,
      intra_common::CAO_FORM_SDOMAIN: sub_domain,
      intra_common::CAO_FORM_RTYPE: record_type,
      intra_common::CAO_FORM_VALUE: value,
      record_line_key: record_line_value,
    });

    let result: Value = self.request("ModifyRecord", payload)?;

    //
    // Result process
    #[cfg(debug_assertions)]
    eprintln!("{}", serde_json::to_string_pretty(&result)?);

    // response processing
    if let Some(response) = result.get("Response") {
      if let Some(id) = response.get("RecordId") {
        id.as_u64().ok_or(Error::http_failed(format!(
          "Record id parse failed {:?}",
          &result
        )))
      } else {
        Err(Error::http_failed(format!("Request failed {:?}", &result)))
      }
    } else {
      Err(Error::http_failed(format!(
        "Failed to parse result {:?}",
        &result
      )))
    }
  }

  fn delete_record(&self, id: u64) -> Result<()> {
    let payload = json!({
        intra_common::CAO_FORM_DOMAIN: &self.domain,
        intra_common::CAO_FORM_RID: id,
    });

    let result: Value = self.request("DeleteRecord", payload)?;

    //
    // Result process
    #[cfg(debug_assertions)]
    eprintln!("{}", serde_json::to_string_pretty(&result)?);

    // response processing
    if result.get("Response").is_some() {
      Ok(())
    } else {
      Err(Error::http_failed(format!(
        "Failed to parse result {:?}",
        &result
      )))
    }
  }
}

#[cfg(test)]
mod test {
  use super::Provider;
  use crate::provider::interface::DnsProviderBuild;
  use crate::provider::interface::{DnsProvider, Record};
  use trust_dns_resolver::config::*;
  use trust_dns_resolver::Resolver;

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

    let type_a = String::from("A");
    let r_line_id = String::from("0");
    let value_a = String::from("1.2.3.4");
    let value_b = String::from("2.3.4.5");

    let mut resolver_config = ResolverConfig::new();
    resolver_config.add_name_server(NameServerConfig {
      socket_addr: "119.29.29.29:53".parse().unwrap(),
      bind_addr: None,
      protocol: Protocol::Udp,
      tls_dns_name: None,
      trust_negative_responses: true,
    });
    let mut resolver_opt = ResolverOpts::default();
    resolver_opt.attempts = 10;
    resolver_opt.negative_max_ttl = Some(time::Duration::from_micros(1000));
    let resolver =
      Resolver::new(ResolverConfig::default(), resolver_opt).unwrap();

    let provider = Provider::build_provider(
      var("DNSPOD_TEST_TOKEN")
        .expect("Need environment variable: DNSPOD_TEST_TOKEN"),
      domain.clone(),
    )
    .unwrap();

    sleep(sleep_time);

    let response = resolver.lookup_ip(full_domain.as_str());
    println!("1. {:?}", response);
    if response.is_ok() {
      // let address = response.iter().next().expect("no addresses returned!");
      panic!("Should clean test domain before the test! {:?}", response);
    }

    let id = provider
      .add_record(&sub_domain, &type_a, &r_line_id, &value_a)
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
      Record {
        id: id,
        sub_domain: sub_domain.clone(),
        value: String::from("1.2.3.4"),
        r_type: String::from("A"),
        r_line: String::from("默认"),
      }
    );

    sleep(sleep_time);

    provider
      .modify_record(id, Some(&sub_domain), &type_a, &r_line_id, &value_b)
      .unwrap();

    sleep(sleep_time);

    let response = resolver.lookup_ip(full_domain.as_str()).unwrap();
    let address = response.iter().next().expect("no addresses returned!");
    assert_eq!(address, IpAddr::V4(Ipv4Addr::new(2, 3, 4, 5)));

    provider.delete_record(id).unwrap();

    sleep(sleep_time);
    sleep(sleep_time);

    let response = resolver.lookup_ip(full_domain.as_str());
    if response.is_ok() {
      panic!("Record should be deleted the test!");
    }
  }
}
