mod args;
mod provider;
mod error;
mod interface;

#[cfg(all(feature = "ureq", feature = "curl"))]
compile_error!("Features `ureq' and `curl' cannot be enabled at the same time.");

use crate::args::{Args, RecordCmds};
use crate::provider::build_dns_provider;
use crate::error::Error;
use crate::interface::{interface_list, interface_or_value};

fn fetch_key(file_name: String) -> Result<String, Error>  {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(file_name)?;
    let mut s = String::with_capacity(64);
    f.read_to_string(&mut s)?;
    Ok(String::from(s.trim()))
}

fn main() -> Result<(), Error> {

    tracing_subscriber::fmt::init();
    let args = Args::get_args();

    match args {
        Err(err) => {
            eprintln!("{}", err);
        },
        Ok(param) => { match param {
            Args::Record{provider, key, domain, cmd} => {
                let key = fetch_key(key)?;
                let provider = build_dns_provider(&provider, key, domain)?;
                match cmd {
                    RecordCmds::Add{
                        sub_domain, record_type, record_line, value, interface
                    } => {
                        let value = interface_or_value(interface, value)?;
                        let id = provider.add_record(
                            &sub_domain,
                            &record_type,
                            &record_line,
                            &value,
                        )?;
                        print!("{}", id);
                    },
                    RecordCmds::List{offset, length, sub_domain} => {
                        let records = provider.list_record(
                            offset,
                            length,
                            sub_domain.as_deref(),
                        )?;
                        for record in records {
                            println!("{}", record);
                        }
                    },
                    RecordCmds::Modify{
                        record_id, sub_domain, record_type, record_line, value, interface
                    } => {
                        let value = interface_or_value(interface, value)?;
                        provider.modify_record(
                            record_id,
                            sub_domain.as_deref(),
                            &record_type,
                            &record_line,
                            &value
                        )?;
                    },
                    RecordCmds::Delete{record_id} => {
                        provider.delete_record(record_id)?;
                    },
                    // _ => unimplemented!("Unimplemented option: {:?}", param.cmd),
                }
            },
            Args::Interface{interface} => {
                for (n, i) in interface_list(interface)? {
                    eprintln!("{}: {}", n, i);
                }

            }
            // _ => unimplemented!("Unimplemented option: {:?}", param.cmd),
        };}
    };

    Ok(())
}

// use curl::easy::Easy;

// // Capture output into a local `Vec`.
// fn main() {
//     let mut dst = Vec::new();
//     let mut easy = Easy::new();
//     easy.url("https://www.rust-lang.org/").unwrap();

//     {
//         let mut transfer = easy.transfer();
//         transfer.write_function(|data| {
//             dst.extend_from_slice(data);
//             Ok(data.len())
//         }).unwrap();
//         transfer.perform().unwrap();
//     }
//     let dst_s = String::from_utf8(dst).unwrap();
//     println!("{}", dst_s);

//     let mut dst = Vec::new();
//     easy.url("http://baidu.com/").unwrap();

//     {
//         let mut transfer = easy.transfer();
//         transfer.write_function(|data| {
//             dst.extend_from_slice(data);
//             Ok(data.len())
//         }).unwrap();
//         transfer.perform().unwrap();
//     }
//     let dst_s = String::from_utf8(dst).unwrap();
//     println!("{}", dst_s);
// }
