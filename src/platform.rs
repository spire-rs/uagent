/// The platform a [`Profile`](crate::Profile) runs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Platform {
    /// Windows 10 or 11, 64-bit.
    Windows,
    /// macOS.
    MacOs,
    /// Desktop Linux, 64-bit.
    Linux,
    /// An Android phone.
    Android,
    /// An iPhone.
    Ios,
}

impl Platform {
    /// The desktop platforms.
    pub const DESKTOP: [Self; 3] = [Self::Windows, Self::MacOs, Self::Linux];

    pub(crate) const fn is_mobile(self) -> bool {
        matches!(self, Self::Android | Self::Ios)
    }

    /// The platform as Chromium's `Sec-CH-UA-Platform` names it.
    pub(crate) const fn client_hint(self) -> &'static str {
        match self {
            Self::Windows => "\"Windows\"",
            Self::MacOs => "\"macOS\"",
            Self::Linux => "\"Linux\"",
            Self::Android => "\"Android\"",
            Self::Ios => "\"iOS\"",
        }
    }
}
