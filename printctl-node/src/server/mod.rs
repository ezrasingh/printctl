mod config;
mod devices;
pub use config::ServerConfig;

#[path = "./"]
pub mod grpc {
    #[path = "printctl.rs"]
    pub mod v0;

    pub use v0::printctl_server::Printctl;
    pub use v0::printctl_server::PrintctlServer;
}

use crate::prelude::*;

use simple_mdns::async_discovery::ServiceDiscovery;
use tonic::transport::Server;

use crate::discovery;

#[tonic::async_trait]
impl grpc::Printctl for discovery::Node<ServiceDiscovery> {}

#[tokio::main]
pub async fn run(
    server_config: ServerConfig,
    discovery_config: discovery::DiscoveryConfig,
) -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::info!("Stating discovery...");
    let node = discovery::Node::new(discovery_config).start_discovery();
    tracing::info!("Advertising self as: {}", node.name());

    let addr = server_config.grpc_socket();
    tracing::info!("gRPC server listening on {}", addr);
    Server::builder()
        .add_service(grpc::PrintctlServer::new(node))
        .serve(addr)
        .await
        .expect("could not start gRPC server");

    Ok(())
}
