use crate::error::Error;
use if_addrs::{get_if_addrs, IfAddr, Interface};

fn interface_ip(interface: String) -> Result<String, Error> {
  let result = get_if_addrs()?;

  let opts: Vec<&str> = interface.split(',').map(|s| s.trim()).collect();
  let name = opts[0];

  let filter_name = |i: Interface| {
    if i.name.eq(name) && !i.is_loopback() {
      Some(i.addr)
    } else {
      None
    }
  };

  let filter_ip_type = if opts.len() >= 2 {
    if opts[1].eq("4") {
      |ip: &IfAddr| matches!(ip, IfAddr::V4(_))
    } else if opts[1].eq("6") {
      |ip: &IfAddr| matches!(ip, IfAddr::V6(_))
    } else {
      |_: &IfAddr| true
    }
  } else {
    |_: &IfAddr| true
  };

  let mut list = result
    .into_iter()
    .filter_map(filter_name)
    .filter(filter_ip_type)
    .map(|ip| match ip {
      IfAddr::V4(ip) => format!("{}", ip.ip),
      IfAddr::V6(ip) => format!("{}", ip.ip),
    })
    .filter(|s| opts.len() < 3 || s.starts_with(opts[2]));

  let nth = if opts.len() >= 4 {
    let n = opts[3].parse().unwrap_or(0);
    list.nth(n)
  } else {
    list.next()
  };
  nth.ok_or_else(|| Error::Reason(format!("No such interface: {}", interface)))
}

pub fn interface_or_value(
  interface: Option<String>,
  value: Option<String>,
) -> Result<String, Error> {
  if let Some(value) = value {
    Ok(value)
  } else {
    let interface =
      interface.ok_or(Error::Reason(String::from("Missing argument")))?;
    interface_ip(interface)
  }
}

pub fn interface_list(
  interface: Option<String>,
) -> Result<Vec<(String, String)>, Error> {
  if let Some(interface) = interface {
    let s = interface_ip(interface.clone())?;
    Ok(vec![(interface, s)])
  } else {
    Ok(
      get_if_addrs()?
        .into_iter()
        .map(|i| (i.name, format!("{:?}", i.addr)))
        .collect(),
    )
  }
}
