#[path = "./"]
pub mod grpc {
    #[path = "printctl.rs"]
    pub mod v0;

    pub use v0::printctl_server::Printctl;
    pub use v0::printctl_server::PrintctlServer;
}

use std::pin::Pin;
use tokio_stream::Stream as TokioStream;
use tonic::{Request, Response};

use super::state::{ServerState, SharedState};
use crate::discovery::ServiceDiscovery;

type TonicResult<T> = Result<T, tonic::Status>;
type TonicResponse<T> = TonicResult<Response<T>>;
type ResponseStream<T> = Pin<Box<dyn TokioStream<Item = TonicResult<T>> + Send>>;

#[tonic::async_trait]
impl grpc::Printctl for SharedState<ServerState<ServiceDiscovery>> {
    type DeviceStream = ResponseStream<grpc::v0::DeviceEvent>;

    async fn available_devices(
        &self,
        request: Request<grpc::v0::AvailableDevicesRequest>,
    ) -> TonicResponse<grpc::v0::AvailableDevicesResponse> {
        todo!()
    }

    async fn device(
        &self,
        request: Request<grpc::v0::DeviceStreamRequest>,
    ) -> TonicResponse<Self::DeviceStream> {
        // open port
        // save connection
        // return some id I guess or error
        todo!()
    }
}
