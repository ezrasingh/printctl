use crate::prelude::*;

mod api;
mod config;
mod devices;
mod state;

pub use config::ServerConfig;

use crate::discovery;

#[tokio::main]
pub async fn run(
    server_config: config::ServerConfig,
    discovery_config: discovery::DiscoveryConfig,
) -> Result<()> {
    use tonic::transport::Server;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::info!("Stating discovery...");
    let node = discovery::Node::new(discovery_config).start_discovery();
    tracing::info!("Advertising self as: {}", node.name());

    let state = state::ServerState::new(node);

    let addr = server_config.grpc_socket();
    tracing::info!("gRPC server listening on {}", addr);
    Server::builder()
        .add_service(api::grpc::PrintctlServer::new(state.as_arc_mutex()))
        .serve(addr)
        .await
        .expect("could not start gRPC server");

    Ok(())
}
