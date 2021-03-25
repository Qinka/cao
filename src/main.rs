mod args;
mod provider;
mod error;


use crate::args::{Args, Cmds};
use crate::provider::build_dns_provider;
use crate::error::Error;

fn fetch_key(file_name: String) -> Result<String, Error>  {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(file_name)?;
    let mut s = String::with_capacity(64);
    f.read_to_string(&mut s)?;
    Ok(String::from(s.trim()))
}

fn interface_ip(interface: String) -> Result<String, Error> {
    Ok(String::from("0.0.0.0"))
}

fn interface_or_value(interface: Option<String>, value: Option<String>) -> Result<String, Error> {
    if value.is_some() {
        Ok(value.unwrap())
    } else {
        let interface = interface.ok_or(Error::MissingRequiredArgument)?;
        interface_ip(interface)
    }
}

fn main() -> Result<(), Error> {
    let args = Args::args();

    match args {
        Err(err) => {
            eprintln!("{}", err);
        },
        Ok(param) => {
            let key = fetch_key(param.key)?;
            let provider = build_dns_provider(param.provider, key, param.domain)?;
            match param.cmd {
                Cmds::Add{sub_domain, record_type, record_line, value, interface} => {
                    let value = interface_or_value(interface, value)?;
                    let id = provider.add_record(sub_domain, record_type, record_line, value)?;
                    print!("{}", id);
                },
                Cmds::List{offset, length, sub_domain} => {
                    let records = provider.list_record(offset, length, sub_domain)?;
                    for record in records {
                        println!("{}", record);
                    }
                },
                Cmds::Modify{record_id, sub_domain, record_type, record_line, value, interface} => {
                    let value = interface_or_value(interface, value)?;
                    provider.modify_record(record_id, sub_domain, record_type, record_line, value)?;
                },
                Cmds::Delete{record_id} => {
                    provider.delete_record(record_id)?;
                },
                // _ => unimplemented!("Unimplemented option: {:?}", param.cmd),
            };
        }
    };

    Ok(())
}
