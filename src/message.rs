use mime::Mime;

/// Represents a multipart message composed of multiple `Part`s.
///
/// `Message` is a container for the individual parts of a multipart message,
/// providing convenient methods to access, iterate, or consume the parts.
#[derive(Clone, Debug)]
pub struct Message<'a> {
    pub(crate) parts: Vec<Part<'a>>,
}

impl<'a> Message<'a> {
    /// Consumes the `Message` and returns its parts as a `Vec<Part>`.
    pub fn into_parts(self) -> Vec<Part<'a>> {
        self.parts
    }

    /// Returns a shared slice of the parts contained in the message.
    pub const fn parts(&'_ self) -> &'_ [Part<'a>] {
        self.parts.as_slice()
    }

    /// Returns an iterator over the parts of the message.
    pub fn iter(&self) -> std::slice::Iter<'_, Part<'a>> {
        self.parts.iter()
    }
}

/// Represents a single part of a multipart message.
///
/// Each `Part` contains a `Content-Type` and a body as a string slice.
#[derive(Clone, Debug)]
pub struct Part<'a> {
    pub(crate) content_type: Mime,
    pub(crate) body: &'a str,
}

impl<'a> Part<'a> {
    /// Consumes the `Part` and returns its `Content-Type`.
    pub fn into_content_type(self) -> Mime {
        self.content_type
    }

    /// Returns a reference to the `Content-Type` of this part.
    pub const fn content_type(&self) -> &Mime {
        &self.content_type
    }

    /// Returns a reference to the body of this part.
    pub const fn body(&self) -> &'a str {
        self.body
    }
}
