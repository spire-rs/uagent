## uagent

[![Build Status][action-badge]][action-url]
[![Crate Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

**Check out other `spire` projects [here](https://github.com/spire-rs).**

[action-badge]: https://img.shields.io/github/actions/workflow/status/spire-rs/uagent/build.yml?branch=main&label=build&logo=github&style=flat-square
[action-url]: https://github.com/spire-rs/uagent/actions/workflows/build.yml
[crates-badge]: https://img.shields.io/crates/v/uagent.svg?logo=rust&style=flat-square
[crates-url]: https://crates.io/crates/uagent
[docs-badge]: https://img.shields.io/docsrs/uagent?logo=Docs.rs&style=flat-square
[docs-url]: https://docs.rs/uagent

Browser profiles for Chrome, Edge, Firefox and Safari in the Rust programming
language, with user agents, matching navigation headers and deterministic
selection by relative browser and platform weights. Browser captures ship with
the crate and require no data fetch during builds or at runtime.

### Examples

- create a browser profile and read its navigation headers:

```rust
use uagent::{Browser, Platform, Profile};

let profile = Profile::new(Browser::Chrome, Platform::Windows)?;
assert_eq!(profile.version(), Browser::Chrome.latest());

// Header structs are returned in capture order.
for header in profile.headers() {
    println!("{}: {}", header.name, header.value);
}
# Ok::<(), uagent::Error>(())
```

- override the language preferences, validating BCP 47 tags and quality values:

```rust
use uagent::{Browser, Platform, Profile};

let profile = Profile::new(Browser::Firefox, Platform::MacOs)?
    .with_language("de-DE,de;q=0.9")?;
assert!(profile.headers().iter().any(|header| {
    header.name == "accept-language" && header.value == "de-DE,de;q=0.9"
}));
# Ok::<(), uagent::Error>(())
```

- pick a profile for each crawler identity, deterministically by its number:

```rust
use uagent::{Browser, Platform, UserAgents};

let agents = UserAgents::new()
    .browsers([Browser::Chrome, Browser::Firefox])
    .share(Browser::Firefox, 20)
    .platforms([Platform::Windows, Platform::MacOs])
    .versions(Browser::Chrome, 155..=156)
    .language("en-US,en;q=0.9")?;

assert_eq!(agents.pick(42), agents.pick(42));
# Ok::<(), uagent::Error>(())
```

### Links

- [User-Agent](https://www.rfc-editor.org/rfc/rfc9110.html#section-10.1.5) and
  [Accept-Language](https://www.rfc-editor.org/rfc/rfc9110.html#section-12.5.4)
  in RFC 9110 on RFC-Editor.org
- [Language Tags](https://www.rfc-editor.org/rfc/rfc5646.html) in RFC 5646
  on RFC-Editor.org
- [User-Agent Client Hints](https://developer.chrome.com/docs/privacy-security/user-agent-client-hints)
  on Chrome for Developers
- [Browser capture guide](https://github.com/spire-rs/uagent/tree/main/tools)
  in this repository

### Notes

- Headers describe a page opened directly. Host, cookies, connection management
  and Accept-Encoding are left to the HTTP client. TLS and HTTP/2 emulation are
  outside this crate's scope.
- Default browser weights are Chrome 65, Safari 18, Edge 10 and Firefox 7, with
  desktop platforms and the latest two major versions, or one for Safari. These
  weights are configurable approximations. Empty selections and invalid version
  ranges panic during configuration.
- Selection is stable for the same configuration and crate version. Patch
  releases refresh captures; pin the crate version to retain exact profiles.
- Initial captures cover macOS. Other platform variants originate from the Spire
  draft and have not been verified against captures. iOS browsers other than
  Safari, and Android Edge, are not modeled.
- Other major versions reuse the latest captured navigation headers. Historical
  header changes are not modeled; Safari retains its captured minor/patch suffix.
