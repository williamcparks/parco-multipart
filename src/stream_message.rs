use crate::{
    MultiPartError, Part,
    boundaries::{Boundaries, BoundaryError},
    try_parse::PartError,
};

/// Attempts to parse a raw multipart message via streaming its corresponding `Part`s.
///
/// ## Collecting into a [`Vec`]
///
/// if you want to collect the parts into a [`Vec`] use [`crate::Message`] instead
///
/// This function:
/// 1. Infers the multipart boundary from the input string.
/// 2. Splits the input into bounded sections using that boundary.
/// 3. Parses each bounded section into a `Part`.
pub struct StreamMessage<'a> {
    boundaries: Boundaries<'a>,
    input: &'a str,
}

impl<'a> StreamMessage<'a> {
    /// infer the boundary of the message and setup the iterator
    pub fn try_parse(input: &'a str) -> Result<Self, MultiPartError> {
        let boundary = Boundaries::try_infer_boundary(input)?;
        Ok(Self {
            boundaries: Boundaries::new(input, boundary),
            input,
        })
    }
}

impl<'a> Iterator for StreamMessage<'a> {
    type Item = Result<Part<'a>, MultiPartError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let bounded_result = self.boundaries.next()?;

        let bounded = match bounded_result {
            Ok(ok) => ok,
            Err(BoundaryError::NoStarting) => {
                let err = MultiPartError::NoStartingBoundary(self.input.into());
                self.input = "";
                return Some(Err(err));
            }
            Err(BoundaryError::NoEnding) => {
                let err = MultiPartError::NoEndingBoundary(self.input.into());
                self.input = "";
                return Some(Err(err));
            }
        };

        let part = match Part::try_parse(bounded) {
            Ok(ok) => ok,
            Err(PartError::Mime {
                source,
                content_type,
            }) => {
                let message: Box<str> = self.input.into();
                self.input = "";
                return Some(Err(MultiPartError::Mime {
                    source,
                    content_type,
                    message,
                }));
            }
        };

        Some(Ok(part))
    }
}
