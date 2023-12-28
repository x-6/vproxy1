use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Network parse error")]
    NetworkParseError(#[from] cidr::errors::NetworkParseError),
    #[error("Address parse error")]
    AddressParseError(#[from] std::net::AddrParseError),
}