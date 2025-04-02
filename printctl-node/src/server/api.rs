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
        use tokio_stream::wrappers::ReceiverStream;
        use tokio_util::codec::FramedRead;

        let req = request.into_inner();
        let baud_rate = req.baud_rate.try_into().unwrap();
        let device_stream = self
            .lock()
            .unwrap()
            .create_stream(&req.device_port, baud_rate)
            .expect("could not create device stream");

        let (device_rx, _) = tokio::io::split(device_stream);

        let mut device_reader = FramedRead::new(device_rx, StringDecoder::new());
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        tokio::spawn(async move {
            loop {
                let serial_event = device_reader.next().fuse();
                tokio::select! {
                    maybe_serial = serial_event => {
                        match maybe_serial {
                            Some(Ok(message)) => {
                                println!("{}", message);
                                let device_event = grpc::v0::DeviceEvent {
                                    message
                                };
                                match tx.send(Ok(device_event)).await {
                                    Ok(_) => {
                                        // item (server response) was queued to be send to client
                                    },
                                    Err(e) => {
                                        println!("Error transmitting DeviceEvent: {:?}\r", e);
                                        // output_stream was build from rx and both are dropped
                                        break;
                                    },
                                }
                            },
                            Some(Err(e)) => {
                                println!("Device serial Error: {:?}\r", e);
                                // This most likely means that the serial port has been unplugged.
                                break;
                            },
                            None => continue,
                        }
                    }
                }
            }
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::DeviceConnectionStream
        ))
    }
}
