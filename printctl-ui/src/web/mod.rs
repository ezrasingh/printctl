use std::net::IpAddr;

use crate::prelude::*;

pub fn start(http_addr: IpAddr, http_port: u16) -> Result<()> {
    println!("Starting web UI at http://{}:{}/", http_addr, http_port);
    Ok(())
}
