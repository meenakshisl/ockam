#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[macro_use]
extern crate tracing;

use ockam::{route, Address, Context, Message, Result, Routed, Worker};

/// ## How will Connor’s Ockam Command ensure message flow is:
///
///     transport < - > secure channel < - > app workers?
///
///
/// ## Suborbital use case
///
/// ### Connor's Ockam Command does:
///
///   #1 create an identity for their control plane
///
///   #2 create an identity for their edge plane
///
///   #3 provision their edge plane with the identity of their control plane
///
///   #4 provision their control plane with the identity of their edge plane
///
///   #5 start a httpd on their control plane with:
///
///          bind_addr: 127.0.0.1:4001
///
///   #6 create an outlet on ockamcloud to their control plane with:
///
///         cloud_addr: suborbital.node.ockam.network:4000
///              alias: mrinal_cp
///      outlet_target: host.docker.internal:4001 (httpd started in #1)
///
///   #7 create an inlet on ockamcloud from their edge plane with:
///
///         cloud_addr: suborbital.node.ockam.network:4000
///              alias: mrinal_cp
///      inlet_address: 127.0.0.1:4002
///
///   #8 send a http request from their edge plane to the httpd on their control plane with:
///
///           protocol: http
///          authority: 127.0.0.1:4002
///             method: GET
///               path: /
///
/// ### Connor’s Ockam Command needs:
///
///   - An address for Ockam Cloud
///   - AuthN to establish their identity with Ockam Cloud
///   - AuthZ to send messages to Ockam Cloud
///   - AuthZ to create an outlet on Ockam Cloud
///   - AuthZ to create an inlet on Ockam Cloud
///
/// ### Connor’s Outlet needs:
///
///   - AuthN to establish their identity with Connor’s Inlet
///   - AuthZ to send messages to Connor’s Inlet
///
/// ### Connor’s Inlet needs:
///
///   - AuthN to establish their identity with Connor’s Outlet
///   - AuthZ to send messages to Connor’s Outlet
///
/// ### Connor’s httpd needs:
///
///   - AuthZ to receive requests from Connor’s Outlet
///
/// ### Connor’s curl needs:
///
///   - AuthZ to connect to Connor’s Inlet
///
/// ### Connor’s Workers need:
///
///   - TcpTransport needs AuthZ to send messages to Worker:
///       connectivity by parenthood: Worker -> TcpTransport(Worker)
///
///   - SecureChannel needs AuthZ to send messages to TcpTransport:
///       connectivity by endowment: Worker -> SecureChannel(TcpTransport)

// - ockam::node --------------------------------------------------------------

#[ockam::node]
async fn main(ctx: Context) -> ockam::Result<()> {
    info!("oh hai!");

    Ok(())
}

// - attribute based access control (abac) ------------------------------------

use ockam_capability::abac::{mem::Memory as AbacBackend, Abac};

fn default_abac() -> AbacBackend {
    AbacBackend::new()
}
