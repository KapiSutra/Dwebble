use anyhow::Context;

pub fn free_local_ipv4_port() -> anyhow::Result<u16> {
    let port = port_check::free_local_ipv4_port().context("failed to free_local_ipv4_port");
    Ok(port?)
}
