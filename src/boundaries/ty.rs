use crate::MultiPartError;

use super::{BoundaryError, Cursor};

pub(crate) struct Boundaries<'a> {
    src: &'a str,
    boundary: &'a str,
}

impl<'a> Boundaries<'a> {
    pub(crate) const fn new(src: &'a str, boundary: &'a str) -> Self {
        Self { src, boundary }
    }

    pub(crate) fn try_infer_boundary(src: &'_ str) -> Result<&'_ str, MultiPartError> {
        match src.strip_prefix("--").and_then(|v| v.lines().next()) {
            Some(line) => Ok(line),
            None => Err(MultiPartError::CouldNotInferBoundary(src.into())),
        }
    }
}

impl<'a> Iterator for Boundaries<'a> {
    type Item = Result<&'a str, BoundaryError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cursor = Cursor::new(self.src);
        let (range, line) = match cursor.next() {
            Some((range, line)) if line.starts_with("--") && line.contains(self.boundary) => {
                (range, line)
            }
            _ => return Some(Err(BoundaryError::NoStarting)),
        };
        if line.ends_with("--") {
            return None;
        }

        let start_idx = range.end + 1;

        for (range, line) in cursor {
            if !line.starts_with("--") || !line.contains(self.boundary) {
                continue;
            }

            let end_idx = range.start;

            let content = unsafe { self.src.get_unchecked(start_idx..end_idx) };

            self.src = unsafe { self.src.get_unchecked(range.start..) };

            return Some(Ok(content));
        }

        Some(Err(BoundaryError::NoEnding))
    }
}
