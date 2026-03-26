use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_query_map};
use leptos_router::NavigateOptions;

use crate::components::filter_chips::FilterChips;
use crate::components::filter_panel::FilterPanel;
use crate::components::school_card::SchoolCard;
use crate::components::sort_controls::SortControls;
use crate::components::view_toggle::ViewToggle;
use crate::models::{all_districts, all_languages, all_profiles, School, SortField};
use crate::pages::map::MapView;
use crate::state::AppState;

/// Parse a comma-separated query param value into a Vec<String>.
fn parse_csv_param(query: &leptos_router::params::ParamsMap, key: &str) -> Vec<String> {
    query
        .get(key)
        .filter(|v| !v.is_empty())
        .map(|v| v.split(',').map(String::from).collect())
        .unwrap_or_default()
}

/// Parse a tri-state query param (ja/nein/absent) into Option<bool>.
fn parse_tristate_param(query: &leptos_router::params::ParamsMap, key: &str) -> Option<bool> {
    match query.get(key).as_deref() {
        Some("ja") => Some(true),
        Some("nein") => Some(false),
        _ => None,
    }
}

/// Build a query string from filter state. Only includes non-default values.
fn build_query_string(
    districts: &[String],
    profiles: &[String],
    grundstaendig: Option<bool>,
    languages: &[String],
    ganztag: Option<bool>,
    sort: &SortField,
    view: &str,
) -> String {
    let mut params = Vec::new();

    if !districts.is_empty() {
        params.push(format!("district={}", districts.join(",")));
    }
    if !profiles.is_empty() {
        params.push(format!("profile={}", profiles.join(",")));
    }
    match grundstaendig {
        Some(true) => params.push("grundstaendig=ja".to_string()),
        Some(false) => params.push("grundstaendig=nein".to_string()),
        None => {}
    }
    if !languages.is_empty() {
        params.push(format!("language={}", languages.join(",")));
    }
    match ganztag {
        Some(true) => params.push("ganztag=ja".to_string()),
        Some(false) => params.push("ganztag=nein".to_string()),
        None => {}
    }
    if *sort != SortField::Name {
        params.push(format!("sort={}", sort.to_query()));
    }
    if view == "map" {
        params.push("view=map".to_string());
    }

    params.join("&")
}

/// Toggle a value in a list: add if absent, remove if present.
fn toggle_in_list(list: &[String], value: &str) -> Vec<String> {
    if list.contains(&value.to_string()) {
        list.iter().filter(|v| v.as_str() != value).cloned().collect()
    } else {
        let mut new_list = list.to_vec();
        new_list.push(value.to_string());
        new_list
    }
}

/// Apply filters and sorting to the school list.
fn filter_and_sort(
    schools: &[School],
    districts: &[String],
    profiles: &[String],
    grundstaendig: Option<bool>,
    languages: &[String],
    ganztag: Option<bool>,
    sort: &SortField,
) -> Vec<School> {
    let mut filtered: Vec<School> = schools
        .iter()
        .filter(|s| {
            // District filter
            if !districts.is_empty() && !districts.contains(&s.district) {
                return false;
            }
            // Profile filter: school must have at least one matching profile
            if !profiles.is_empty()
                && !s.profile.iter().any(|p| profiles.contains(p))
            {
                return false;
            }
            // Grundstaendig filter
            if let Some(wanted) = grundstaendig {
                if wanted && s.accepts_after_4th_grade != Some(true) {
                    return false;
                }
                if !wanted && s.accepts_after_4th_grade == Some(true) {
                    return false;
                }
            }
            // Language filter: school must offer at least one matching language
            if !languages.is_empty()
                && !s.languages.iter().any(|l| languages.contains(&l.name))
            {
                return false;
            }
            // Ganztag filter
            if let Some(wanted) = ganztag {
                if wanted && s.ganztag != Some(true) {
                    return false;
                }
                if !wanted && s.ganztag == Some(true) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    // Sort
    match sort {
        SortField::Name => filtered.sort_by(|a, b| a.name.cmp(&b.name)),
        SortField::District => {
            filtered.sort_by(|a, b| a.district.cmp(&b.district).then(a.name.cmp(&b.name)))
        }
        SortField::StudentCount => filtered.sort_by(|a, b| {
            // Descending by student count; None sorts last
            b.student_count
                .unwrap_or(0)
                .cmp(&a.student_count.unwrap_or(0))
        }),
        // Travel time sorts -- will be wired with actual data in Plan 04-02.
        // For now, fall back to name sort when travel times are not yet available.
        SortField::TravelTimeWalk
        | SortField::TravelTimeBike
        | SortField::TravelTimeCar => {
            filtered.sort_by(|a, b| a.name.cmp(&b.name));
        }
    }

    filtered
}

/// The main listing page showing all schools as filterable cards.
#[component]
pub fn ListingPage() -> impl IntoView {
    let state = use_context::<AppState>().expect("AppState must be provided");
    let all_schools = state.schools.clone();

    // Derive filter options from the full school list
    let districts_list = all_districts(&all_schools);
    let profiles_list = all_profiles(&all_schools);
    let languages_list = all_languages(&all_schools);

    let query = use_query_map();
    let navigate = use_navigate();

    // Read filter state from URL query params
    let selected_districts = Signal::derive(move || {
        parse_csv_param(&query.read(), "district")
    });
    let selected_profiles = Signal::derive(move || {
        parse_csv_param(&query.read(), "profile")
    });
    let selected_grundstaendig = Signal::derive(move || {
        parse_tristate_param(&query.read(), "grundstaendig")
    });
    let selected_languages = Signal::derive(move || {
        parse_csv_param(&query.read(), "language")
    });
    let selected_ganztag = Signal::derive(move || {
        parse_tristate_param(&query.read(), "ganztag")
    });
    let current_sort = Signal::derive(move || {
        query
            .read()
            .get("sort")
            .map(|s| SortField::from_query(&s))
            .unwrap_or_default()
    });

    // Read view mode from URL query param
    let is_map_view = Signal::derive(move || {
        query.read().get("view").as_deref() == Some("map")
    });

    // Count active filters
    let active_filter_count = Signal::derive(move || {
        let mut count = 0;
        count += selected_districts.get().len();
        count += selected_profiles.get().len();
        if selected_grundstaendig.get().is_some() {
            count += 1;
        }
        count += selected_languages.get().len();
        if selected_ganztag.get().is_some() {
            count += 1;
        }
        count
    });

    // Helper to navigate with updated filters (now includes view parameter)
    let nav = navigate.clone();
    let navigate_with_filters = move |districts: Vec<String>,
                                       profiles: Vec<String>,
                                       grundstaendig: Option<bool>,
                                       languages: Vec<String>,
                                       ganztag: Option<bool>,
                                       sort: SortField,
                                       view: &str| {
        let qs = build_query_string(&districts, &profiles, grundstaendig, &languages, ganztag, &sort, view);
        let path = if qs.is_empty() {
            "/".to_string()
        } else {
            format!("/?{}", qs)
        };
        nav(
            &path,
            NavigateOptions {
                replace: true,
                ..Default::default()
            },
        );
    };

    // Callbacks for filter changes — preserve current view mode
    let nav_fn = navigate_with_filters.clone();
    let on_toggle_district = Callback::new(move |district: String| {
        let new_districts = toggle_in_list(&selected_districts.get(), &district);
        let view = if is_map_view.get() { "map" } else { "" };
        nav_fn(
            new_districts,
            selected_profiles.get(),
            selected_grundstaendig.get(),
            selected_languages.get(),
            selected_ganztag.get(),
            current_sort.get(),
            view,
        );
    });

    let nav_fn = navigate_with_filters.clone();
    let on_toggle_profile = Callback::new(move |profile: String| {
        let new_profiles = toggle_in_list(&selected_profiles.get(), &profile);
        let view = if is_map_view.get() { "map" } else { "" };
        nav_fn(
            selected_districts.get(),
            new_profiles,
            selected_grundstaendig.get(),
            selected_languages.get(),
            selected_ganztag.get(),
            current_sort.get(),
            view,
        );
    });

    let nav_fn = navigate_with_filters.clone();
    let on_set_grundstaendig = Callback::new(move |val: Option<bool>| {
        let view = if is_map_view.get() { "map" } else { "" };
        nav_fn(
            selected_districts.get(),
            selected_profiles.get(),
            val,
            selected_languages.get(),
            selected_ganztag.get(),
            current_sort.get(),
            view,
        );
    });

    let nav_fn = navigate_with_filters.clone();
    let on_toggle_language = Callback::new(move |language: String| {
        let new_languages = toggle_in_list(&selected_languages.get(), &language);
        let view = if is_map_view.get() { "map" } else { "" };
        nav_fn(
            selected_districts.get(),
            selected_profiles.get(),
            selected_grundstaendig.get(),
            new_languages,
            selected_ganztag.get(),
            current_sort.get(),
            view,
        );
    });

    let nav_fn = navigate_with_filters.clone();
    let on_set_ganztag = Callback::new(move |val: Option<bool>| {
        let view = if is_map_view.get() { "map" } else { "" };
        nav_fn(
            selected_districts.get(),
            selected_profiles.get(),
            selected_grundstaendig.get(),
            selected_languages.get(),
            val,
            current_sort.get(),
            view,
        );
    });

    let nav_fn = navigate_with_filters.clone();
    let on_sort_change = Callback::new(move |sort: SortField| {
        let view = if is_map_view.get() { "map" } else { "" };
        nav_fn(
            selected_districts.get(),
            selected_profiles.get(),
            selected_grundstaendig.get(),
            selected_languages.get(),
            selected_ganztag.get(),
            sort,
            view,
        );
    });

    let nav_fn = navigate_with_filters.clone();
    let on_clear_all = Callback::new(move |_: ()| {
        let view = if is_map_view.get() { "map" } else { "" };
        nav_fn(
            vec![],
            vec![],
            None,
            vec![],
            None,
            SortField::Name,
            view,
        );
    });

    // View toggle callback — switches between list and map views
    let nav_fn = navigate_with_filters.clone();
    let on_toggle_view = Callback::new(move |_: ()| {
        let new_view = if is_map_view.get() { "" } else { "map" };
        nav_fn(
            selected_districts.get(),
            selected_profiles.get(),
            selected_grundstaendig.get(),
            selected_languages.get(),
            selected_ganztag.get(),
            current_sort.get(),
            new_view,
        );
    });

    // Filtered + sorted schools memo
    let schools_for_memo = all_schools.clone();
    let filtered_schools = Memo::new(move |_| {
        filter_and_sort(
            &schools_for_memo,
            &selected_districts.get(),
            &selected_profiles.get(),
            selected_grundstaendig.get(),
            &selected_languages.get(),
            selected_ganztag.get(),
            &current_sort.get(),
        )
    });

    let school_count = move || filtered_schools.get().len();

    view! {
        <main class="listing-page">
            <header class="listing-header">
                <h1>"Berliner Gymnasien"</h1>
                <p class="school-count">{move || format!("{} Schulen gefunden", school_count())}</p>
                <div class="header-controls">
                    <FilterChips active_count=active_filter_count on_clear_all=on_clear_all />
                    <SortControls current_sort=current_sort on_sort_change=on_sort_change />
                    <ViewToggle is_map_view=is_map_view on_toggle=on_toggle_view />
                </div>
            </header>
            <div class="listing-content">
                <aside class="filter-sidebar">
                    <FilterPanel
                        districts=districts_list
                        profiles=profiles_list
                        languages=languages_list
                        selected_districts=selected_districts
                        selected_profiles=selected_profiles
                        selected_grundstaendig=selected_grundstaendig
                        selected_languages=selected_languages
                        selected_ganztag=selected_ganztag
                        on_toggle_district=on_toggle_district
                        on_toggle_profile=on_toggle_profile
                        on_toggle_language=on_toggle_language
                        on_set_grundstaendig=on_set_grundstaendig
                        on_set_ganztag=on_set_ganztag
                    />
                </aside>
                <div style:display=move || {
                    if is_map_view.get() { "block" } else { "none" }
                } style="flex:1;min-width:0">
                    <MapView filtered_schools=filtered_schools />
                </div>
                <section class="school-grid" style:display=move || {
                    if is_map_view.get() { "none" } else { "" }
                }>
                    <For
                        each=move || filtered_schools.get()
                        key=|s| s.school_id.clone()
                        let:school
                    >
                        <SchoolCard school=school />
                    </For>
                </section>
            </div>
        </main>
    }
}
