use std::fmt;
use std::ops::RangeInclusive;

use crate::{Browser, Platform, Profile, Result, language};

/// Selects browser profiles by relative browser and platform weights.
///
/// [`pick`](Self::pick) is deterministic for a given configuration and crate
/// version. Capture updates in patch releases may change profiles; pin the crate
/// version when resuming a crawl must retain exactly the same headers.
///
/// ```
/// use uagents::{Browser, UserAgents};
/// let agents = UserAgents::new().browsers([Browser::Chrome, Browser::Firefox]);
/// assert_eq!(agents.pick(7), agents.pick(7));
/// ```
#[derive(Clone)]
pub struct UserAgents {
    browsers: Vec<(Browser, u32)>,
    platforms: Vec<Platform>,
    versions: Vec<(Browser, RangeInclusive<u32>)>,
    language: Option<String>,
}

impl Default for UserAgents {
    /// Desktop Chrome, Safari, Edge and Firefox, at their latest two major
    /// versions, Safari at its latest, see [`UserAgents::new`].
    fn default() -> Self {
        Self {
            browsers: [
                Browser::Chrome,
                Browser::Safari,
                Browser::Edge,
                Browser::Firefox,
            ]
            .map(|browser| (browser, default_share(browser)))
            .to_vec(),
            platforms: Platform::DESKTOP.to_vec(),
            versions: Vec::new(),
            language: None,
        }
    }
}

impl UserAgents {
    /// Picks among desktop Chrome, Safari, Edge and Firefox, in proportion
    /// to configurable approximate shares: 65, 18, 10 and 7. Each
    /// runs on its own mix of desktop platforms, at its latest two major
    /// versions, Safari at its latest.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Picks only among `browsers`, keeping their shares, so their relative
    /// weights stay the same.
    ///
    /// # Panics
    ///
    /// Panics if none of `browsers` runs on the
    /// [platforms](Self::platforms).
    #[must_use]
    pub fn browsers(mut self, browsers: impl IntoIterator<Item = Browser>) -> Self {
        let mut unique = Vec::new();
        for browser in browsers {
            if !unique.contains(&browser) {
                unique.push(browser);
            }
        }
        self.browsers = unique
            .into_iter()
            .map(|browser| {
                let share = self
                    .browsers
                    .iter()
                    .find(|(other, _)| *other == browser)
                    .map_or_else(|| default_share(browser), |(_, share)| *share);
                (browser, share)
            })
            .collect();
        self.check();
        self
    }

    /// Picks `browser` in proportion to `share` among the others' shares,
    /// adding it if it was left out; a share of zero leaves it out.
    ///
    /// # Panics
    ///
    /// Panics if no browser left runs on the [platforms](Self::platforms).
    #[must_use]
    pub fn share(mut self, browser: Browser, share: u32) -> Self {
        self.browsers.retain(|(other, _)| *other != browser);
        if share > 0 {
            self.browsers.push((browser, share));
        }
        self.check();
        self
    }

    /// Picks among `platforms`, the desktop ones by default; each browser
    /// runs on those of them it supports.
    ///
    /// # Panics
    ///
    /// Panics if none of the browsers runs on any of `platforms`.
    #[must_use]
    pub fn platforms(mut self, platforms: impl IntoIterator<Item = Platform>) -> Self {
        self.platforms = platforms.into_iter().collect();
        self.check();
        self
    }

    /// Picks `browser`'s major version among `versions`, instead of its
    /// latest.
    ///
    /// # Panics
    ///
    /// Panics if `versions` is empty or starts at zero.
    #[must_use]
    pub fn versions(mut self, browser: Browser, versions: RangeInclusive<u32>) -> Self {
        assert!(
            !versions.is_empty() && *versions.start() > 0,
            "`UserAgents::versions` needs versions above zero"
        );
        self.versions.retain(|(other, _)| *other != browser);
        self.versions.push((browser, versions));
        self
    }

    /// Overrides every profile's captured `Accept-Language` value.
    ///
    /// Accepts the same language tags and quality values as [`Profile::with_language`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::InvalidLanguage`] for invalid tags, quality values,
    /// or characters that cannot appear in the header.
    pub fn language(mut self, language: &str) -> Result<Self> {
        language::validate(language)?;
        self.language = Some(language.to_owned());
        Ok(self)
    }

    fn check(&self) {
        assert!(
            self.browsers.iter().any(|(browser, _)| {
                browser
                    .platforms()
                    .iter()
                    .any(|platform| self.platforms.contains(platform))
            }),
            "`UserAgents` has no browser that runs on its platforms"
        );
    }

    /// Returns the profile of identity `n`.
    #[must_use]
    pub fn pick(&self, n: u64) -> Profile {
        let mut draws = Draws(n);
        let browsers: Vec<(Browser, u32)> = self
            .browsers
            .iter()
            .copied()
            .filter(|(browser, _)| {
                browser
                    .platforms()
                    .iter()
                    .any(|platform| self.platforms.contains(platform))
            })
            .collect();
        let browser = weighted(&browsers, draws.next());

        let platforms: Vec<(Platform, u32)> = browser
            .platforms()
            .iter()
            .copied()
            .filter(|platform| self.platforms.contains(platform))
            .map(|platform| (platform, share(browser, platform)))
            .collect();
        let platform = weighted(&platforms, draws.next());

        let versions = self
            .versions
            .iter()
            .find(|(other, _)| *other == browser)
            .map_or_else(
                || default_versions(browser),
                |(_, versions)| versions.clone(),
            );
        let span = u64::from(versions.end() - versions.start()) + 1;
        let offset = u32::try_from(draws.next() % span).unwrap_or_default();
        let version = versions.start() + offset;

        let mut profile = Profile::new(browser, version, platform)
            .unwrap_or_else(|_| unreachable!("browsers are only picked on their platforms"));
        profile.language.clone_from(&self.language);
        profile
    }
}

impl fmt::Debug for UserAgents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserAgents")
            .field("browsers", &self.browsers)
            .field("platforms", &self.platforms)
            .field("versions", &self.versions)
            .finish_non_exhaustive()
    }
}

/// How common `browser` is, relative to the others.
const fn default_share(browser: Browser) -> u32 {
    match browser {
        Browser::Chrome => 65,
        Browser::Safari => 18,
        Browser::Edge => 10,
        Browser::Firefox => 7,
    }
}

/// A browser's latest two major versions, Safari's latest.
fn default_versions(browser: Browser) -> RangeInclusive<u32> {
    let latest = browser.latest();
    match browser {
        Browser::Safari => latest..=latest,
        _ => latest - 1..=latest,
    }
}

/// How common `platform` is among `browser`'s users.
const fn share(browser: Browser, platform: Platform) -> u32 {
    match (browser, platform) {
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

/// Picks among `choices` in proportion to their weights, by `draw`.
fn weighted<T: Copy>(choices: &[(T, u32)], draw: u64) -> T {
    let total: u64 = choices.iter().map(|(_, weight)| u64::from(*weight)).sum();
    let mut point = draw % total.max(1);
    for (choice, weight) in choices {
        let weight = u64::from(*weight);
        if point < weight {
            return *choice;
        }
        point -= weight;
    }
    choices[0].0
}

/// Numbers drawn from `n`, the same in every run: splitmix64.
struct Draws(u64);

impl Draws {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}
