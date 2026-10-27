use std::collections::HashMap;

use serde::Deserialize;
use uagent::{Browser, Error, Platform, Profile, UserAgents};

#[derive(Deserialize)]
struct Capture {
    browser: String,
    version: u32,
    platform: String,
    headers: Vec<(String, String)>,
}

#[test]
fn every_header_matches_the_latest_real_browser_capture() {
    let mut captures = HashMap::<String, Capture>::new();
    for path in std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/captures")).unwrap() {
        let path = path.unwrap().path();
        if path.extension().is_none_or(|extension| extension != "json") {
            continue;
        }
        let capture: Capture = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        let replace = captures.get(&capture.browser).is_none_or(|previous| {
            (capture.version, capture.platform == "MacOs")
                > (previous.version, previous.platform == "MacOs")
        });
        if replace {
            captures.insert(capture.browser.clone(), capture);
        }
    }
    for browser in Browser::ALL {
        let capture = &captures[&format!("{browser:?}")];
        let platform = match capture.platform.as_str() {
            "MacOs" => Platform::MacOs,
            "Windows" => Platform::Windows,
            "Linux" => Platform::Linux,
            other => panic!("unknown capture platform {other}"),
        };
        assert_eq!(browser.latest(), capture.version);
        let profile = Profile::new(browser, platform).unwrap();
        let actual: Vec<_> = profile
            .headers()
            .into_iter()
            .map(|header| (header.name.to_owned(), header.value.into_owned()))
            .collect();
        assert_eq!(actual, capture.headers, "{browser:?}");
    }
}

#[test]
fn chromium_hints_follow_the_published_version_141_example() {
    // https://developer.chrome.com/blog/user-agent-reduction-android-model-and-version
    let profile = Profile::new(Browser::Chrome, Platform::Windows)
        .unwrap()
        .with_version(141)
        .unwrap();
    let headers: HashMap<_, _> = profile
        .headers()
        .into_iter()
        .map(|header| (header.name, header.value.into_owned()))
        .collect();
    assert_eq!(
        headers["sec-ch-ua"],
        r#""Google Chrome";v="141", "Not?A_Brand";v="8", "Chromium";v="141""#
    );
    assert_eq!(headers["sec-ch-ua-platform"], "\"Windows\"");
    assert_eq!(headers["sec-ch-ua-mobile"], "?0");
    let edge = Profile::new(Browser::Edge, Platform::Windows)
        .unwrap()
        .with_version(141)
        .unwrap();
    let headers: HashMap<_, _> = edge
        .headers()
        .into_iter()
        .map(|header| (header.name, header.value.into_owned()))
        .collect();
    assert_eq!(
        headers["sec-ch-ua"],
        r#""Microsoft Edge";v="141", "Not?A_Brand";v="8", "Chromium";v="141""#
    );
}

#[test]
fn all_supported_platforms_produce_consistent_profiles() {
    for browser in Browser::ALL {
        for platform in Platform::ALL {
            let profile = Profile::new(browser, platform);
            if !browser.platforms().contains(&platform) {
                assert_eq!(
                    profile,
                    Err(Error::UnsupportedPlatform { browser, platform })
                );
                continue;
            }
            let profile = profile.unwrap();
            let headers: HashMap<_, _> = profile
                .headers()
                .into_iter()
                .map(|header| (header.name, header.value.into_owned()))
                .collect();
            assert_eq!(headers["user-agent"], profile.user_agent());
            assert!(!headers.contains_key("accept-encoding"));
            assert!(!headers.contains_key("host"));
            assert!(!headers.contains_key("cookie"));
            if matches!(browser, Browser::Chrome | Browser::Edge) {
                assert_eq!(
                    headers["sec-ch-ua-mobile"],
                    if Platform::MOBILE.contains(&platform) {
                        "?1"
                    } else {
                        "?0"
                    }
                );
                assert!(headers["sec-ch-ua"].contains(&browser.latest().to_string()));
            } else {
                assert!(!headers.contains_key("sec-ch-ua"));
            }
        }
    }
    assert!(matches!(
        Profile::new(Browser::Chrome, Platform::Linux)
            .unwrap()
            .with_version(0),
        Err(Error::InvalidVersion)
    ));
    assert!(matches!(
        Profile::new(Browser::Safari, Platform::Windows),
        Err(Error::UnsupportedPlatform { .. })
    ));
    assert!(Profile::new(Browser::Firefox, Platform::Ios).is_err());
}

#[test]
fn language_overrides_work_for_profiles_and_picks() {
    let profile = Profile::new(Browser::Chrome, Platform::MacOs).unwrap();
    assert_eq!(
        profile
            .clone()
            .with_language("de-DE,de;q=0.9")
            .unwrap()
            .headers()
            .into_iter()
            .map(|header| (header.name, header.value.into_owned()))
            .collect::<HashMap<_, _>>()["accept-language"],
        "de-DE,de;q=0.9"
    );
    for invalid in ["en\r\nx-injected: yes", "en\0", "français", "en\u{7f}"] {
        assert_eq!(
            profile.clone().with_language(invalid),
            Err(Error::InvalidLanguage)
        );
        assert!(UserAgents::new().language(invalid).is_err());
    }
    let picked = UserAgents::new().language("fr-FR").unwrap().pick(42);
    assert_eq!(
        picked
            .headers()
            .into_iter()
            .map(|header| (header.name, header.value.into_owned()))
            .collect::<HashMap<_, _>>()["accept-language"],
        "fr-FR"
    );
}

#[test]
fn language_tags_wildcards_and_quality_values_are_validated() {
    let profile = Profile::new(Browser::Chrome, Platform::MacOs).unwrap();
    for valid in [
        "en",
        "zh-Hant-TW",
        "en-US-u-ca-gregory",
        "x-spire",
        "i-klingon",
        "*",
        "de-DE, de;q=0.9, *;q=0.001",
        "en;q=0",
        "en;q=1",
        "en;q=0.",
        "en;q=1.000",
        "en\t;\tQ=0.5",
    ] {
        let updated = profile.clone().with_language(valid).unwrap();
        let headers: HashMap<_, _> = updated
            .headers()
            .into_iter()
            .map(|header| (header.name, header.value.into_owned()))
            .collect();
        assert_eq!(headers["accept-language"], valid);
        assert!(UserAgents::new().language(valid).is_ok(), "{valid:?}");
    }
    for invalid in [
        "",
        " ",
        "en_US",
        "en--US",
        "en-u",
        "en,",
        ",en",
        "en,,de",
        "en;q=",
        "en;q=.5",
        "en;q=-0.1",
        "en;q=1.1",
        "en;q=2",
        "en;q=00",
        "en;q=0.1234",
        "en;q=NaN",
        "en;q=0.5;q=0.2",
        "en;p=0.5",
        "en;q =0.5",
    ] {
        assert_eq!(
            profile.clone().with_language(invalid),
            Err(Error::InvalidLanguage),
            "{invalid:?}"
        );
        assert!(UserAgents::new().language(invalid).is_err(), "{invalid:?}");
    }
}

#[test]
fn picks_are_deterministic_and_follow_browser_and_platform_weights() {
    let agents = UserAgents::new();
    for n in [0, 1, 7, 42, u64::MAX] {
        assert_eq!(agents.pick(n), agents.clone().pick(n));
    }
    let mut counts = HashMap::new();
    let mut chrome_windows = 0;
    for n in 0..20_000 {
        let profile = agents.pick(n);
        *counts.entry(profile.browser()).or_insert(0) += 1;
        if profile.browser() == Browser::Chrome && profile.platform() == Platform::Windows {
            chrome_windows += 1;
        }
        assert!(Platform::DESKTOP.contains(&profile.platform()));
        let latest = profile.browser().latest();
        assert!(if profile.browser() == Browser::Safari {
            profile.version() == latest
        } else {
            (latest - 1..=latest).contains(&profile.version())
        });
    }
    for (browser, expected) in [
        (Browser::Chrome, 13_000),
        (Browser::Safari, 3_600),
        (Browser::Edge, 2_000),
        (Browser::Firefox, 1_400),
    ] {
        assert!(
            (expected - 400..=expected + 400).contains(&counts[&browser]),
            "{browser:?}: {}",
            counts[&browser]
        );
    }
    assert!((8_700..9_500).contains(&chrome_windows));
}

#[test]
fn restrictions_duplicates_and_extreme_weights_and_versions_are_supported() {
    let agents = UserAgents::new()
        .browsers([Browser::Chrome, Browser::Firefox, Browser::Chrome])
        .share(Browser::Firefox, u32::MAX)
        .share(Browser::Chrome, u32::MAX)
        .platforms([Platform::Linux])
        .versions(Browser::Chrome, 1..=u32::MAX)
        .versions(Browser::Firefox, 141..=141);
    for n in [0, 42, u64::MAX] {
        let profile = agents.pick(n);
        assert_eq!(profile.platform(), Platform::Linux);
        assert!([Browser::Chrome, Browser::Firefox].contains(&profile.browser()));
    }
    let unique = UserAgents::new().browsers([Browser::Chrome, Browser::Firefox]);
    let duplicates =
        UserAgents::new().browsers([Browser::Chrome, Browser::Firefox, Browser::Chrome]);
    assert!((0..100).all(|n| unique.pick(n) == duplicates.pick(n)));
    let firefox = unique
        .share(Browser::Chrome, 0)
        .versions(Browser::Firefox, 141..=141);
    assert!(
        (0..100)
            .all(|n| firefox.pick(n).browser() == Browser::Firefox
                && firefox.pick(n).version() == 141)
    );
    let safari = UserAgents::new()
        .browsers([Browser::Safari])
        .platforms([Platform::Ios]);
    assert_eq!(safari.pick(42).platform(), Platform::Ios);
}

#[test]
fn invalid_selections_fail_during_configuration() {
    assert!(std::panic::catch_unwind(|| UserAgents::new().browsers([])).is_err());
    assert!(std::panic::catch_unwind(|| UserAgents::new().platforms([])).is_err());
    assert!(
        std::panic::catch_unwind(|| UserAgents::new()
            .browsers([Browser::Safari])
            .platforms([Platform::Linux]))
        .is_err()
    );
    assert!(
        std::panic::catch_unwind(|| UserAgents::new().versions(Browser::Chrome, 0..=1)).is_err()
    );
    let reversed = std::ops::RangeInclusive::new(2, 1);
    assert!(
        std::panic::catch_unwind(|| UserAgents::new().versions(Browser::Chrome, reversed)).is_err()
    );
    assert!(
        std::panic::catch_unwind(|| UserAgents::new()
            .browsers([Browser::Chrome])
            .share(Browser::Chrome, 0))
        .is_err()
    );
}

#[test]
fn profiles_default_to_latest_and_can_restore_it_after_version_overrides() {
    for browser in Browser::ALL {
        let original = Profile::new(browser, browser.platforms()[0])
            .unwrap()
            .with_language("de-DE,de;q=0.9")
            .unwrap();
        assert_eq!(original.version(), browser.latest());
        for version in [1, browser.latest() - 1, u32::MAX] {
            let overridden = original.clone().with_version(version).unwrap();
            assert_eq!(overridden.version(), version);
            let restored = overridden.with_latest_version();
            assert_eq!(restored, original);
            assert_eq!(restored.headers(), original.headers());
        }
        assert_eq!(original.clone().with_latest_version(), original);
        assert_eq!(original.with_version(0), Err(Error::InvalidVersion));
    }
}
