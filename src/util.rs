// Copyright (c) 2023 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

/// Retrieve a language text from a language map.
///
/// 1. Try retrieving an exact language match first. E.g. `de_DE`.
/// 2. Then try without country specific variant only. E.g. `de`.
/// 3. Then try another country variant. If multiple variants are available, a random variant is
///    returned. E.g. `de_CH`
/// 4. If the language is not available, the default English text with key `en` is returned.
/// 5. If an English text is missing, the first entry in the map is returned.
/// 6. None is returned if the map is empty.
///
/// # Arguments
///
/// * `map`: the language map with (language_key, language_text) entries.
/// * `lang`: the language key
///
/// returns: the found language text, `None` if the map is empty.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use uc_api::util::text_from_language_map;
///
/// let map = Some(HashMap::from([
///     ("en".into(), "English fallback".into()),
///     ("de".into(), "German fallback".into()),
///     ("de_DE".into(), "German".into()),
///     ("de_CH".into(), "Swiss German".into()),
///     ("en_UK".into(), "UK English".into())]));
///
/// // direct match
/// let text = text_from_language_map(map.as_ref(), "en_UK");
/// assert_eq!(Some("UK English"), text);
/// // fallback to main language if country specific variant is missing
/// let text = text_from_language_map(map.as_ref(), "de_AT");
/// assert_eq!(Some("German fallback"), text);
/// // fallback to English default if language is missing
/// let text = text_from_language_map(map.as_ref(), "it");
/// assert_eq!(Some("English fallback"), text);
/// ```
pub fn text_from_language_map(
    map: Option<&HashMap<String, String>>,
    lang: impl AsRef<str>,
) -> Option<&str> {
    if let Some(map) = map {
        let lang = lang.as_ref();
        let short_lang = lang.split_once('_').map(|(l, _)| l).unwrap_or("en");

        // direct match first
        map.get(lang)
            // if not found try language fallback
            .or_else(|| {
                map.iter()
                    .find_map(|(k, v)| if k == short_lang { Some(v) } else { None })
            })
            // if not found return first matching country variant (random)
            .or_else(|| {
                map.iter().find_map(|(k, v)| {
                    if k.starts_with(&format!("{short_lang}_")) {
                        Some(v)
                    } else {
                        None
                    }
                })
            })
            // English
            .or_else(|| map.get("en"))
            // fallback: first entry in language map
            .or_else(|| map.iter().next().map(|(_, v)| v))
            .map(|v| v.as_str())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn test_languages() -> HashMap<String, String> {
        HashMap::from([
            ("en".into(), "English fallback".into()),
            ("de".into(), "German fallback".into()),
            ("de_DE".into(), "German".into()),
            ("de_CH".into(), "Swiss German".into()),
            ("en_UK".into(), "UK English".into()),
            ("en_US".into(), "US English".into()),
            ("en_AU".into(), "AU English".into()),
            ("fr_FR".into(), "French".into()),
        ])
    }

    #[test]
    fn text_from_language_map_without_language_map() {
        let text = text_from_language_map(None, "en_UK");
        assert_eq!(None, text);
    }

    #[test]
    fn text_from_language_map_with_direct_match() {
        let map = Some(test_languages());

        let text = text_from_language_map(map.as_ref(), "en_UK");
        assert_eq!(Some("UK English"), text);
        let text = text_from_language_map(map.as_ref(), "de_CH");
        assert_eq!(Some("Swiss German"), text);
        let text = text_from_language_map(map.as_ref(), "de_DE");
        assert_eq!(Some("German"), text);
    }

    #[test]
    fn text_from_language_map_with_missing_country_and_no_default() {
        let map = Some(test_languages());

        let text = text_from_language_map(map.as_ref(), "fr_CA");
        assert_eq!(Some("French"), text);
    }

    #[test]
    fn text_from_language_map_with_missing_language() {
        let map = Some(test_languages());

        let text = text_from_language_map(map.as_ref(), "it");
        assert_eq!(Some("English fallback"), text);
    }
}
