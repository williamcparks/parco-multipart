use thiserror::Error;

/// Represents errors that can occur while parsing or handling multipart messages.
#[derive(Debug, Error)]
pub enum MultiPartError {
    /// Occurs when the parser cannot determine the boundary string from the message.
    ///
    /// The inner `Box<str>` contains the raw message that caused the failure.
    #[error("Could Not Infer Boundary From Message\n\n{0}")]
    CouldNotInferBoundary(Box<str>),

    /// Occurs when the multipart message does not contain the expected starting boundary.
    ///
    /// The inner `Box<str>` contains the raw message that caused the failure.
    #[error("No Starting Boundary\n\n{0}")]
    NoStartingBoundary(Box<str>),

    /// Occurs when the multipart message does not contain the expected ending boundary.
    ///
    /// The inner `Box<str>` contains the raw message that caused the failure.
    #[error("No Ending Boundary\n\n{0}")]
    NoEndingBoundary(Box<str>),

    /// Represents a MIME parsing error encountered while processing the multipart message.
    ///
    /// Contains the original `mime::FromStrError` as `source`, the `content_type` that caused the error
    /// The `message` field contains the raw message that caused the failure.
    #[error("Mime: {source} Due To {content_type}\n\n{message}")]
    Mime {
        source: mime::FromStrError,
        content_type: Box<str>,
        message: Box<str>,
    },
}
