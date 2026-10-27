use std::fmt;

use crate::{Browser, Platform};

/// A result returned by browser profile configuration.
pub type Result<T> = std::result::Result<T, Error>;

/// An invalid browser profile configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Browser versions must be greater than zero.
    InvalidVersion,
    /// No template is available for this browser/platform combination.
    UnsupportedPlatform {
        /// The requested browser.
        browser: Browser,
        /// The requested platform.
        platform: Platform,
    },
    /// An Accept-Language value contains an invalid language tag or quality value.
    InvalidLanguage,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidVersion => f.write_str("a browser version must be above zero"),
            Self::UnsupportedPlatform { browser, platform } => {
                write!(f, "no template for {browser:?} on {platform:?}")
            }
            Self::InvalidLanguage => f.write_str("invalid Accept-Language tag or quality value"),
        }
    }
}

impl std::error::Error for Error {}
