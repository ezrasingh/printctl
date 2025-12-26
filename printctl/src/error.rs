use crate::printer::PrinterCommand;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("serial port error: {0}")]
    Serial(#[from] tokio_serial::Error),

    #[error("Printer is not connected")]
    NotConnected,

    #[error("Channel send error")]
    Recv(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("Channel receive error")]
    Send(#[from] tokio::sync::mpsc::error::SendError<PrinterCommand>),
}
