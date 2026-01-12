use crate::{
    Message, MultiPartError, Part,
    boundaries::{Boundaries, BoundaryError},
    try_parse::part::PartError,
};

impl<'a> Message<'a> {
    /// Attempts to parse a raw multipart message into a `Message` containing its `Part`s.
    ///
    /// ## Streaming
    ///
    /// if you want to stream the parts into an iterator use [`crate::StreamMessage`] instead
    ///
    /// This function:
    /// 1. Infers the multipart boundary from the input string.
    /// 2. Splits the input into bounded sections using that boundary.
    /// 3. Parses each bounded section into a `Part`.
    pub fn try_parse(input: &'a str) -> Result<Self, MultiPartError> {
        let mut parts = Vec::new();

        let boundary = Boundaries::try_infer_boundary(input)?;
        for bounded_result in Boundaries::new(input, boundary) {
            let bounded = match bounded_result {
                Ok(ok) => ok,
                Err(BoundaryError::NoStarting) => {
                    return Err(MultiPartError::NoStartingBoundary(input.into()));
                }
                Err(BoundaryError::NoEnding) => {
                    return Err(MultiPartError::NoEndingBoundary(input.into()));
                }
            };

            let part = match Part::try_parse(bounded) {
                Ok(ok) => ok,
                Err(PartError::Mime {
                    source,
                    content_type,
                }) => {
                    return Err(MultiPartError::Mime {
                        source,
                        content_type,
                        message: input.into(),
                    });
                }
            };

            parts.push(part);
        }

        Ok(Self { parts })
    }
}
