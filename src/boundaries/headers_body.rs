use super::{Cursor, HeadersBodyError};

pub(crate) struct HeadersBody<'a> {
    pub headers: &'a str,
    pub body: &'a str,
}

impl<'a> HeadersBody<'a> {
    pub(crate) fn try_parse(bounded: &'a str) -> Result<Self, HeadersBodyError> {
        for (range, line) in Cursor::new(bounded) {
            if line.is_empty() {
                let headers = unsafe { bounded.get_unchecked(..range.start) };
                let body = unsafe { bounded.get_unchecked(range.end..) };
                return Ok(Self { headers, body });
            }
        }

        Err(HeadersBodyError::NoHeaderSeperator)
    }
}
