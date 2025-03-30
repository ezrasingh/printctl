use tonic::{Request, Response};

use super::state::{ServerState, SharedState};
use crate::discovery::ServiceDiscovery;

#[path = "./"]
pub mod grpc {
    #[path = "printctl.rs"]
    pub mod v0;

    pub use v0::printctl_server::Printctl;
    pub use v0::printctl_server::PrintctlServer;
}

type TonicResult<T> = std::result::Result<Response<T>, tonic::Status>;

#[tonic::async_trait]
impl grpc::Printctl for SharedState<ServerState<ServiceDiscovery>> {
    async fn available_devices(
        &self,
        request: Request<grpc::v0::AvailableDevicesRequest>,
    ) -> TonicResult<grpc::v0::AvailableDevicesResponse> {
        todo!()
    }

    async fn connect(
        &self,
        request: Request<grpc::v0::ConnectRequest>,
    ) -> TonicResult<grpc::v0::ConnectResponse> {
        // open port
        // save connection
        // return some id I guess or error
        todo!()
    }

    async fn disconnect(
        &self,
        request: Request<grpc::v0::DisconnectRequest>,
    ) -> TonicResult<grpc::v0::DisconnectResponse> {
        // close port
        // remove connection
        // return some id I guess or error
        todo!()
    }
}
