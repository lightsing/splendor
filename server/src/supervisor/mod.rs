#[cfg(feature = "supervisor")]
mod enable;
#[cfg(feature = "supervisor")]
pub use enable::*;

#[cfg(not(feature = "supervisor"))]
mod disable;
#[cfg(not(feature = "supervisor"))]
pub use disable::*;

#[derive(Debug, thiserror::Error)]
pub enum SupervisorError {
    #[cfg(feature = "supervisor")]
    #[error("channel error")]
    ChannelError,
}
