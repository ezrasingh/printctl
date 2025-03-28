#[path = "."]
pub mod grpc {
    #[path = "printctl.rs"]
    pub mod v0;

    pub use v0::printctl_server::PrintctlServer;
}
