use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskQueueError {
    #[error("Closed")]
    Closed,

    #[error("Timeout")]
    Timeout,
}
