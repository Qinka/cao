
use curl::easy::{Easy, Form};
use serde_json::Value;
use crate::error::Error;

use super::intra_common::*;

fn fetch_and_into_json(handle: &mut Easy) -> Result<Value, Error> {

    handle.useragent(CAO_USER_AGENT)?;

    #[cfg(feature = "curl_invailed_cert")]
    handle.ssl_verify_peer(false)?;

    let mut rt_u8  = Vec::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            rt_u8.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }
    let rt_str = String::from_utf8(rt_u8)?;

    #[cfg(debug_assertions)]
    eprintln!("{:?}", rt_str);

    Ok(serde_json::de::from_str(&rt_str)?)
}

fn new_form() -> Result<Form, Error> {
    let mut form   = Form::new();
    form.part(CAO_FORM_FORMAT).contents(FORMAT_JSON.as_bytes()).add()?;
    Ok(form)
}

pub fn add_record(
    domain:      &str,
    key:         &str,
    sub_domain:  &str,
    record_type: &str,
    record_line: &str,
    value:       &str,
) -> Result<Value, Error> {

    let mut form   = new_form()?;
    let mut handle = Easy::new();

    form.part(CAO_FORM_TOKEN).contents(key.as_bytes()).add()?;
    form.part(CAO_FORM_DOMAIN).contents(domain.as_bytes()).add()?;
    form.part(CAO_FORM_SDOMAIN).contents(sub_domain.as_bytes()).add()?;
    form.part(CAO_FORM_RTYPE).contents(record_type.as_bytes()).add()?;
    form.part(CAO_FORM_VALUE).contents(value.as_bytes()).add()?;
    if record_line.chars().all(char::is_numeric) {
        form.part(CAO_FORM_RLINE_ID).contents(record_line.as_bytes()).add()?;
    } else {
        form.part(CAO_FORM_RLINE).contents(record_line.as_bytes()).add()?;
    }

    handle.url(DNSPOD_ADD_RECORD)?;
    handle.httppost(form)?;

    fetch_and_into_json(&mut handle)
}

pub fn list_record(
    domain:     &str,
    key:        &str,
    offset:     Option<&str>,
    length:     Option<&str>,
    sub_domain: Option<&str>,
) -> Result<Value, Error> {

    let mut form   = new_form()?;
    let mut handle = Easy::new();

    form.part(CAO_FORM_TOKEN).contents(key.as_bytes()).add()?;
    form.part(CAO_FORM_DOMAIN).contents(domain.as_bytes()).add()?;
    if let Some(offset) = offset {
        form.part(CAO_FORM_OFFSET).contents(offset.as_bytes()).add()?;
    }
    if let Some(length) = length {
        form.part(CAO_FORM_LENGTH).contents(length.as_bytes()).add()?;
    }
    if let Some(sub_domain) = sub_domain {
        form.part(CAO_FORM_SDOMAIN).contents(sub_domain.as_bytes()).add()?;
    }

    handle.url(DNSPOD_LIST_RECORDS)?;
    handle.httppost(form)?;

    fetch_and_into_json(&mut handle)
}

pub fn info_record(
    domain: &str,
    key:    &str,
    id:     &str,
) -> Result<Value, Error> {

    let mut form   = new_form()?;
    let mut handle = Easy::new();

    form.part(CAO_FORM_TOKEN).contents(key.as_bytes()).add()?;
    form.part(CAO_FORM_DOMAIN).contents(domain.as_bytes()).add()?;
    form.part(CAO_FORM_RID).contents(id.as_bytes()).add()?;

    handle.url(DNSPOD_INFO_RECORD)?;
    handle.httppost(form)?;

    fetch_and_into_json(&mut handle)
}

pub fn modify_record(
    domain:            &str,
    key:               &str,
    id:                &str,
    sub_domain: Option<&str>,
    r_type:            &str,
    r_line:            &str,
    value:             &str,
) -> Result<Value, Error>{

    let mut form   = new_form()?;
    let mut handle = Easy::new();

    form.part(CAO_FORM_TOKEN).contents(key.as_bytes()).add()?;
    form.part(CAO_FORM_DOMAIN).contents(domain.as_bytes()).add()?;
    form.part(CAO_FORM_RID).contents(id.as_bytes()).add()?;
    form.part(CAO_FORM_RTYPE).contents(r_type.as_bytes()).add()?;
    form.part(CAO_FORM_VALUE).contents(value.as_bytes()).add()?;

    if let Some(sub_domain) = sub_domain {
        form.part(CAO_FORM_SDOMAIN).contents(sub_domain.as_bytes()).add()?;
    }
    if r_line.chars().all(char::is_numeric) {
        form.part(CAO_FORM_RLINE_ID).contents(r_line.as_bytes()).add()?;
    } else {
        form.part(CAO_FORM_RLINE).contents(r_line.as_bytes()).add()?;
    }

    handle.url(DNSPOD_MODIFY_RECORD)?;
    handle.httppost(form)?;

    fetch_and_into_json(&mut handle)
}
pub fn delete_record(
    domain: &str,
    key:    &str,
    id:     &str,
) -> Result<Value, Error> {

    let mut form   = new_form()?;
    let mut handle = Easy::new();

    form.part(CAO_FORM_TOKEN).contents(key.as_bytes()).add()?;
    form.part(CAO_FORM_DOMAIN).contents(domain.as_bytes()).add()?;
    form.part(CAO_FORM_RID).contents(id.as_bytes()).add()?;

    handle.url(DNSPOD_DELETE_RECORD)?;
    handle.httppost(form)?;

    fetch_and_into_json(&mut handle)
}
