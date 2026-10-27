use crate::{Platform, data};

/// A browser a [`Profile`](crate::Profile) presents as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Browser {
    /// Google Chrome.
    Chrome,
    /// Microsoft Edge.
    Edge,
    /// Mozilla Firefox.
    Firefox,
    /// Apple Safari.
    Safari,
}

impl Browser {
    /// All browsers with profile templates.
    pub const ALL: [Self; 4] = [Self::Chrome, Self::Edge, Self::Firefox, Self::Safari];

    /// The newest major version in this release's capture data.
    #[must_use]
    pub const fn latest(self) -> u32 {
        match self {
            Self::Chrome => data::CHROME_LATEST,
            Self::Edge => data::EDGE_LATEST,
            Self::Firefox => data::FIREFOX_LATEST,
            Self::Safari => data::SAFARI_LATEST,
        }
    }

    /// Platforms with templates for this browser. Other browser/platform combinations
    /// (including iOS browsers other than Safari) are not modeled.
    #[must_use]
    pub const fn platforms(self) -> &'static [Platform] {
        match self {
            Self::Chrome | Self::Firefox => &[
                Platform::Windows,
                Platform::MacOs,
                Platform::Linux,
                Platform::Android,
            ],
            Self::Edge => &[Platform::Windows, Platform::MacOs, Platform::Linux],
            Self::Safari => &[Platform::MacOs, Platform::Ios],
        }
    }
}
