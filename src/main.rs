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
    use if_addrs::{get_if_addrs, IfAddr};

    let opts: Vec<&str> = interface.split(',').map(|s| s.trim()).collect();
    let name = opts[0].clone();

    let result = get_if_addrs()?;

    eprintln!("{:?}", result);

    let f: Vec<_> = result
        .into_iter()
        .filter_map(|r| {
            if r.name.eq(name) && !r.is_loopback() {
                Some(r.addr)
            } else {
                None
            }
        }).collect();

    if opts.len() >= 2 && opts[1].ne("4") && opts[1].ne("6") {
        return Err(Error::InterfaceFilterError(format!("Unknown filter {}, should be 4 or 6", opts[1])));
    }

    let f: Vec<_> = if opts.len() >= 2 {
        if opts[1].eq("4") {
            f.into_iter().filter(|ip| {
                if let IfAddr::V4(_) = ip {
                    true
                } else {
                    false
                }
            }).collect()
        } else if opts[1].eq("6") {
            f.into_iter().filter(|ip| {
                if let IfAddr::V6(_) = ip {
                    true
                } else {
                    false
                }
            }).collect()
        } else {
            f
        }
    } else {
        f
    };

    let f: Vec<_> = f.into_iter().map(|ip| {
        match ip {
            IfAddr::V4(ip) => format!("{}", ip.ip),
            IfAddr::V6(ip) => format!("{}", ip.ip),
        }
    }).collect();

    let f: Vec<_> = if opts.len() >= 3 {
            f.into_iter()
                .filter(|s| s.starts_with(opts[2]))
                .collect()
        } else {
            f
        };

    if f.len() > 0 {
        Ok(f[0].clone())
    } else {
        Err(Error::InterfaceFilterError(format!("No such interface: {}", interface)))
    }

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
                    eprintln!("value: {}", value);
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
