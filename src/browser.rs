use std::ops::RangeInclusive;

use crate::{Header, Platform, generated};

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

    /// Returns the latest captured navigation header template.
    pub(crate) const fn headers(self) -> &'static [Header] {
        match self {
            Self::Chrome => generated::CHROME_HEADERS,
            Self::Edge => generated::EDGE_HEADERS,
            Self::Firefox => generated::FIREFOX_HEADERS,
            Self::Safari => generated::SAFARI_HEADERS,
        }
    }

    /// The newest major version in this release's capture data.
    #[must_use]
    pub const fn latest(self) -> u32 {
        match self {
            Self::Chrome => generated::CHROME_LATEST,
            Self::Edge => generated::EDGE_LATEST,
            Self::Firefox => generated::FIREFOX_LATEST,
            Self::Safari => generated::SAFARI_LATEST,
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

    /// The default relative weight of this browser.
    pub(crate) const fn default_share(self) -> u32 {
        match self {
            Browser::Chrome => 65,
            Browser::Safari => 18,
            Browser::Edge => 10,
            Browser::Firefox => 7,
        }
    }

    /// A browser's latest two major versions, Safari's latest.
    pub(crate) fn default_versions(self) -> RangeInclusive<u32> {
        let latest = self.latest();
        match self {
            Browser::Safari => latest..=latest,
            _ => latest - 1..=latest,
        }
    }

    /// The relative weight of `platform` among this browser's users.
    pub(crate) const fn share(self, platform: Platform) -> u32 {
        match (self, platform) {
            (Browser::Chrome | Browser::Firefox, Platform::Windows) => 70,
            (Browser::Chrome, Platform::MacOs) => 22,
            (Browser::Chrome, Platform::Linux) => 8,
            (Browser::Firefox, Platform::MacOs | Platform::Linux) => 15,
            (Browser::Edge, Platform::Windows) => 90,
            (Browser::Edge, Platform::MacOs) => 9,
            (Browser::Edge, Platform::Linux) => 1,
            _ => 100,
        }
    }
}
