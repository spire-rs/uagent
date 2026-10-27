use crate::{Browser, Error, Platform, Result, data, language};

/// A browser version and platform with navigation headers for a page opened directly.
///
/// The latest capture supplies header values; other major versions substitute the
/// version into the user agent and Chromium client hints. This does not emulate
/// TLS, HTTP/2, or the browser's networking stack.
///
/// ```
/// use uagents::{Browser, Platform, Profile};
/// let profile = Profile::new(Browser::Chrome, 156, Platform::Windows)?;
/// assert!(profile.user_agent().contains("Chrome/156.0.0.0"));
/// # Ok::<_, uagents::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    browser: Browser,
    version: u32,
    platform: Platform,
    pub(crate) language: Option<String>,
}

impl Profile {
    /// Creates the profile of `browser` at major `version` on `platform`.
    ///
    /// # Errors
    ///
    /// Returns an error if the browser/platform combination has no template,
    /// or `version` is zero. Versions other than the captured version use
    /// the latest captured navigation headers with a substituted major version.
    pub fn new(browser: Browser, version: u32, platform: Platform) -> Result<Self> {
        if !browser.platforms().contains(&platform) {
            return Err(Error::UnsupportedPlatform { browser, platform });
        }
        if version == 0 {
            return Err(Error::InvalidVersion);
        }
        Ok(Self {
            browser,
            version,
            platform,
            language: None,
        })
    }

    /// Overrides the captured `Accept-Language` value.
    ///
    /// Accepts a BCP 47 language tag or a comma-separated list of language tags
    /// and optional quality values, such as `de-DE,de;q=0.9`. `*` is also allowed.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidLanguage`] for invalid tags, quality values,
    /// or characters that cannot appear in the header.
    pub fn with_language(mut self, language: &str) -> Result<Self> {
        language::validate(language)?;
        self.language = Some(language.to_owned());
        Ok(self)
    }

    /// Returns the browser.
    #[must_use]
    pub const fn browser(&self) -> Browser {
        self.browser
    }

    /// Returns the major version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    /// Returns the platform.
    #[must_use]
    pub const fn platform(&self) -> Platform {
        self.platform
    }

    /// Returns the captured user agent with the requested version and platform.
    ///
    /// Chromium reduces its version to the major; Safari retains the captured
    /// minor/patch suffix. Platform variants without captures use the draft's
    /// fixed platform descriptions.
    #[must_use]
    pub fn user_agent(&self) -> String {
        let template = data::headers(self.browser)
            .iter()
            .find_map(|&(name, value)| (name == "user-agent").then_some(value))
            .unwrap_or_else(|| unreachable!("every browser capture has a user agent"));
        let latest = self.browser.latest();
        let version = self.version;
        let mut agent = template.to_owned();
        for product in ["Chrome/", "Edg/", "Firefox/", "Version/", "rv:"] {
            agent = agent.replace(
                &format!("{product}{latest}."),
                &format!("{product}{version}."),
            );
        }
        let platform = match (self.browser, self.platform) {
            (Browser::Firefox, Platform::MacOs) => "Macintosh; Intel Mac OS X 10.15",
            (Browser::Firefox, Platform::Android) => "Android 15; Mobile",
            (_, Platform::Windows) => "Windows NT 10.0; Win64; x64",
            (_, Platform::MacOs) => "Macintosh; Intel Mac OS X 10_15_7",
            (_, Platform::Linux) => "X11; Linux x86_64",
            (_, Platform::Android) => "Linux; Android 10; K",
            (_, Platform::Ios) => "iPhone; CPU iPhone OS 18_6 like Mac OS X",
        };
        if let Some(start) = agent.find('(')
            && let Some(end) = agent[start..].find(')')
        {
            let platform = if self.browser == Browser::Firefox {
                format!("{platform}; rv:{version}.0")
            } else {
                platform.to_owned()
            };
            agent.replace_range(start + 1..start + end, &platform);
        }
        if self.platform == Platform::Android {
            if self.browser == Browser::Firefox {
                agent = agent.replace("Gecko/20100101", &format!("Gecko/{version}.0"));
            } else {
                agent = agent.replace(" Safari/", " Mobile Safari/");
            }
        } else if self.platform == Platform::Ios {
            agent = agent.replace(" Safari/605.1.15", " Mobile/15E148 Safari/604.1");
        }
        agent
    }

    /// Returns Chromium's `Sec-CH-UA`: the browser, Chromium and a made-up
    /// brand, ordered and named by the major version, as Chromium derives
    /// them.
    fn brands(&self) -> String {
        const GREASE_CHARS: [&str; 11] = [" ", "(", ":", "-", ".", "/", ")", ";", "=", "?", "_"];
        const GREASE_VERSIONS: [&str; 3] = ["8", "99", "24"];
        const ORDERS: [[usize; 3]; 6] = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];

        let seed = usize::try_from(self.version).expect("u32 fits usize on supported targets");
        let grease = format!(
            "\"Not{}A{}Brand\";v=\"{}\"",
            GREASE_CHARS[seed % GREASE_CHARS.len()],
            GREASE_CHARS[(seed + 1) % GREASE_CHARS.len()],
            GREASE_VERSIONS[seed % GREASE_VERSIONS.len()],
        );
        let brand = match self.browser {
            Browser::Edge => "Microsoft Edge",
            _ => "Google Chrome",
        };
        let order = ORDERS[seed % ORDERS.len()];
        let mut brands = [String::new(), String::new(), String::new()];
        brands[order[0]] = grease;
        brands[order[1]] = format!("\"Chromium\";v=\"{}\"", self.version);
        brands[order[2]] = format!("\"{brand}\";v=\"{}\"", self.version);
        brands.join(", ")
    }

    /// Returns header names and values in capture order, excluding transport
    /// headers (`Host`, connection management, and `Accept-Encoding`).
    ///
    /// `Accept-Encoding` belongs to the HTTP client, which knows its decoders.
    /// Header order is a convenience for clients that support it; an HTTP header
    /// map does not preserve it.
    #[must_use]
    pub fn headers(&self) -> Vec<(&'static str, String)> {
        data::headers(self.browser)
            .iter()
            .map(|&(name, value)| {
                let value = match name {
                    "user-agent" => self.user_agent(),
                    "sec-ch-ua" => self.brands(),
                    "sec-ch-ua-mobile" => if self.platform.is_mobile() {
                        "?1"
                    } else {
                        "?0"
                    }
                    .to_owned(),
                    "sec-ch-ua-platform" => self.platform.client_hint().to_owned(),
                    "accept-language" => self.language.clone().unwrap_or_else(|| value.to_owned()),
                    _ => value.to_owned(),
                };
                (name, value)
            })
            .collect()
    }
}
