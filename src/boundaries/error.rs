use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum BoundaryError {
    #[error("No Starting Boundary")]
    NoStarting,

    #[error("No Ending Boundary")]
    NoEnding,
}

#[derive(Debug, Error)]
pub(crate) enum HeadersBodyError {
    #[error("No Header Seperator")]
    NoHeaderSeperator,
}
