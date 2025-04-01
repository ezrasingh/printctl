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

use super::devices;
use super::state::{ServerState, SharedState};
use crate::discovery::ServiceDiscovery;
use crate::server::devices::DeviceManager;

type TonicResult<T> = Result<T, tonic::Status>;
type TonicResponse<T> = TonicResult<Response<T>>;
type ResponseStream<T> = Pin<Box<dyn TokioStream<Item = TonicResult<T>> + Send>>;

#[tonic::async_trait]
impl grpc::Printctl for SharedState<ServerState<ServiceDiscovery>> {
    type DeviceConnectionStream = ResponseStream<grpc::v0::DeviceEvent>;

    async fn available_devices(
        &self,
        _: Request<grpc::v0::AvailableDevicesRequest>,
    ) -> TonicResponse<grpc::v0::AvailableDevicesResponse> {
        let devices: Vec<grpc::v0::DeviceInfo> = Self::list_ports()
            .into_iter()
            .map(|port_info| port_info.into())
            .collect();

        let reply = grpc::v0::AvailableDevicesResponse { devices };
        Ok(Response::new(reply))
    }

    async fn device_connection(
        &self,
        request: Request<grpc::v0::DeviceStreamRequest>,
    ) -> TonicResponse<Self::DeviceConnectionStream> {
        use devices::string_decoder::StringDecoder;
        use futures::{StreamExt, future::FutureExt};
        use tokio_util::bytes::Bytes;
        use tokio_util::codec::BytesCodec;

        let req = request.into_inner();
        let baud_rate = req.baud_rate.try_into().unwrap();
        let stream = self
            .lock()
            .unwrap()
            .create_stream(&req.device_port, baud_rate)
            .expect("could not create serial stream");

        let (port_rx, port_tx) = tokio::io::split(stream);

        let mut serial_reader = tokio_util::codec::FramedRead::new(port_rx, StringDecoder::new());
        let serial_sink = tokio_util::codec::FramedWrite::new(port_tx, BytesCodec::new());
        let (serial_writer, serial_consumer) = futures::channel::mpsc::unbounded::<Bytes>();

        tokio::spawn(async move {
            loop {
                let mut serial_event = serial_reader.next().fuse();
            }
        });
        todo!()
    }
}
