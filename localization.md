# Widget-Lokalisierung

Die Texte des mCaptcha-Widgets werden aus JSON-Dateien unter `locales/` geladen.
Englisch (`locales/en.json`) ist die Standardsprache; Deutsch ist in
`locales/de.json` enthalten.

## Sprache konfigurieren

Setze beim mCaptcha-Server die Umgebungsvariable `MCAPTCHA_LANGUAGE`, zum Beispiel:

```text
MCAPTCHA_LANGUAGE=de-DE
```

`MCAPTCHA_language` wird aus Kompatibilitätsgründen ebenfalls unterstützt. Die
Sprachangabe wird ohne Beachtung der Groß-/Kleinschreibung ausgewertet und
Unterstriche werden in Bindestriche umgewandelt. Zuerst wird der vollständige
Sprachcode gesucht; gibt es dafür keine eigene Locale, wird die Primärsprache
verwendet. Somit verwenden `de-DE`, `de_DE` und `DE` die deutsche Locale. Fehlt
die Variable oder ist die Sprache unbekannt, verwendet das Widget Englisch.

Die Locale wird beim Start des Servers ausgewählt und in die Widget-Seite
gerendert. Eine Änderung der Umgebungsvariable wird daher nach einem Neustart
des Servers wirksam.

## Sprache hinzufügen oder Texte ändern

1. Lege `locales/<sprachcode>.json` mit allen Schlüsseln aus `locales/en.json`
   an. Die englische Datei ist die Referenz für den vollständigen Schlüsselsatz.
2. Ergänze den Sprachcode und `include_str!`-Eintrag in `LOCALES` in
   `src/locale.rs`. Sprachcodes werden in Kleinbuchstaben und ohne
   Gebietssuffix registriert, zum Beispiel `fr` für `fr-FR`.
3. Übersetze die Werte der JSON-Datei. Die Schlüssel müssen unverändert bleiben:

   | Schlüssel | Verwendung |
   | --- | --- |
   | `before` | Text vor der Verifizierung |
   | `during` | Text während der Verarbeitung |
   | `after` | Text nach erfolgreicher Verifizierung |
   | `error` | Fehlermeldung |
   | `noscript` | Hinweis bei deaktiviertem JavaScript |
   | `source_code` | Link zum Quellcode |
   | `logo_alt` | Alternativtext des Logos |
   | `privacy` | Datenschutz-Link |
   | `terms` | Link zu den Bedingungen |

Die Server- und HTML-Lokalisierung wird in `src/locale.rs`,
`src/widget/mod.rs` und `templates/widget/index.html` umgesetzt. Die Texte für
die dynamischen Checkbox-Zustände werden als `data-msg-*`-Attribute in die
Widget-Seite geschrieben und in `templates/widget/const.ts` vom Client
ausgelesen. Die statischen Texte stehen direkt im serverseitig gerenderten
HTML.

Die JSON-Dateien werden beim Kompilieren mit `include_str!` eingebettet. Nach
Änderungen an Übersetzungen oder nach dem Hinzufügen einer Sprache muss der
Server deshalb neu gebaut und das neue Image ausgerollt werden.

## Tests

Die Auflösung der Sprachcodes und der Fallback auf Englisch sind im Test
`locale::tests::locale_resolution_works` abgedeckt. Zum Testen der
Locale-Logik:

```sh
cargo test --bin mcaptcha locale
```
