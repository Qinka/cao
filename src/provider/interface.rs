
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub struct Record {
    pub id: i32,
    pub sub_domain: String,
    pub value: String,
    pub r_type: String,
    pub r_line: String,
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "id: {}, name: {}, value: {}, type: {}, line: {}",
            self.id, self.sub_domain, self.value, self.r_type, self.r_line)
    }
}

pub trait DnsProvider {
    fn build_provider(key: String, domain: String) -> Result<Box<dyn DnsProvider>, Error>
        where Self:Sized;
    fn add_record(&self, sub_domain: String, record_type: String, record_line: String, value: String) -> Result<i32, Error>;
    fn list_record(&self, offset: Option<i32>, length: Option<i32>, sub_domain: Option<String>) -> Result<Vec<Record>, Error>;
    fn modify_record(&self, id: i32, sub_domain: Option<String>, r_type: String, r_line: String, value: String) -> Result<(), Error>;
    fn delete_record(&self, id: i32) -> Result<(), Error>;
}
