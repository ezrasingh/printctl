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

use crate::mesh;

impl grpc::Printctl for mesh::Node<ServiceDiscovery> {}

#[tokio::main]
pub async fn run(config: ServerConfig) -> Result<()> {
    let addr = "[::1]:50051".parse().unwrap();

    println!("Advertising machine as: {}", config.node_name());
    let node = mesh::Node::new(config.node_name(), &None)?.start_discovery();

    println!("PrintctlServer listening on {}", addr);

    Server::builder()
        .add_service(grpc::PrintctlServer::new(node))
        .serve(addr)
        .await
        .expect("could not start gRPC server");

    Ok(())
}
