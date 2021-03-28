



use ureq;
use serde_json::Value;
use crate::error::Error;


const CAO_USER_AGENT:         &str = concat!("cao/", env!("CARGO_PKG_VERSION"),", Johann Li <me@qinka.pro>") ;
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


pub fn add_record(
    domain: &String,
    key:    &String,
    sub_domain: &String,
    record_type: &String,
    record_line: &String,
    value:       &String
) -> Result<Value, Error> {

    let record_line: (&str, &str) =
        if record_line.chars().all(char::is_numeric) {
            (CAO_FORM_RLINE_ID, record_line)
        } else {
            (CAO_FORM_RLINE,    record_line)
        };

    Ok(ureq::post(DNSPOD_ADD_RECORD)
        .set(HTTP_HEADER_USER_AGENT, CAO_USER_AGENT)
        .send_form(&[
            (CAO_FORM_TOKEN,   key),
            (CAO_FORM_DOMAIN,  domain),
            (CAO_FORM_SDOMAIN, sub_domain),
            (CAO_FORM_RTYPE,   record_type),
            record_line,
            (CAO_FORM_VALUE,   value),
        ])?
        .into_json()?
    )
}

pub fn list_record(
    domain:     &String,
    key:        &String,
    offset:     Option<&String>,
    length:     Option<&String>,
    sub_domain: Option<&String>,
) -> Result<Value, Error> {

    let mut form = vec![
        (CAO_FORM_TOKEN,  key.as_str()),
        (CAO_FORM_DOMAIN, domain.as_str()),
    ];

    if let Some(offset) = offset {
        form.push((CAO_FORM_OFFSET, offset));
    }
    if let Some(length) = length {
        form.push((CAO_FORM_LENGTH, length));
    }
    if let Some(sub_domain) = sub_domain {
        form.push((CAO_FORM_SDOMAIN, sub_domain));
    }

    Ok(ureq::post(DNSPOD_LIST_RECORDS)
        .set(HTTP_HEADER_USER_AGENT, CAO_USER_AGENT)
        .send_form(form.as_slice())?
        .into_json()?
    )
}

pub fn info_record(
    domain: &String,
    key:    &String,
    id:     &String,
) -> Result<Value, Error> {
    Ok(ureq::post(DNSPOD_INFO_RECORD)
        .set(HTTP_HEADER_USER_AGENT, CAO_USER_AGENT)
        .send_form(&[
            (CAO_FORM_DOMAIN,  domain),
            (CAO_FORM_TOKEN, key),
            (CAO_FORM_RID,    id),
        ])?
        .into_json()?
    )
}

pub fn modify_record(
    domain:     &String,
    key:        &String,
    id:         &String,
    sub_domain: Option<&String>,
    r_type:     &String,
    r_line:     &String,
    value:      &String,
) -> Result<Value, Error>{
    let mut form: Vec<(&str, &str)> = vec![
        (CAO_FORM_TOKEN,  key),
        (CAO_FORM_DOMAIN, domain),
        (CAO_FORM_RID,    id),
        (CAO_FORM_RTYPE,  r_type),
        (CAO_FORM_VALUE,  value),
    ];

    if let Some(sub_domain) = sub_domain {
        form.push((CAO_FORM_SDOMAIN, sub_domain));
    }
    if r_line.chars().all(char::is_numeric) {
        form.push((CAO_FORM_RLINE_ID, r_line));
    } else {
        form.push((CAO_FORM_RLINE,    r_line));
    }

    Ok(ureq::post(DNSPOD_MODIFY_RECORD)
        .set(HTTP_HEADER_USER_AGENT, CAO_USER_AGENT)
        .send_form(form.as_slice())?
        .into_json()?
    )
}
pub fn delete_record(
    domain: &String,
    key:    &String,
    id:     &String,
) -> Result<Value, Error> {

    Ok(ureq::post(DNSPOD_DELETE_RECORD)
        .set(HTTP_HEADER_USER_AGENT, CAO_USER_AGENT)
        .send_form(&[
            (CAO_FORM_TOKEN,  key),
            (CAO_FORM_DOMAIN, domain),
            (CAO_FORM_RID,    id),
        ])?
        .into_json()?
    )
}
