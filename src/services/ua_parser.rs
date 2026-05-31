use std::sync::Arc;

use ua_parser::Extractor;

#[derive(Clone)]
pub struct UAParser {
    ua_extractor: Arc<Extractor<'static>>,
}

impl UAParser {
    /// Loads `ua_parser` rules from regexes.yaml
    ///
    /// # Panics
    /// When file is invalid or no file found
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        let regexes_file = std::fs::File::open("regexes.yaml").expect("Regexes file expexted");
        let regexes: ua_parser::Regexes<'_> =
            serde_yaml::from_reader(regexes_file).expect("Expected valid regexes file");
        let extractor = Extractor::try_from(regexes).expect("Expected valid regexes");
        Self {
            ua_extractor: Arc::new(extractor),
        }
    }

    /// Parses user agent from http `UserAgent` header string
    ///
    /// # Errors
    /// Unknown user agent
    pub fn parse_ua(&self, ua_string: &str) -> loco_rs::Result<String> {
        let Some(browser) = self.ua_extractor.ua.extract(ua_string) else {
            return Ok("Unknown".to_string());
        };
        let version = match (browser.major, browser.minor) {
            (Some(major), Some(minor)) => format!(" {major}.{minor}"),
            (Some(major), None) => format!(" {major}"),
            _ => String::new(),
        };
        Ok(format!("{}{}", browser.family, version))
    }
}
