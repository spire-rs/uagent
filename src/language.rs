use oxilangtag::LanguageTag;

use crate::{Error, Result};

/// Validates language tags and weights in an Accept-Language field value.
pub(crate) fn validate(language: &str) -> Result<()> {
    // OWS permits spaces and tabs, but never newlines or other control bytes.
    if !language
        .bytes()
        .all(|byte| byte == b'\t' || (32..127).contains(&byte))
    {
        return Err(Error::InvalidLanguage);
    }
    for preference in language.split(',') {
        let (tag, quality) = preference
            .split_once(';')
            .map_or((preference, None), |(tag, quality)| (tag, Some(quality)));
        let tag = tag.trim();
        if tag != "*" && LanguageTag::parse(tag).is_err() {
            return Err(Error::InvalidLanguage);
        }
        if let Some(quality) = quality {
            let Some((name, value)) = quality.trim().split_once('=') else {
                return Err(Error::InvalidLanguage);
            };
            if !name.eq_ignore_ascii_case("q") || !valid_quality(value) {
                return Err(Error::InvalidLanguage);
            }
        }
    }
    Ok(())
}

fn valid_quality(value: &str) -> bool {
    let (integer, fraction) = value.split_once('.').unwrap_or((value, ""));
    matches!(integer, "0" | "1")
        && fraction.len() <= 3
        && fraction
            .bytes()
            .all(|digit| digit.is_ascii_digit() && (integer == "0" || digit == b'0'))
}
