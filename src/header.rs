use std::borrow::Cow;

/// A named header field in a browser profile.
///
/// Values can borrow from bundled captures or own text derived from the
/// requested version, platform or language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// The lowercase header name.
    pub name: &'static str,
    /// The header field value.
    pub value: Cow<'static, str>,
}

impl Header {
    /// Creates a header from a captured name and value.
    #[must_use]
    pub const fn from_static(name: &'static str, value: &'static str) -> Self {
        Self {
            name,
            value: Cow::Borrowed(value),
        }
    }
}
