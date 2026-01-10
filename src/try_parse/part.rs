use thiserror::Error;

use crate::{
    Part,
    boundaries::{HeadersBody, HeadersBodyError},
};

#[derive(Debug, Error)]
pub(crate) enum PartError {
    #[error("{source} - {content_type}")]
    Mime {
        source: mime::FromStrError,
        content_type: Box<str>,
    },
}

impl<'a> Part<'a> {
    pub(crate) fn try_parse(bounded: &'a str) -> Result<Self, PartError> {
        let HeadersBody { headers, body } = match HeadersBody::try_parse(bounded) {
            Ok(ok) => ok,
            Err(HeadersBodyError::NoHeaderSeperator) => {
                return Ok(Self {
                    content_type: mime::TEXT_PLAIN,
                    body: bounded,
                });
            }
        };

        for line in headers.lines() {
            let (name, value) = match line.split_once(':') {
                Some(some) => some,
                None => continue,
            };

            if name.eq_ignore_ascii_case("content-type") {
                let content_type = match value.trim().parse() {
                    Ok(ok) => ok,
                    Err(err) => {
                        return Err(PartError::Mime {
                            source: err,
                            content_type: value.into(),
                        });
                    }
                };

                return Ok(Self { content_type, body });
            }
        }

        Ok(Self {
            content_type: mime::TEXT_PLAIN,
            body,
        })
    }
}
