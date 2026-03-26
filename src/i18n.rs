use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Language {
    De,
    En,
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Language::De => "de",
            Language::En => "en",
        }
    }
}

/// Provide the language signal via Leptos context.
pub fn provide_i18n() {
    let stored = get_stored_language();
    let lang = RwSignal::new(stored);
    provide_context(lang);

    Effect::new(move |_| {
        let l = lang.get();
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(el) = doc.document_element() {
                let _ = el.set_attribute("lang", l.code());
            }
            doc.set_title(match l {
                Language::De => "Berliner Gymnasien Vergleich",
                Language::En => "Berlin Gymnasien Comparison",
            });
        }
        store_language(l);
    });
}

pub fn use_language() -> RwSignal<Language> {
    expect_context::<RwSignal<Language>>()
}

fn get_stored_language() -> Language {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("lang").ok().flatten())
        .map(|v| if v == "en" { Language::En } else { Language::De })
        .unwrap_or(Language::De)
}

fn store_language(lang: Language) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.set_item("lang", lang.code());
    }
}

/// Translate a static key.
pub fn t(key: &'static str, lang: Language) -> &'static str {
    use Language::*;
    match (key, lang) {
        // --- Common ---
        ("no_data", De) => "Keine Angabe",
        ("no_data", En) => "No data",
        ("yes", De) => "Ja",
        ("yes", En) => "Yes",
        ("no", De) => "Nein",
        ("no", En) => "No",
        ("all", De) => "Alle",
        ("all", En) => "All",
        ("unknown", De) => "Unbekannt",
        ("unknown", En) => "Unknown",

        // --- Address Input ---
        ("address_placeholder", De) => "Adresse eingeben",
        ("address_placeholder", En) => "Enter address",
        ("calculating", De) => " Berechne...",
        ("calculating", En) => " Calculating...",
        ("searching", De) => " Suche...",
        ("searching", En) => " Searching...",
        ("search", De) => "Suchen",
        ("search", En) => "Search",
        ("clear_address", De) => "Adresse loeschen",
        ("clear_address", En) => "Clear address",

        // --- Filter Panel ---
        ("show_hide_filters", De) => "Filter anzeigen / verbergen",
        ("show_hide_filters", En) => "Show / hide filters",
        ("filters", De) => "Filter",
        ("filters", En) => "Filters",
        ("district", De) => "Bezirk",
        ("district", En) => "District",
        ("profile", De) => "Profil",
        ("profile", En) => "Profile",
        ("grundstaendig", De) => "Grundstaendig (ab Klasse 5)",
        ("grundstaendig", En) => "Starting from grade 5",
        ("foreign_language", De) => "Fremdsprache",
        ("foreign_language", En) => "Foreign language",
        ("ganztag", De) => "Ganztag",
        ("ganztag", En) => "All-day school",

        // --- Filter Chips ---
        ("clear_all_filters", De) => "Alle Filter loeschen",
        ("clear_all_filters", En) => "Clear all filters",

        // --- Sort Controls ---
        ("sort_by", De) => "Sortieren:",
        ("sort_by", En) => "Sort:",
        ("name_az", De) => "Name (A-Z)",
        ("name_az", En) => "Name (A-Z)",
        ("student_count_sort", De) => "Schueleranzahl",
        ("student_count_sort", En) => "Student count",
        ("travel_walk_sort", De) => "Fahrzeit (zu Fuss)",
        ("travel_walk_sort", En) => "Travel time (walking)",
        ("travel_bike_sort", De) => "Fahrzeit (Fahrrad)",
        ("travel_bike_sort", En) => "Travel time (bicycle)",
        ("travel_car_sort", De) => "Fahrzeit (Auto)",
        ("travel_car_sort", En) => "Travel time (car)",

        // --- View Toggle ---
        ("list", De) => "Liste",
        ("list", En) => "List",
        ("map", De) => "Karte",
        ("map", En) => "Map",

        // --- School Card ---
        ("from_grade_5", De) => "ab Klasse 5",
        ("from_grade_5", En) => "from grade 5",
        ("calculating_travel", De) => "Berechne Fahrzeit...",
        ("calculating_travel", En) => "Calculating travel time...",
        ("on_foot", De) => "Zu Fuss",
        ("on_foot", En) => "On foot",
        ("bicycle", De) => "Fahrrad",
        ("bicycle", En) => "Bicycle",
        ("car", De) => "Auto",
        ("car", En) => "Car",
        ("no_travel_time", De) => "Keine Fahrzeit verfuegbar",
        ("no_travel_time", En) => "No travel time available",

        // --- Rating ---
        ("no_rating", De) => "Keine Bewertung",
        ("no_rating", En) => "No rating",

        // --- Listing Page ---
        ("berlin_gymnasien", De) => "Berliner Gymnasien",
        ("berlin_gymnasien", En) => "Berlin Gymnasien",

        // --- Detail Page ---
        ("back_to_list", De) => "\u{2190} Zurueck zur Liste",
        ("back_to_list", En) => "\u{2190} Back to list",
        ("school_not_found", De) => "Schule nicht gefunden",
        ("school_not_found", En) => "School not found",
        ("school_not_found_desc", De) => "Die angeforderte Schule konnte nicht gefunden werden.",
        ("school_not_found_desc", En) => "The requested school could not be found.",
        ("to_overview", De) => "Zur Schuluebersicht",
        ("to_overview", En) => "To school overview",
        ("public_school", De) => "Oeffentlich",
        ("public_school", En) => "Public",
        ("private_school", De) => "Privat",
        ("private_school", En) => "Private",
        ("all_day_school", De) => "Ganztagsschule",
        ("all_day_school", En) => "All-day school",
        ("half_day_school", De) => "Halbtagsschule",
        ("half_day_school", En) => "Half-day school",
        ("grade_average", De) => "Notendurchschnitt",
        ("grade_average", En) => "Grade average",
        ("oversubscribed", De) => "Ueberbucht",
        ("oversubscribed", En) => "Oversubscribed",
        ("selection_process", De) => "Auswahlverfahren",
        ("selection_process", En) => "Selection process",
        ("trial_class", De) => "Probeunterricht",
        ("trial_class", En) => "Trial class",
        ("entrance_test", De) => "Aufnahmetest",
        ("entrance_test", En) => "Entrance test",
        ("notes", De) => "Hinweise",
        ("notes", En) => "Notes",
        ("profile_languages", De) => "Profil & Sprachen",
        ("profile_languages", En) => "Profile & Languages",
        ("language_col", De) => "Sprache",
        ("language_col", En) => "Language",
        ("from_grade", De) => "Ab Klasse",
        ("from_grade", En) => "From grade",
        ("admission", De) => "Aufnahmebedingungen",
        ("admission", En) => "Admission requirements",
        ("ratings", De) => "Bewertungen",
        ("ratings", En) => "Ratings",
        ("open_day", De) => "Tag der offenen Tuer",
        ("open_day", En) => "Open day",
        ("contact", De) => "Kontakt",
        ("contact", En) => "Contact",
        ("telephone", De) => "Telefon",
        ("telephone", En) => "Phone",
        ("email_label", De) => "E-Mail",
        ("email_label", En) => "Email",
        ("data_provenance", De) => "Datenherkunft",
        ("data_provenance", En) => "Data provenance",
        ("last_updated", De) => "Letzte Aktualisierung",
        ("last_updated", En) => "Last updated",
        ("data_sources", De) => "Datenquellen",
        ("data_sources", En) => "Data sources",
        ("completeness_label", De) => "Vollstaendigkeit",
        ("completeness_label", En) => "Completeness",

        // --- Not Found ---
        ("page_not_found", De) => "Seite nicht gefunden",
        ("page_not_found", En) => "Page not found",
        ("page_not_found_desc", De) => "Die angeforderte Seite existiert nicht.",
        ("page_not_found_desc", En) => "The requested page does not exist.",
        ("to_home", De) => "Zur Startseite",
        ("to_home", En) => "To home page",

        // --- Format Templates (use with t_fmt) ---
        ("n_students", De) => "{0} Schueler",
        ("n_students", En) => "{0} students",
        ("pct_complete", De) => "{0}% vollstaendig",
        ("pct_complete", En) => "{0}% complete",
        ("filters_active", De) => "{0} Filter aktiv",
        ("filters_active", En) => "{0} filters active",
        ("n_reviews", De) => "({0} Bewertungen)",
        ("n_reviews", En) => "({0} reviews)",
        ("as_of", De) => "Stand: {0}",
        ("as_of", En) => "As of: {0}",
        ("n_schools_found", De) => "{0} Schulen gefunden",
        ("n_schools_found", En) => "{0} schools found",
        ("calculating_travel_times", De) => " Fahrzeiten werden berechnet...",
        ("calculating_travel_times", En) => " Travel times are being calculated...",
        ("n_students_n_teachers", De) => "{0} Schueler / {1} Lehrkraefte",
        ("n_students_n_teachers", En) => "{0} students / {1} teachers",
        ("n_teachers", De) => "{0} Lehrkraefte",
        ("n_teachers", En) => "{0} teachers",
        ("grade_n", De) => "Klasse {0}",
        ("grade_n", En) => "Grade {0}",
        ("abitur_avg", De) => "Abiturdurchschnitt: {0}",
        ("abitur_avg", En) => "Abitur average: {0}",

        // Fallback: return the key itself
        (other, _) => other,
    }
}

/// Format a translated template with numbered placeholders ({0}, {1}, ...).
pub fn t_fmt(key: &'static str, lang: Language, args: &[&str]) -> String {
    let template = t(key, lang);
    let mut result = template.to_string();
    for (i, arg) in args.iter().enumerate() {
        result = result.replace(&format!("{{{i}}}"), arg);
    }
    result
}

/// Translate a profile type code to a human-readable label.
pub fn profile_label<'a>(profile: &'a str, lang: Language) -> &'a str {
    use Language::*;
    match (profile, lang) {
        ("MINT", _) => "MINT",
        ("bilingual_english", _) => "Bilingual EN",
        ("bilingual_french", _) => "Bilingual FR",
        ("altsprachlich", De) => "Altsprachlich",
        ("altsprachlich", En) => "Classical Languages",
        ("music", De) => "Musik",
        ("music", En) => "Music",
        ("sports", De) => "Sport",
        ("sports", En) => "Sports",
        (p, _) => p,
    }
}
