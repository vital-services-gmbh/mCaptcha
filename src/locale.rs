// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lokalisierung des Widgets. Pro Sprache existiert eine Datei unter `locales/`.
//! Die Sprache wird über die Umgebungsvariable `MCAPTCHA_LANGUAGE` (bzw. `MCAPTCHA_language`)
//! gewählt, z. B. `de-DE`. Standard ist Englisch.
use std::env;

use serde::Deserialize;

/// Sprachcode der Standardsprache
pub const DEFAULT_LANGUAGE: &str = "en";

/// Alle eingebetteten Sprachdateien (Sprachcode, JSON-Inhalt).
/// Neue Sprachen werden hier ergänzt.
const LOCALES: &[(&str, &str)] = &[
    ("en", include_str!("../locales/en.json")),
    ("de", include_str!("../locales/de.json")),
];

/// Übersetzte Texte des Widgets
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Locale {
    /// Text vor der Verifizierung
    pub before: String,
    /// Text während der Verarbeitung
    pub during: String,
    /// Text nach erfolgreicher Verifizierung
    pub after: String,
    /// Text bei einem Fehler
    pub error: String,
    /// Hinweis, wenn JavaScript deaktiviert ist
    pub noscript: String,
    /// Linktext zum Quellcode
    pub source_code: String,
    /// Alternativtext des Logos
    pub logo_alt: String,
    /// Linktext zur Datenschutzseite
    pub privacy: String,
    /// Linktext zu den Bedingungen
    pub terms: String,
}

impl Locale {
    /// Lädt die Texte für `language` (z. B. `de-DE`, `de_DE`, `de`).
    /// Zuerst wird der vollständige Code gesucht, danach nur die Primärsprache.
    /// Unbekannte Sprachen fallen auf Englisch zurück.
    pub fn load(language: &str) -> Self {
        let normalized = language.trim().replace('_', "-").to_lowercase();
        let primary = normalized.split('-').next().unwrap_or("");
        let find = |code: &str| {
            LOCALES
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(code))
        };
        let (_, json) = find(&normalized)
            .or_else(|| find(primary))
            .or_else(|| find(DEFAULT_LANGUAGE))
            .expect("default locale must exist");
        serde_json::from_str(json).expect("locale file is invalid")
    }

    /// Lädt die Texte anhand der Umgebungsvariable `MCAPTCHA_LANGUAGE`
    /// (alternativ `MCAPTCHA_language`).
    pub fn from_env() -> Self {
        let language = env::var("MCAPTCHA_LANGUAGE")
            .or_else(|_| env::var("MCAPTCHA_language"))
            .unwrap_or_else(|_| DEFAULT_LANGUAGE.to_string());
        Self::load(&language)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_resolution_works() {
        let en = Locale::load("en");
        assert_eq!(en.before, "I'm not a robot");
        assert_eq!(Locale::load("unknown"), en);
        assert_eq!(Locale::load(""), en);
        let de = Locale::load("de-DE");
        assert_eq!(de.before, "Ich bin kein Roboter");
        assert_eq!(Locale::load("de_de"), de);
        assert_eq!(Locale::load("DE"), de);
    }
}
