use std::{io, fmt};
use std::str::FromStr;
use std::path::PathBuf;
use std::net::{IpAddr, ToSocketAddrs};

/// An enum corresponding to the address to serve on
#[derive(Debug, Clone, PartialEq)]
pub enum Address {
    /// The hostname to serve over TCP.
    Hostname(String),
    /// The IP address to serve over TCP.
    Ip(IpAddr),
    /// The path to the unix domain socket.
    Unix(PathBuf),
}

impl Address {
    crate const UNIX_PREFIX: &'static str = "unix:";

    crate fn is_unix(&self) -> bool {
        match self {
            Address::Unix(..) => true,
            _ => false
        }
    }
}

impl FromStr for Address {
    type Err = io::Error;

    fn from_str(string: &str) -> io::Result<Self> {
        #[cfg(unix)]
        {
            if string.starts_with(Address::UNIX_PREFIX) {
                let address = &string[Address::UNIX_PREFIX.len()..];
                return Ok(Address::Unix(address.into()));
            }
        }

        if (string, 0).to_socket_addrs()?.next().is_some() {
            if let Ok(ip) = IpAddr::from_str(string) {
                return Ok(Address::Ip(ip));
            } else {
                return Ok(Address::Hostname(string.into()));
            }
        }

        Err(io::Error::new(io::ErrorKind::Other, "failed to resolve TCP address"))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Address::Hostname(name) => name.fmt(f),
            Address::Ip(addr) => addr.fmt(f),
            Address::Unix(path) => write!(f, "{}{}", Address::UNIX_PREFIX, path.display()),
        }
    }
}
