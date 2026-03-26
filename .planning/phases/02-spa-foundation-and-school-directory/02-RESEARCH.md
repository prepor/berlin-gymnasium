# Phase 2: SPA Foundation and School Directory - Research

**Researched:** 2026-03-26
**Domain:** Rust/Leptos WASM SPA with filtering, detail pages, GitHub Pages deployment
**Confidence:** HIGH (core stack verified via official docs and crates.io; one critical routing decision revision required)

## Summary

Phase 2 is a greenfield frontend phase: create a Rust/Leptos CSR single-page application that reads embedded school data (106 YAML files converted to JSON at build time), presents a filterable card grid of Berlin Gymnasien, provides detail pages per school, and deploys to GitHub Pages via GitHub Actions + Trunk.

The Rust toolchain, Trunk, and wasm32-unknown-unknown target are NOT currently installed on the development machine. No Cargo.toml or Rust project structure exists yet. The phase must bootstrap the entire Rust frontend project from scratch. The 106 school YAML files from Phase 1 are ready and will serve as the data source.

**CRITICAL FINDING:** Hash routing (decision D-17 from CONTEXT.md) is NOT available in Leptos 0.8. The `leptos_router` crate does not implement a `HashHistory` or hash-based `LocationProvider`. The standard approach for GitHub Pages is the 404.html fallback: copy `index.html` to `404.html` so GitHub Pages serves the SPA for all paths. This means routes will be path-based (`/school/{id}`) not hash-based (`/#/school/{id}`). Filter state should be persisted in URL query parameters (e.g., `/?district=Mitte&profile=MINT`), which works with the 404.html approach.

**Primary recommendation:** Use path-based routing with 404.html fallback. Use `use_query_map` from `leptos_router` for filter state in URL query params. Use `build.rs` with `serde-saphyr` for YAML-to-JSON conversion at compile time, embedded via `include_str!`.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Clean card grid layout -- each school is a card showing key info at a glance
- **D-02:** Mobile-first responsive design -- cards stack vertically on phone, 2-3 columns on desktop
- **D-03:** Utility CSS approach (inline styles or minimal CSS) -- no heavy framework, keep WASM bundle small
- **D-04:** German-language UI -- target audience is Berlin parents; all labels, filters, and content in German
- **D-05:** Sidebar filters on desktop, collapsible top panel on mobile
- **D-06:** Filters: District (multi-select), Profile (multi-select), Grundstaendig (yes/no/all), Language (multi-select), Ganztag (yes/no/all)
- **D-07:** Filters combine with AND logic (narrow results)
- **D-08:** Active filter count shown as badge; clear-all button
- **D-09:** Filter state persisted in URL hash params so filtered views are shareable
- **D-10:** Card shows: school name, district, profile badges (colored chips), grundstaendig flag (prominent if yes), student count, completeness score indicator
- **D-11:** Cards are clickable -- navigate to detail page
- **D-12:** Sort options: alphabetical, by district, by student count (default: alphabetical)
- **D-13:** Detail page sections in order: Hero (name, district, address, website link) -> Profile & Languages -> Admission Requirements -> Ratings -> Open Day -> Contact -> Data Provenance (sources, last updated, confidence)
- **D-14:** Missing data shown as "Keine Angabe" (no information) rather than hiding the section -- transparency over aesthetics
- **D-15:** Back button returns to listing with filters preserved
- **D-16:** External links (website, email, phone) open in new tab / native handler
- **D-17:** Hash routing (`/#/`, `/#/school/{id}`) -- simplest for GitHub Pages, no 404.html hack needed (**RESEARCH OVERRIDE: Not available in Leptos 0.8 -- see routing section**)
- **D-18:** Routes: `/#/` (listing with filters), `/#/school/{school_id}` (detail page) (**RESEARCH OVERRIDE: Use `/` and `/school/{school_id}` with 404.html fallback**)
- **D-19:** Data embedded at build time: build.rs reads `data/schools/*.yaml` -> serializes to single JSON string -> `include_str!` in WASM binary
- **D-20:** WASM optimization profile: `opt-level = "z"`, `lto = true`, `codegen-units = 1` in Cargo.toml release profile
- **D-21:** GitHub Actions workflow: trunk build -> deploy to gh-pages branch
- **D-22:** Trunk config: `trunk build --release --public-url /amman/` (repo name)

### Claude's Discretion
- Exact CSS styling and color palette -- pick something clean and professional
- Component decomposition -- how to split Leptos components
- Whether to use leptos-use for URL state or hand-roll a simple hash parser
- Build script approach -- build.rs vs separate binary crate for YAML->JSON

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| LIST-01 | User can view a listing of all Berlin Gymnasien with name, district, profile, and grundstaendig flag | Card grid component with embedded JSON data; SchoolRecord fields map directly to card content |
| LIST-02 | User can filter schools by district (Bezirk) | Multi-select filter using `district` field; 12 Berlin districts as enum; URL query param `district=Mitte,Pankow` |
| LIST-03 | User can filter schools by profile/specialization | Multi-select filter using `profile` list field; known values: MINT, bilingual_english, bilingual_french, altsprachlich, music, sports, other |
| LIST-04 | User can filter schools by grundstaendig status | Boolean toggle filter on `accepts_after_4th_grade` field; yes/no/all tri-state |
| LIST-05 | User can filter schools by foreign language offered | Multi-select on `languages[].name` field; known values: Englisch, Franzoesisch, Latein, Spanisch, Russisch, etc. |
| LIST-06 | User can filter schools by Ganztag mode | Boolean toggle on `ganztag` field; yes/no/all tri-state |
| LIST-07 | Listing is mobile-responsive and usable on phone screens | Mobile-first CSS; cards stack vertically; collapsible filter panel; media queries via leptos-use `use_media_query` or CSS |
| DETL-01 | User can view a detail page for each school showing all available data fields | Route `/school/{school_id}` renders full SchoolRecord; all fields displayed with "Keine Angabe" for missing |
| DETL-02 | Detail page shows contact info (phone, email, website link) | Direct mapping from `phone`, `email`, `website` fields with appropriate link types (tel:, mailto:, https) |
| DETL-03 | Detail page shows ratings from each source | Iterate `ratings` dict; each RatingEntry displayed with source, score, scale_min/scale_max, review_count |
| DETL-04 | Detail page shows admission requirements | Render `admission_requirements` sub-struct fields: notendurchschnitt, oversubscribed, selection_criteria, probeunterricht, entrance_test, notes |
| DETL-05 | Detail page shows next open day date | Display `open_day` field (ISO date string) formatted in German locale |
| DETL-06 | Detail page shows data freshness indicator | Display `last_updated` field with relative freshness (e.g., "Aktualisiert am 26.03.2026") |
| DEPL-01 | Site is a static Rust/Leptos WASM SPA compiled with Trunk | Trunk build produces dist/ with index.html + app.wasm; CSR mode via `leptos = { features = ["csr"] }` |
| DEPL-02 | Site is deployed to GitHub Pages | GitHub Actions workflow: install Rust nightly + wasm32 target + Trunk, build, deploy dist/ to gh-pages |
| DEPL-03 | School data embedded at build time (YAML -> JSON -> include_str!) | build.rs reads data/schools/*.yaml with serde-saphyr, serializes to JSON, writes to OUT_DIR, include_str! in app |
</phase_requirements>

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| leptos | 0.8.17 | Reactive WASM SPA framework (CSR mode) | Project constraint; fine-grained reactivity; official CSR support via `features = ["csr"]` |
| leptos_router | 0.8.12 | Client-side path-based routing | Bundled with leptos ecosystem; supports `path!()` macro, `use_query_map`, `use_params`, `use_navigate` |
| leptos_meta | 0.8.x | HTML `<head>` management (title, meta tags) | Included in start-trunk template; sets page title per route |
| reactive_stores | 0.8.x | Store derive macro for fine-grained struct reactivity | Official leptos companion crate; enables field-level reactivity on filter/app state without nested signals |
| serde | 1.x | Serialization framework | Universal Rust standard; required by serde_json |
| serde_json | 1.x | JSON deserialization at runtime in WASM | Parses embedded schools.json; smaller and faster than YAML parsing in WASM |
| wasm-bindgen | 0.2.x | Rust-to-JS FFI | Required for browser API access; Trunk manages version automatically |
| web-sys | 0.3.x | Browser DOM/API bindings | Window, Document, Storage, Location access from Rust |
| trunk | 0.17.x | WASM build tool and dev server | Official recommended tool for Leptos CSR; auto-manages wasm-bindgen and wasm-opt; produces dist/ for deployment |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| leptos-use | 0.18.x | Utility hooks (debounce, media query, local storage) | `use_debounce_fn` for filter changes; `use_media_query` for responsive breakpoints; `use_local_storage` for future favorites (Phase 5) |
| serde-saphyr | 0.0.10 | YAML deserialization (build step only) | build.rs reads `data/schools/*.yaml`; build-dependency only, not in WASM binary |
| console_log | 1.x | Logging in WASM via browser console | Dev-time debugging; included in start-trunk template |
| console_error_panic_hook | 0.1.x | Panic messages to browser console | Dev-time debugging; prevents silent WASM panics |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| reactive_stores (Store derive) | Plain signals + provide_context | Store gives field-level reactivity on filter state struct without manual signal wiring; more ergonomic for complex state |
| leptos-use `use_media_query` | Pure CSS media queries | CSS-only is simpler for layout; leptos-use adds Rust-side responsive logic if needed for conditional rendering |
| serde-saphyr (build) | serde_yaml | serde_yaml is unmaintained since 2023; serde-saphyr is actively maintained |
| Custom CSS | Tailwind CSS | Tailwind adds build complexity (PostCSS/CLI); custom CSS keeps WASM bundle concerns simpler; D-03 says "utility CSS approach or minimal CSS" |

### Installation

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup default nightly

# Install Trunk
cargo install --locked trunk

# Create project (manual, not cargo-generate -- we need custom structure)
cargo init --name berlin-gymnasien
```

### Cargo.toml

```toml
[package]
name = "berlin-gymnasien"
version = "0.1.0"
edition = "2024"

[dependencies]
leptos = { version = "0.8", features = ["csr"] }
leptos_router = "0.8"
leptos_meta = "0.8"
reactive_stores = "0.8"
leptos-use = { version = "0.18", features = ["storage"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Window", "Document", "HtmlElement"] }
console_log = "1"
console_error_panic_hook = "0.1"
log = "0.4"

[build-dependencies]
serde = { version = "1", features = ["derive"] }
serde-saphyr = "0.0.10"
serde_json = "1"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
```

### rust-toolchain.toml

```toml
[toolchain]
channel = "nightly"
targets = ["wasm32-unknown-unknown"]
```

## Architecture Patterns

### Recommended Project Structure

```
amman/
  Cargo.toml
  Cargo.lock
  rust-toolchain.toml
  Trunk.toml
  index.html                     # Trunk entry point
  build.rs                       # YAML -> JSON at compile time
  public/
    styles.css                   # Global styles (or .scss)
    favicon.ico
  src/
    main.rs                      # mount_to_body, hydrate entry
    app.rs                       # App component with Router
    models.rs                    # School, FilterState structs (serde + Store)
    state.rs                     # Global state provider (Store<AppState>)
    components/
      mod.rs
      school_card.rs             # Individual school card
      school_list.rs             # Filterable grid of cards
      filter_panel.rs            # Sidebar/mobile filter UI
      filter_chips.rs            # Active filter badges + clear
      sort_controls.rs           # Sort dropdown
    pages/
      mod.rs
      listing.rs                 # Main listing page (filters + grid)
      detail.rs                  # School detail page
      not_found.rs               # 404 fallback
  data/
    schools/                     # 106 YAML files (Phase 1 output)
  .github/
    workflows/
      deploy.yml                 # GitHub Actions: build + deploy to Pages
```

### Pattern 1: Build-Time Data Embedding

**What:** `build.rs` reads all YAML school files, converts to a single JSON string, and writes to `OUT_DIR`. The app uses `include_str!` to embed the JSON at compile time.

**When to use:** Always -- this is the data loading strategy for the entire SPA.

**Example:**

```rust
// build.rs
use std::fs;
use std::path::Path;

// SchoolRecord mirrors pipeline/models.py but only the fields needed for display
#[derive(serde::Serialize, serde::Deserialize)]
struct SchoolRecord {
    school_id: String,
    name: String,
    district: String,
    last_updated: String,
    // ... all other fields as Option<T>
}

fn main() {
    let schools_dir = Path::new("data/schools");
    let mut schools: Vec<SchoolRecord> = Vec::new();

    for entry in fs::read_dir(schools_dir).expect("data/schools/ must exist") {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |e| e == "yaml") {
            let content = fs::read_to_string(entry.path()).unwrap();
            let school: SchoolRecord = serde_saphyr::from_str(&content).unwrap();
            schools.push(school);
        }
    }

    schools.sort_by(|a, b| a.name.cmp(&b.name));

    let json = serde_json::to_string(&schools).unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    fs::write(Path::new(&out_dir).join("schools.json"), json).unwrap();

    // Tell Cargo to re-run if YAML files change
    println!("cargo:rerun-if-changed=data/schools/");
}
```

```rust
// src/models.rs
const SCHOOLS_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/schools.json"));

pub fn load_schools() -> Vec<School> {
    serde_json::from_str(SCHOOLS_JSON).expect("embedded schools.json must be valid")
}
```

### Pattern 2: Path-Based Routing with 404.html Fallback

**What:** Use standard `leptos_router` path-based routing. Copy `index.html` to `404.html` in the build step so GitHub Pages serves the SPA for all paths.

**When to use:** Always -- this replaces the hash routing approach from D-17/D-18 because Leptos 0.8 does not support hash routing.

**Example:**

```rust
// src/app.rs
use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route, ParentRoute};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    // Provide global state
    provide_context(Store::new(AppState::default()));

    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=ListingPage />
                <Route path=path!("/school/:id") view=DetailPage />
            </Routes>
        </Router>
    }
}
```

**GitHub Actions deploy step:**
```yaml
- name: Copy index.html to 404.html for SPA routing
  run: cp dist/index.html dist/404.html
```

### Pattern 3: Filter State via URL Query Parameters

**What:** Filter state is encoded in URL query parameters so filtered views are shareable. Changes to filters programmatically navigate to update the URL.

**When to use:** Implements D-09 (shareable filter URLs) without hash routing.

**Example:**

```rust
// In listing page component
use leptos_router::hooks::{use_query_map, use_navigate};

#[component]
pub fn ListingPage() -> impl IntoView {
    let query = use_query_map();
    let navigate = use_navigate();

    // Read filters from URL
    let districts = move || {
        query.read()
            .get("district")
            .map(|d| d.split(',').map(String::from).collect::<Vec<_>>())
            .unwrap_or_default()
    };

    // Update URL when filter changes
    let set_district_filter = move |new_districts: Vec<String>| {
        let params = build_query_string(&new_districts, /* other filters */);
        navigate(&format!("/?{params}"), NavigateOptions {
            replace: true,
            ..Default::default()
        });
    };
    // ...
}
```

### Pattern 4: Global State with Store

**What:** A `#[derive(Store)]` struct holds app-wide state (loaded schools, current sort, UI state). Provided via context at the Router level.

**When to use:** For state that multiple components need to read (school data, active sort order).

**Example:**

```rust
use reactive_stores::Store;

#[derive(Clone, Debug, Store, Default)]
pub struct AppState {
    pub schools: Vec<School>,
    pub sort_by: SortField,
    pub sort_asc: bool,
}
```

### Pattern 5: Derived Signals for Filtered/Sorted Schools

**What:** A `Memo` or derived signal computes the filtered+sorted school list reactively from URL query params and school data. The listing component reads this derived value.

**When to use:** Core pattern for the listing page. Avoids recomputing filters on every render.

**Example:**

```rust
let filtered_schools = Memo::new(move |_| {
    let all_schools = app_state.schools();
    let current_districts = districts();
    let current_profiles = profiles();
    // ... apply all filters with AND logic
    all_schools.iter()
        .filter(|s| current_districts.is_empty() || current_districts.contains(&s.district))
        .filter(|s| /* profile filter */)
        .filter(|s| /* grundstaendig filter */)
        .cloned()
        .collect::<Vec<_>>()
});
```

### Anti-Patterns to Avoid

- **Embedding YAML parser in WASM:** Never use serde-saphyr or any YAML parser in the runtime WASM binary. Convert YAML to JSON at build time in `build.rs`. YAML parsers add significant binary size.
- **Fetching data at runtime:** Do not fetch schools.json via HTTP. Use `include_str!` to embed it. The SPA has no server; all data must be compile-time embedded.
- **Re-rendering entire list on filter change:** Use `<For>` component with keyed iteration. Memo the filtered list. Do not rebuild the entire DOM on every filter toggle.
- **Putting filter state only in signals (not URL):** Filter state MUST be in URL query params for shareability (D-09). Signals are derived from URL, not the other way around.
- **Using `cargo-leptos` instead of `trunk`:** cargo-leptos is for SSR full-stack apps. Trunk is the correct tool for CSR-only WASM SPAs.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| YAML parsing in build step | Custom parser | serde-saphyr 0.0.10 | Full YAML spec compliance; serde integration; maintained |
| URL query param serialization | Manual string building | `leptos_router::hooks::use_query_map` + `use_navigate` | Handles encoding, reactivity, browser history correctly |
| Responsive breakpoint detection | Manual `window.innerWidth` checks | `leptos-use::use_media_query` or CSS media queries | Handles resize events, SSR safety, reactive updates |
| Debounced filter updates | Manual timer management | `leptos-use::use_debounce_fn` | Handles cleanup, edge cases, reactive integration |
| WASM binary optimization | Manual wasm-opt invocation | Trunk's built-in `data-wasm-opt="z"` | Trunk auto-downloads and runs wasm-opt; zero config |
| Dev server with hot reload | Manual setup | `trunk serve` | Built-in live reload, WASM rebuilds, port management |
| GitHub Pages deployment | Manual rsync/push | `actions/deploy-pages@v4` + `actions/upload-pages-artifact@v4` | Official GitHub Actions; handles permissions, artifacts, atomic deploys |

**Key insight:** Trunk manages the entire WASM build pipeline (wasm-bindgen, wasm-opt, asset bundling, dev server). Do not invoke these tools manually.

## Common Pitfalls

### Pitfall 1: Hash Routing Assumed Available (Leptos Does NOT Support It)

**What goes wrong:** CONTEXT.md decision D-17 specifies hash routing (`/#/`). Leptos 0.8's router does NOT implement hash-based routing. The `LocationProvider` only supports the History API (`popstate`). Attempting to implement hash routing would require forking/extending the router.
**Why it happens:** Hash routing is a common SPA pattern. The Leptos issue tracker has an open feature request (#2184) but it remains unimplemented as of March 2026.
**How to avoid:** Use path-based routing (`/`, `/school/:id`) with the 404.html fallback pattern. Copy `index.html` to `404.html` in the Trunk build output. This is the official Leptos recommendation for GitHub Pages.
**Warning signs:** Routes work in `trunk serve` but fail on GitHub Pages with actual 404 errors.

### Pitfall 2: WASM Bundle Bloat from Dependencies

**What goes wrong:** An unoptimized WASM binary can reach 3-5 MB. Adding regex, full serde features, or YAML parsers to the runtime binary inflates size dramatically.
**Why it happens:** Rust's monomorphization and serde codegen produce large output without size optimization.
**How to avoid:** Set release profile (`opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`) from day one. Use `data-wasm-opt="z"` in index.html. Never add the regex crate. Keep serde-saphyr as build-dependency only.
**Warning signs:** `trunk build --release` output exceeds 1.5 MB for the .wasm file before compression.

### Pitfall 3: GitHub Pages Public URL Mismatch

**What goes wrong:** Assets fail to load on GitHub Pages because paths assume root (`/`) but the site is served from a subdirectory (`/amman/`).
**Why it happens:** GitHub Pages for project repos serves from `https://username.github.io/repo-name/`. All asset URLs must be relative to this base.
**How to avoid:** Always pass `--public-url /amman/` to `trunk build`. Verify in `Trunk.toml` or CI workflow. Test with `trunk serve --public-url /amman/` locally.
**Warning signs:** Blank page on GitHub Pages; browser console shows 404 for .wasm and .js files.

### Pitfall 4: Filter State Lost on Navigation

**What goes wrong:** User applies filters, clicks a school card to view details, presses back, and filters are reset.
**Why it happens:** If filters are stored only in component-local signals, they are destroyed when the listing component unmounts during navigation.
**How to avoid:** Store ALL filter state in URL query parameters. The listing page reads filters from URL on mount. Back navigation restores the URL (and thus filters) automatically via browser history. Use `NavigateOptions { replace: true }` when updating filter params to avoid polluting history stack.
**Warning signs:** Pressing browser back after viewing a detail page shows unfiltered listing.

### Pitfall 5: build.rs Not Re-Running on YAML Changes

**What goes wrong:** You update a school YAML file but the embedded JSON in the WASM binary doesn't change.
**Why it happens:** Cargo only re-runs build.rs when its dependencies change, not when data files change, unless explicitly told.
**How to avoid:** Add `println!("cargo:rerun-if-changed=data/schools/");` in build.rs. For individual file tracking, iterate the directory and emit a `rerun-if-changed` for each file.
**Warning signs:** Stale school data after editing YAML files; need `cargo clean` to see changes.

### Pitfall 6: Nightly Toolchain Instability

**What goes wrong:** A nightly Rust update breaks compilation. The start-trunk template uses nightly by default for WASM.
**Why it happens:** Nightly Rust can have breaking changes. Some Leptos features require nightly.
**How to avoid:** Pin a specific nightly date in `rust-toolchain.toml` (e.g., `channel = "nightly-2026-03-15"`). Update deliberately, not automatically. Alternatively, try stable first -- Leptos 0.8 CSR may work on stable (the `Params` derive requires nightly for non-Option fields, but this can be worked around).
**Warning signs:** CI fails after a Rust nightly update with unrelated errors.

### Pitfall 7: Multi-Value Query Params Limitation

**What goes wrong:** Leptos router's `ParamsMap` maps `String -> String`. Using `?district=Mitte&district=Pankow` (repeated keys) loses all but the last value.
**Why it happens:** The router's query map is single-valued per key. This is a known issue (#2882).
**How to avoid:** Encode multi-select filters as comma-separated values: `?district=Mitte,Pankow&profile=MINT,bilingual_english`. Parse with `.split(',')` on the Rust side.
**Warning signs:** Only one filter value is applied when multiple are selected.

## Code Examples

### Entry Point (main.rs)

```rust
// Source: Leptos start-trunk template pattern
use leptos::prelude::*;

mod app;
mod models;
mod state;
mod components;
mod pages;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    mount_to_body(app::App);
}
```

### index.html (Trunk Entry Point)

```html
<!DOCTYPE html>
<html lang="de" dir="ltr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Berliner Gymnasien Vergleich</title>
    <link data-trunk rel="css" href="public/styles.css" />
    <link data-trunk rel="icon" href="public/favicon.ico" />
    <link data-trunk rel="rust" data-wasm-opt="z" />
</head>
<body></body>
</html>
```

### Rust Data Model (mirrors Python SchoolRecord)

```rust
// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct School {
    pub school_id: String,
    pub name: String,
    pub district: String,
    pub last_updated: String,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub coords: Option<Coords>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub traeger: Option<String>,
    #[serde(default)]
    pub student_count: Option<u32>,
    #[serde(default)]
    pub teacher_count: Option<u32>,
    #[serde(default)]
    pub accepts_after_4th_grade: Option<bool>,
    #[serde(default)]
    pub profile: Vec<String>,
    #[serde(default)]
    pub ganztag: Option<bool>,
    #[serde(default)]
    pub languages: Vec<LanguageEntry>,
    #[serde(default)]
    pub open_day: Option<String>,
    #[serde(default)]
    pub admission_requirements: Option<AdmissionRequirements>,
    #[serde(default)]
    pub abitur_average: Option<f64>,
    #[serde(default)]
    pub ratings: std::collections::HashMap<String, RatingEntry>,
    #[serde(default)]
    pub completeness_score: Option<f64>,
    #[serde(default)]
    pub data_sources: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Coords {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LanguageEntry {
    pub name: String,
    pub from_grade: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AdmissionRequirements {
    pub notendurchschnitt: Option<f64>,
    pub oversubscribed: Option<bool>,
    pub selection_criteria: Option<String>,
    pub probeunterricht: Option<bool>,
    pub entrance_test: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RatingEntry {
    pub source: String,
    pub score: Option<f64>,
    #[serde(default = "default_scale_min")]
    pub scale_min: f64,
    #[serde(default = "default_scale_max")]
    pub scale_max: f64,
    pub review_count: Option<u32>,
    pub retrieved: String,
}

fn default_scale_min() -> f64 { 1.0 }
fn default_scale_max() -> f64 { 5.0 }
```

### GitHub Actions Workflow

```yaml
# .github/workflows/deploy.yml
name: Deploy to GitHub Pages

on:
  push:
    branches: ["main"]

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: "pages"
  cancel-in-progress: false

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Set up Rust nightly
        uses: dtolnay/rust-toolchain@nightly
        with:
          targets: wasm32-unknown-unknown

      - name: Install Trunk
        run: cargo install --locked trunk

      - name: Build
        run: trunk build --release --public-url /amman/

      - name: Copy 404.html for SPA routing
        run: cp dist/index.html dist/404.html

      - name: Setup Pages
        uses: actions/configure-pages@v4

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v4
        with:
          path: dist

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    needs: build
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `serde_yaml` (dtolnay) | `serde-saphyr` 0.0.10 | 2023 (serde_yaml unmaintained) | Must use serde-saphyr for YAML in build step |
| Leptos 0.6 signals only | Leptos 0.8 + `reactive_stores` `Store` derive | 0.7+ (late 2024) | Field-level reactivity on structs without manual signal wrapping |
| Manual wasm-bindgen + wasm-opt | Trunk auto-manages both | Trunk 0.16+ | Zero-config WASM toolchain management |
| Hash routing for GitHub Pages SPAs | 404.html fallback (path-based) | Standard since GitHub Pages inception | Hash routing never implemented in Leptos; 404.html is the standard |
| Leptos 0.6 `create_signal` | Leptos 0.8 `signal()` function | 0.7 (late 2024) | API change: `let (count, set_count) = signal(0)` instead of `create_signal` |
| Leptos 0.6 `<Router>` props | Leptos 0.8 `path!()` macro + `<Routes>` | 0.7-0.8 (2024-2025) | Route paths defined with type-safe `path!()` macro |

**Deprecated/outdated:**
- `serde_yaml`: unmaintained since 2023; use `serde-saphyr`
- `create_signal`, `create_memo`, `create_effect`: replaced with `signal()`, `Memo::new()`, `Effect::new()` in Leptos 0.7+
- `<Router mode=...>`: Leptos 0.8 router does not have a mode prop; there is only the History API LocationProvider

## Open Questions

1. **Stable vs Nightly Rust for Leptos 0.8 CSR**
   - What we know: The start-trunk template defaults to nightly. Leptos docs mention nightly is needed for some features.
   - What's unclear: Whether stable Rust can compile a basic Leptos 0.8 CSR app. The `Params` derive may require nightly only for non-Option fields.
   - Recommendation: Start with nightly (matches template). Pin a specific date. Try stable later if desired.

2. **leptos-use storage feature interaction with CSR**
   - What we know: `use_local_storage` requires the `storage` feature flag. It is designed to handle SSR/CSR differences.
   - What's unclear: Whether there are any gotchas using `use_local_storage` in a pure CSR (no hydration) context.
   - Recommendation: Enable the `storage` feature. For Phase 2, local storage is not required (filter state is in URL). Use it in Phase 5 for favorites.

3. **Profile badge color scheme**
   - What we know: CONTEXT.md suggests MINT=green, music=purple, sports=blue, bilingual=orange, altsprachlich=red.
   - What's unclear: Whether additional profile values exist in the data beyond these.
   - Recommendation: Define a color map for known profiles. Use a neutral gray for unknown/other profiles. Claude's discretion per CONTEXT.md.

4. **Exact `public-url` value for GitHub Pages**
   - What we know: Decision D-22 says `--public-url /amman/`. This assumes the repo is named `amman` under the user's GitHub account.
   - What's unclear: Whether the repo URL will change.
   - Recommendation: Use `/amman/` as specified. This can be changed in one place (Trunk.toml or CI workflow) if the repo name changes. Use `${GITHUB_REPOSITORY#*/}` in CI for flexibility.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| rustup | Rust toolchain management | NOT INSTALLED | -- | Must install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| rustc (nightly) | Compiling Leptos WASM | NOT INSTALLED | -- | Installed via rustup |
| wasm32-unknown-unknown target | WASM compilation target | NOT INSTALLED | -- | `rustup target add wasm32-unknown-unknown` |
| trunk | WASM build tool | NOT INSTALLED | -- | `cargo install --locked trunk` |
| wasm-bindgen CLI | Rust-JS FFI (auto-managed by Trunk) | NOT INSTALLED | -- | Trunk downloads automatically |
| wasm-opt (binaryen) | WASM binary optimization | NOT INSTALLED | -- | Trunk downloads automatically |
| GitHub Actions | CI/CD deployment | AVAILABLE (GitHub repo) | -- | -- |
| data/schools/*.yaml | School data input | AVAILABLE | 106 files | -- |

**Missing dependencies with no fallback:**
- Rust toolchain (rustup + nightly + wasm32 target) -- MUST be installed before any frontend work
- Trunk -- MUST be installed for building and dev serving

**Missing dependencies with fallback:**
- wasm-bindgen CLI and wasm-opt: Trunk auto-downloads these; no manual install needed

**NOTE:** The GitHub Actions CI/CD workflow will install all tools independently. Local development requires manual installation of rustup and trunk only.

## Project Constraints (from CLAUDE.md)

- **Tech stack**: Rust + Leptos static SPA compiled to WASM (project constraint)
- **Deployment**: GitHub Pages -- no server-side rendering or API
- **Data format**: YAML files, one per school, in `data/schools/`
- **Build tool**: Trunk (not cargo-leptos) for CSR/WASM SPA
- **German UI**: All user-facing text in German
- **GSD Workflow**: Use `/gsd:execute-phase` for implementation work

## Sources

### Primary (HIGH confidence)
- [Leptos Book: Deploying CSR Apps](https://book.leptos.dev/deployment/csr.html) -- 404.html pattern, Trunk build, GitHub Pages workflow
- [Leptos Book: Binary Size Optimization](https://book.leptos.dev/deployment/binary_size.html) -- release profile, wasm-opt
- [Leptos Book: Router - Params and Queries](https://book.leptos.dev/router/18_params_and_queries.html) -- use_query_map, use_params, Params derive
- [Leptos Book: Global State Management](https://book.leptos.dev/15_global_state.html) -- Store derive, provide_context, URL as state
- [Leptos start-trunk template](https://github.com/leptos-rs/start-trunk) -- project structure, Cargo.toml, index.html, Trunk.toml
- [leptos_router docs.rs](https://docs.rs/leptos_router/latest/leptos_router/) -- Router, Routes, Route, use_navigate API
- [leptos-use docs](https://leptos-use.rs/) -- use_debounce_fn, use_media_query, use_local_storage
- [serde-saphyr on crates.io](https://crates.io/crates/serde-saphyr) -- 0.0.10, YAML deserialization
- [Leptos router example](https://github.com/leptos-rs/leptos/blob/main/examples/router/src/lib.rs) -- path!() macro, Router setup, ParentRoute

### Secondary (MEDIUM confidence)
- [GitHub Pages SPA routing discussion](https://github.com/orgs/community/discussions/64096) -- confirmed 404.html is standard approach
- [Leptos hash routing issue #2184](https://github.com/leptos-rs/leptos/issues/2184) -- confirmed NOT implemented as of March 2026
- [Trunk official site](https://trunkrs.dev/) -- version 0.17.x, auto wasm-opt, install methods
- [Leptos router multi-value query issue #2882](https://github.com/leptos-rs/leptos/issues/2882) -- ParamsMap is single-valued; use comma-separated encoding

### Tertiary (LOW confidence)
- [GitHub Actions Leptos deploy gist](https://gist.github.com/DougAnderson444/e2f4ee87bdbe71a2eb0984a5958bde66) -- older workflow example; adapted for current Actions versions

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all crate versions verified on crates.io/docs.rs; Leptos 0.8.17 is latest
- Architecture: HIGH -- patterns from official Leptos book, start-trunk template, and router examples
- Pitfalls: HIGH -- hash routing limitation confirmed via GitHub issue; binary size guidance from official docs
- Data embedding: HIGH -- build.rs + include_str! is standard Rust pattern; schema mirrors verified Python model
- Routing override: HIGH -- hash routing absence confirmed via official issue tracker and docs.rs API surface

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (Leptos 0.8.x is stable; next major version not imminent)
