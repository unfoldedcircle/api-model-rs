// Copyright (c) 2023 Unfolded Circle ApS and contributors
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

/// Retrieve a language text from a language map.
///
/// 1. If the language is not available, the default english text with key `en` is returned.
/// 2. If an english text is missing, the first entry in the map is returned.
/// 3. None is returned if the map is empty.
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
///     ("en".into(), "movie".into()),
///     ("de".into(), "Film".into()),
///     ("en-UK".into(), "film".into())]));
///
/// // get existing language text
/// let text = text_from_language_map(map.as_ref(), "de");
/// assert_eq!(Some("Film"), text);
/// let text = text_from_language_map(map.as_ref(), "en-UK");
/// assert_eq!(Some("film"), text);
/// // fallback to english default
/// let text = text_from_language_map(map.as_ref(), "it");
/// assert_eq!(Some("movie"), text);
/// ```
pub fn text_from_language_map(
    map: Option<&HashMap<String, String>>,
    lang: impl AsRef<str>,
) -> Option<&str> {
    if let Some(map) = map {
        map.get(lang.as_ref())
            // if not found try getting english
            .or_else(|| map.get("en"))
            .or_else(|| map.get("en-UK"))
            .or_else(|| map.get("en-US"))
            // fallback: first entry in language map
            .or_else(|| map.iter().next().map(|(_, v)| v))
            .map(|v| v.as_str())
    } else {
        None
    }
}
