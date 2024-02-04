
use crate::error::Error;

#[derive(Debug, PartialEq)]
pub struct Record {
    pub id:         u64,
    pub sub_domain: String,
    pub value:      String,
    pub r_type:     String,
    pub r_line:     String,
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "id: {}, name: {}, value: {}, type: {}, line: {}",
            self.id, self.sub_domain, self.value, self.r_type, self.r_line)
    }
}

pub trait DnsProviderBuild: Sized {
    fn build_provider(
        key:    String,
        domain: String,
    ) -> Result<Self, Error>;
}

pub trait DnsProvider {

    fn add_record(
        &self,
        sub_domain:  &str,
        record_type: &str,
        record_line: &str,
        value:       &str,
    ) -> Result<u64, Error>;

    fn list_record(
        &self,
        offset:     Option<i32>,
        length:     Option<i32>,
        sub_domain: Option<&str>,
    ) -> Result<Vec<Record>, Error>;

    fn modify_record(
        &self, id: u64,
        sub_domain: Option<&str>,
        r_type: &str,
        r_line: &str,
        value:  &str,
    ) -> Result<u64, Error>;

    fn delete_record(
        &self,
        id: u64,
    ) -> Result<(), Error>;
}
