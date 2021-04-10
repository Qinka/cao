

use serde_json::Value;
use crate::error::Error;

use super::intra_common::*;

pub fn add_record(
    domain:      &str,
    key:         &str,
    sub_domain:  &str,
    record_type: &str,
    record_line: &str,
    value:       &str,
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
    domain:     &str,
    key:        &str,
    offset:     Option<&str>,
    length:     Option<&str>,
    sub_domain: Option<&str>,
) -> Result<Value, Error> {

    let mut form = vec![
        (CAO_FORM_TOKEN,  key),
        (CAO_FORM_DOMAIN, domain),
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
    domain: &str,
    key:    &str,
    id:     &str,
) -> Result<Value, Error> {
    Ok(ureq::post(DNSPOD_INFO_RECORD)
        .set(HTTP_HEADER_USER_AGENT, CAO_USER_AGENT)
        .send_form(&[
            (CAO_FORM_DOMAIN,  domain),
            (CAO_FORM_TOKEN,   key),
            (CAO_FORM_RID,     id),
        ])?
        .into_json()?
    )
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
    domain: &str,
    key:    &str,
    id:     &str,
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
