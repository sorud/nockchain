use crate::noun::slab::CueError;
use crate::CrownError;
use thiserror::Error;
use tokio::sync::mpsc::error::{SendError, TrySendError};
use tracing::error;

use super::driver::IOAction;

/// Error type for NockApps
#[derive(Debug, Error)]
pub enum NockAppError {
    /// NockApp exited with a code, shouldn't ever be 0, that's a Done/Success.
    #[error("NockApp exited with error code: {0}")]
    Exit(usize),
    #[error("Timeout")]
    Timeout,
    #[error("IO error: {0}")]
    IoError(#[source] std::io::Error),
    #[error("Save error: {0}")]
    SaveError(#[source] std::io::Error),
    #[error("MPSC send error (probably trying to send a poke): {0}")]
    MPSCSendError(#[from] tokio::sync::mpsc::error::SendError<IOAction>),
    #[error("MPSC full error (you probably used try_poke and the channel was full)")]
    MPSCFullError(IOAction),
    #[error("Oneshot receive error (sender dropped): {0}")]
    OneShotRecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error("Error cueing jam buffer: {0}")]
    CueError(#[from] CueError),
    #[error("Error receiving effect broadcast (lagged): {0}")]
    BroadcastRecvLaggedError(u64), // Lagged contains the number of missed messages
    #[error("Error receiving effect broadcast (closed)")]
    BroadcastRecvClosedError,
    #[error("Error joining task (probably the task panicked: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("Error converting string: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Crown error: {0}")]
    CrownError(#[from] CrownError),
    #[error("Channel closed error")]
    ChannelClosedError,
    #[error("Other error")]
    OtherError,
    #[error("Peek failed")]
    PeekFailed,
    #[error("Poke failed")]
    PokeFailed,
    #[error("Unexpected result")]
    UnexpectedResult,
    #[error("nockvm error: {0}")]
    SwordError(#[from] nockvm::noun::Error),
    #[error("Save error: {0}")]
    EncodeError(#[from] bincode::error::EncodeError),
    #[error("Decode error: {0}")]
    DecodeError(#[from] bincode::error::DecodeError),
    #[error("Send error: {0}")]
    SendError(#[from] tokio::sync::watch::error::SendError<u64>),
    #[error("Config error: {0}")]
    ConfigError(#[from] config::ConfigError),
}

impl From<TrySendError<IOAction>> for NockAppError {
    fn from(tse: TrySendError<IOAction>) -> NockAppError {
        match tse {
            TrySendError::Closed(act) => NockAppError::MPSCSendError(SendError(act)),
            TrySendError::Full(act) => NockAppError::MPSCFullError(act),
        }
    }
}
impl From<tokio::sync::broadcast::error::RecvError> for NockAppError {
    fn from(err: tokio::sync::broadcast::error::RecvError) -> Self {
        match err {
            tokio::sync::broadcast::error::RecvError::Lagged(count) => {
                Self::BroadcastRecvLaggedError(count)
            }
            tokio::sync::broadcast::error::RecvError::Closed => Self::BroadcastRecvClosedError,
        }
    }
}
