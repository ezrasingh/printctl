mod config;
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
pub async fn run(config: ServerConfig) -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::info!(message = "Stating discovery...");
    let node_name = config.node_name();
    let node = discovery::Node::new(node_name, &None)?.start_discovery();
    tracing::info!(message = "Advertising self as: ", %node_name);

    let addr = config.grpc_socket();
    tracing::info!(message = "gRPC server listening on {}", %addr);
    Server::builder()
        .add_service(grpc::PrintctlServer::new(node))
        .serve(addr)
        .await
        .expect("could not start gRPC server");

    Ok(())
}
