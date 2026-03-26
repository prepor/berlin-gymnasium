# Phase 4: Travel Time Routing - Research

**Researched:** 2026-03-26
**Domain:** Client-side geocoding + travel time matrix API integration in Rust/Leptos WASM SPA
**Confidence:** MEDIUM

## Summary

This phase adds address geocoding via Photon (photon.komoot.io), travel time calculation via Valhalla FOSSGIS (valhalla1.openstreetmap.de), and sort-by-commute to the existing listing page. The main technical challenge is making async HTTP requests from WASM (the project has no async fetch code yet), parsing JSON responses, and integrating the results into Leptos's reactive signal system.

**Critical finding:** The Valhalla matrix API (`/sources_to_targets`) does NOT support `multimodal` (transit) costing. The matrix endpoint only supports `auto`, `bicycle`, `pedestrian`, and `bikeshare`. Transit routing is only available via the individual `/route` endpoint with `"costing": "multimodal"`. This means the decision D-12 (three parallel matrix requests including multimodal) needs adjustment. The recommended approach is: use the matrix API for `pedestrian` and `auto`, and use `bicycle` as the third mode instead of transit -- or accept N individual `/route` calls for transit (one per school, ~90 sequential or batched requests).

**Primary recommendation:** Use `pedestrian` and `auto` via the matrix endpoint (one call each), and `bicycle` as the third practical mode. Add a note to users that public transit times are not available via this API. If transit is essential, batch individual `/route` calls with `"costing": "multimodal"` (slower, ~90 requests, but functional).

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Use Photon (photon.komoot.io) for address geocoding -- CORS-enabled, no API key, browser-safe
- **D-02:** Do NOT use Nominatim directly from browser (autocomplete prohibited, User-Agent requirement)
- **D-03:** Geocode request format: `https://photon.komoot.io/api/?q={address}&lat=52.52&lon=13.405&limit=5&lang=de` (biased toward Berlin)
- **D-04:** Return top 5 suggestions for user to pick from (dropdown after typing)
- **D-05:** Text input with submit button -- user types address, hits Enter or clicks button
- **D-06:** After submit, show top 5 Photon results as a dropdown for disambiguation
- **D-07:** Selected address stored in URL query param (`?from=lat,lng`) so it's shareable
- **D-08:** "Clear address" button to remove travel time data and return to normal listing
- **D-09:** German UI labels: "Adresse eingeben", "Fahrzeit berechnen", "Adresse loesche"
- **D-10:** Use Valhalla FOSSGIS server: `https://valhalla1.openstreetmap.de/sources_to_targets`
- **D-11:** Matrix endpoint: one source (user address) -> N targets (all filtered school coords) in a single API call
- **D-12:** Three costing modes in parallel requests: `pedestrian` (walking), `multimodal` (transit), `auto` (car) **-- RESEARCH FINDING: multimodal NOT supported on matrix endpoint, see Pitfall 1**
- **D-13:** Parse response `sources_to_targets[0][i].time` (seconds) for each school
- **D-14:** If Valhalla CORS fails at runtime, show error message "Reisezeit-Berechnung nicht verfügbar" and gracefully degrade
- **D-15:** When address is set, each school card shows a travel time row with walk/transit/car times
- **D-16:** Travel times rounded to nearest minute
- **D-17:** Sort-by-travel-time added as a sort option (walk, transit, or car -- user picks which mode to sort by)
- **D-18:** Schools without coords show "Keine Fahrzeit verfuegbar"
- **D-19:** Debounce geocoding requests by 500ms after user stops typing
- **D-20:** Cache travel time results per address (in-memory HashMap)
- **D-21:** Show loading spinner on cards while travel time is being fetched
- **D-22:** If individual school has no route, show "--" instead of time

### Claude's Discretion
- Exact wasm-bindgen fetch implementation (web-sys Fetch API vs gloo-net)
- Whether to create a separate travel_time.rs module or inline in listing
- Spinner/loading indicator styling
- Whether Photon suggestions appear as overlay dropdown or inline list

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| TRVL-01 | User can enter their address to calculate travel time to each school | Photon geocoding API verified with CORS support; address input -> geocode -> lat/lng extraction pattern documented |
| TRVL-02 | Travel time shown for walking, public transport, and car | Valhalla matrix API supports pedestrian + auto; multimodal NOT supported on matrix (see Pitfall 1); bicycle available as alternative or individual /route calls for transit |
| TRVL-03 | User can sort/filter schools by travel time from their address | SortField enum extension pattern documented; filter_and_sort function integration point identified |

</phase_requirements>

## Standard Stack

### Core (New Dependencies)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| wasm-bindgen-futures | 0.4.x | Convert JS Promises to Rust Futures; provides `spawn_local` | Required for async HTTP from WASM; already used by leptos internally but must be an explicit dependency for `spawn_local` |
| gloo-net | 0.6.x | Ergonomic HTTP fetch for WASM | Cleaner than raw web-sys Request/RequestInit/Response. Provides `Request::post().json().send().await?.json().await?` pattern. Built on web-sys internally |
| serde | 1.x (already present) | JSON deserialization of API responses | Already in Cargo.toml |
| serde_json | 1.x (already present) | JSON serialization for request bodies | Already in Cargo.toml |

### Already Available (No Changes Needed)

| Library | Version | Purpose |
|---------|---------|---------|
| leptos | 0.8.x | Reactive framework -- signals, effects, components |
| leptos-use | 0.18.x | `use_debounce_fn` for geocoding input debounce |
| web-sys | 0.3.x | Browser APIs (Window, etc.) -- may need additional features |
| js-sys | 0.3.x | JS object construction (already used in map.rs) |
| wasm-bindgen | 0.2.x | Core WASM-JS FFI |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| gloo-net | Raw web-sys fetch (Request/RequestInit/Response) | More boilerplate but zero new dependencies; project already uses web-sys. gloo-net adds ~5 types of ergonomics |
| gloo-net | reqwest with wasm feature | Much heavier; designed for full HTTP client, overkill for 3-4 fetch calls |

**Recommendation:** Use gloo-net. It is the standard lightweight WASM fetch crate. The existing codebase already uses js-sys/web-sys for Leaflet interop, and gloo-net adds minimal compile overhead while dramatically reducing fetch boilerplate.

**Installation (additions to Cargo.toml):**
```toml
[dependencies]
wasm-bindgen-futures = "0.4"
gloo-net = { version = "0.6", features = ["json"] }
```

If choosing raw web-sys instead of gloo-net, add these features to the existing web-sys entry:
```toml
web-sys = { version = "0.3", features = [
    "Window", "Document", "HtmlElement",
    "Headers", "Request", "RequestInit", "RequestMode", "Response"
] }
```

## Architecture Patterns

### Recommended Module Structure

```
src/
├── models.rs              # Add TravelTime struct, extend SortField enum
├── services/
│   ├── mod.rs             # Module declarations
│   ├── geocoding.rs       # Photon API: geocode_address() -> Vec<GeocodingResult>
│   └── routing.rs         # Valhalla API: fetch_travel_times() -> HashMap<String, TravelTimes>
├── components/
│   ├── address_input.rs   # New: AddressInput component with debounced search + dropdown
│   ├── school_card.rs     # Extend: add travel time row when data available
│   ├── sort_controls.rs   # Extend: add travel time sort options
│   └── ...existing...
├── pages/
│   └── listing.rs         # Integrate: address signal, travel time state, pass to cards
└── ...
```

### Pattern 1: Async Fetch from WASM

**What:** Making HTTP requests from Rust/WASM using gloo-net
**When to use:** All API calls (Photon geocoding, Valhalla routing)

```rust
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

#[derive(Deserialize)]
struct ValhallaResponse {
    sources_to_targets: Vec<Vec<MatrixEntry>>,
}

#[derive(Deserialize)]
struct MatrixEntry {
    distance: Option<f64>,
    time: Option<f64>,  // seconds
    from_index: usize,
    to_index: usize,
}

async fn fetch_matrix(
    origin_lat: f64,
    origin_lng: f64,
    targets: &[(f64, f64)],
    costing: &str,
) -> Result<ValhallaResponse, String> {
    let body = serde_json::json!({
        "sources": [{"lat": origin_lat, "lon": origin_lng}],
        "targets": targets.iter().map(|(lat, lng)| {
            serde_json::json!({"lat": lat, "lon": lng})
        }).collect::<Vec<_>>(),
        "costing": costing,
        "verbose": true,
    });

    let resp = Request::post("https://valhalla1.openstreetmap.de/sources_to_targets")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .map_err(|e| format!("Request build error: {:?}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {:?}", e))?;

    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }

    resp.json::<ValhallaResponse>()
        .await
        .map_err(|e| format!("Parse error: {:?}", e))
}
```

### Pattern 2: Reactive Signal Integration with spawn_local

**What:** Connecting async API results to Leptos reactive signals
**When to use:** When user selects an address and travel times need to load

```rust
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

// In the ListingPage component:
let (travel_times, set_travel_times) = signal::<Option<HashMap<String, TravelTimes>>>(None);
let (travel_loading, set_travel_loading) = signal(false);

// When user selects address:
let on_address_selected = move |lat: f64, lng: f64| {
    set_travel_loading.set(true);
    set_travel_times.set(None);

    let schools = filtered_schools.get();
    spawn_local(async move {
        let targets: Vec<(f64, f64)> = schools.iter()
            .filter_map(|s| s.coords.as_ref().map(|c| (c.lat, c.lng)))
            .collect();

        // Fire pedestrian + auto in parallel via join
        let (walk_result, car_result) = futures::join!(
            fetch_matrix(lat, lng, &targets, "pedestrian"),
            fetch_matrix(lat, lng, &targets, "auto"),
        );

        // Merge results into HashMap<school_id, TravelTimes>
        let mut times = HashMap::new();
        // ... merge walk_result and car_result ...

        set_travel_times.set(Some(times));
        set_travel_loading.set(false);
    });
};
```

**Note:** `futures::join!` requires the `futures` crate (or use `futures-lite`). Alternatively, fire two separate `spawn_local` blocks.

### Pattern 3: Photon Geocoding with Debounce

**What:** Debounced address search using leptos-use
**When to use:** Address input field

```rust
use leptos_use::use_debounce_fn_with_arg;

let (suggestions, set_suggestions) = signal::<Vec<GeocodingResult>>(vec![]);

let debounced_geocode = use_debounce_fn_with_arg(
    move |query: String| {
        spawn_local(async move {
            if query.len() < 3 { return; }
            match geocode_address(&query).await {
                Ok(results) => set_suggestions.set(results),
                Err(_) => set_suggestions.set(vec![]),
            }
        });
    },
    500.0,
);
```

### Pattern 4: URL Query Param for Address (existing pattern extension)

**What:** Store selected address coords in URL for shareability
**When to use:** After user selects geocoded address

The existing `build_query_string` function in listing.rs needs extension to include `from=lat,lng`. The pattern is already established -- add a new parameter.

### Anti-Patterns to Avoid

- **Don't block on async in component body:** Always use `spawn_local` or `Resource`/`Action` for async work. Never `.block_on()` in WASM.
- **Don't rebuild the school list for sorting by travel time:** Instead, pass travel time data alongside schools and sort in the memo. The filtered_schools Memo should incorporate travel times when sorting by commute.
- **Don't make N individual API calls when matrix works:** The matrix endpoint handles 1-to-N routing in one call. Only fall back to individual calls for multimodal if transit is required.
- **Don't store travel times inside the School struct:** Travel times are ephemeral per-session data, not part of the school data model. Use a separate signal/map keyed by school_id.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| HTTP fetch from WASM | Custom web-sys Request/Response wrapper | gloo-net `Request` | Handles headers, body serialization, response parsing, CORS mode |
| Input debouncing | setTimeout + manual cleanup | leptos-use `use_debounce_fn_with_arg` | Handles cleanup on component unmount, timer management |
| JSON serialization of request body | Manual string concatenation | serde_json `json!()` macro | Type-safe, handles escaping, nested structures |
| Promise-to-Future conversion | Manual JsFuture wrapping | wasm-bindgen-futures `spawn_local` | Standard pattern, handles WASM executor |

## Common Pitfalls

### Pitfall 1: Valhalla Matrix Does NOT Support Transit/Multimodal (CRITICAL)

**What goes wrong:** The CONTEXT.md decision D-12 specifies `multimodal` as one of three parallel matrix requests. The Valhalla matrix API documentation explicitly states: "multimodal costing is not supported for the time-distance matrix service at this time."
**Why it happens:** The Valhalla turn-by-turn `/route` endpoint does support multimodal costing. It is easy to assume all endpoints share the same costing options.
**How to avoid:** Two options:
  1. **Replace transit with bicycle** in the matrix call. Show walk/bike/car instead of walk/transit/car. Update UI labels accordingly.
  2. **Use individual /route calls for transit.** Fire ~90 individual POST requests to `https://valhalla1.openstreetmap.de/route` with `"costing": "multimodal"` and `date_time` parameter. Extract `trip.summary.time` from each response. This is slower but provides real transit times.
**Recommendation:** Start with option 1 (bicycle). Transit via individual routes can be added as an enhancement if performance is acceptable.
**Warning signs:** 4xx errors from Valhalla when sending `"costing": "multimodal"` to the matrix endpoint.
**Source:** [Valhalla Matrix API Reference](https://valhalla.github.io/valhalla/api/matrix/api-reference/)

### Pitfall 2: Photon CORS May Be Unreliable

**What goes wrong:** Photon's public API at photon.komoot.io has had a complicated CORS history. GitHub issues #81 and #117 show CORS was added to the codebase but the public server's behavior has been inconsistent over the years.
**Why it happens:** The CORS implementation was merged but the public instance may or may not have it enabled depending on deployment configuration.
**How to avoid:** Test the actual CORS behavior from the browser before building the full feature. If CORS fails, the fallback is Nominatim's `/search` endpoint (which does return CORS headers) -- but Nominatim prohibits autocomplete-style usage, so only use it for explicit submit-triggered searches.
**Warning signs:** `Access-Control-Allow-Origin` header missing from Photon responses in browser dev tools.
**Source:** [Photon CORS Issue #81](https://github.com/komoot/photon/issues/81), [Issue #117](https://github.com/komoot/photon/issues/117)

### Pitfall 3: Valhalla FOSSGIS CORS Unverified

**What goes wrong:** The Valhalla FOSSGIS server at valhalla1.openstreetmap.de may not return CORS headers for POST requests (which are "preflighted" by browsers and require OPTIONS handling).
**Why it happens:** GET requests with simple headers may bypass CORS preflight, but POST with `Content-Type: application/json` triggers a preflight OPTIONS request. The server must handle this.
**How to avoid:** Decision D-14 already covers graceful degradation. But also: Valhalla accepts GET requests with `?json={}` query parameter. If POST CORS fails, try GET with URL-encoded JSON as fallback.
**Warning signs:** Network errors on the OPTIONS preflight request in browser dev tools.
**Workaround:** `GET https://valhalla1.openstreetmap.de/sources_to_targets?json={...}` (URL-encode the JSON body).

### Pitfall 4: Missing wasm-bindgen-futures Dependency

**What goes wrong:** `spawn_local` is not available, compilation fails with "unresolved import" error.
**Why it happens:** The current Cargo.toml does not include `wasm-bindgen-futures`. While leptos uses it internally, the user crate must declare it explicitly to use `spawn_local`.
**How to avoid:** Add `wasm-bindgen-futures = "0.4"` to Cargo.toml dependencies before any async code.

### Pitfall 5: Travel Time Sorting Breaks Memo Dependencies

**What goes wrong:** The current `filtered_schools` Memo only depends on filter signals and the sort field. If travel time data is stored in a separate signal, changing it does not trigger a re-sort.
**Why it happens:** Leptos Memos only re-compute when their tracked signals change. A HashMap<String, TravelTimes> signal must be read inside the Memo for it to become a dependency.
**How to avoid:** Read the travel_times signal inside the `filter_and_sort` function (or the Memo closure) when the sort field is a travel time variant. This registers the dependency.

### Pitfall 6: Valhalla Response Index Mismatch

**What goes wrong:** The `sources_to_targets[0][i].to_index` maps to the position in the `targets` array, not to a school_id. If you filter out schools without coords before building the targets array, the index-to-school mapping must be maintained separately.
**Why it happens:** Valhalla knows nothing about school IDs; it only returns indices.
**How to avoid:** Build a parallel `Vec<String>` of school IDs in the same order as the targets array. Use `to_index` to look up the school_id.

### Pitfall 7: Multimodal Requires date_time Parameter

**What goes wrong:** If you choose to use individual `/route` calls for transit, requests without `date_time` will fail or return nonsensical results because transit schedules are time-dependent.
**Why it happens:** The multimodal algorithm requires knowing when the trip starts to look up transit schedules.
**How to avoid:** Always include `"date_time": {"type": 1, "value": "YYYY-MM-DDThh:mm"}` with the current time when making multimodal route requests.

## Code Examples

### Photon Geocoding Request and Response Parsing

```rust
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PhotonResponse {
    pub features: Vec<PhotonFeature>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PhotonFeature {
    pub geometry: PhotonGeometry,
    pub properties: PhotonProperties,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PhotonGeometry {
    pub coordinates: Vec<f64>,  // [lng, lat] -- NOTE: GeoJSON is lng,lat not lat,lng!
}

#[derive(Clone, Debug, Deserialize)]
pub struct PhotonProperties {
    pub name: Option<String>,
    pub street: Option<String>,
    pub housenumber: Option<String>,
    pub postcode: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

impl PhotonFeature {
    /// Extract lat/lng from GeoJSON coordinates (which are [lng, lat])
    pub fn lat(&self) -> f64 {
        self.geometry.coordinates[1]
    }
    pub fn lng(&self) -> f64 {
        self.geometry.coordinates[0]
    }
    /// Build a display label for the dropdown
    pub fn display_label(&self) -> String {
        let parts: Vec<&str> = [
            self.properties.street.as_deref(),
            self.properties.housenumber.as_deref(),
            self.properties.postcode.as_deref(),
            self.properties.city.as_deref(),
        ]
        .iter()
        .filter_map(|p| *p)
        .collect();
        if parts.is_empty() {
            self.properties.name.clone().unwrap_or_default()
        } else {
            parts.join(", ")
        }
    }
}

pub async fn geocode_address(query: &str) -> Result<Vec<PhotonFeature>, String> {
    let url = format!(
        "https://photon.komoot.io/api/?q={}&lat=52.52&lon=13.405&limit=5&lang=de",
        js_sys::encode_uri_component(query)
    );
    let resp = gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Geocoding error: {:?}", e))?;
    let data: PhotonResponse = resp.json()
        .await
        .map_err(|e| format!("Parse error: {:?}", e))?;
    Ok(data.features)
}
```

### Valhalla Matrix Request

```rust
#[derive(Clone, Debug, Deserialize)]
pub struct ValhallaMatrixResponse {
    pub sources_to_targets: Vec<Vec<ValhallaMatrixEntry>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ValhallaMatrixEntry {
    pub time: Option<f64>,     // seconds, null if unreachable
    pub distance: Option<f64>, // km
    pub to_index: usize,
}

pub async fn fetch_travel_times_matrix(
    origin_lat: f64,
    origin_lng: f64,
    targets: &[(f64, f64)],  // (lat, lng) pairs
    costing: &str,           // "pedestrian", "auto", or "bicycle"
) -> Result<Vec<Option<u32>>, String> {
    let body = serde_json::json!({
        "sources": [{"lat": origin_lat, "lon": origin_lng}],
        "targets": targets.iter().map(|(lat, lng)| {
            serde_json::json!({"lat": lat, "lon": lng})
        }).collect::<Vec<_>>(),
        "costing": costing,
        "verbose": true,
    });

    let resp = gloo_net::http::Request::post(
        "https://valhalla1.openstreetmap.de/sources_to_targets"
    )
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .map_err(|e| format!("Build error: {:?}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {:?}", e))?;

    if !resp.ok() {
        return Err(format!("Valhalla HTTP {}", resp.status()));
    }

    let data: ValhallaMatrixResponse = resp.json()
        .await
        .map_err(|e| format!("Parse error: {:?}", e))?;

    // Extract times from first (only) source row, convert seconds -> minutes
    let times: Vec<Option<u32>> = data.sources_to_targets
        .first()
        .map(|row| {
            row.iter()
                .map(|entry| entry.time.map(|t| (t / 60.0).round() as u32))
                .collect()
        })
        .unwrap_or_default();

    Ok(times)
}
```

### TravelTimes Data Model

```rust
/// Travel times for a single school from the user's address.
#[derive(Clone, Debug, Default)]
pub struct TravelTimes {
    pub walk_minutes: Option<u32>,
    pub bike_minutes: Option<u32>,  // or transit_minutes if using /route
    pub car_minutes: Option<u32>,
}
```

### Extending SortField Enum

```rust
#[derive(Clone, Debug, PartialEq, Default)]
pub enum SortField {
    #[default]
    Name,
    District,
    StudentCount,
    TravelTimeWalk,
    TravelTimeBike,   // or TravelTimeTransit
    TravelTimeCar,
}

impl SortField {
    pub fn from_query(s: &str) -> Self {
        match s {
            "district" => SortField::District,
            "students" => SortField::StudentCount,
            "travel_walk" => SortField::TravelTimeWalk,
            "travel_bike" => SortField::TravelTimeBike,
            "travel_car" => SortField::TravelTimeCar,
            _ => SortField::Name,
        }
    }

    pub fn to_query(&self) -> &'static str {
        match self {
            SortField::Name => "name",
            SortField::District => "district",
            SortField::StudentCount => "students",
            SortField::TravelTimeWalk => "travel_walk",
            SortField::TravelTimeBike => "travel_bike",
            SortField::TravelTimeCar => "travel_car",
        }
    }

    pub fn is_travel_time(&self) -> bool {
        matches!(self, SortField::TravelTimeWalk | SortField::TravelTimeBike | SortField::TravelTimeCar)
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| serde_yaml for YAML | serde-saphyr | 2023 | Already handled in build.rs |
| Raw web-sys fetch | gloo-net | 2023+ | Ergonomic fetch for WASM, widely adopted |
| leptos create_resource (0.6) | Resource/Action (0.8) | 2024 | API changed but spawn_local still works for imperative async |
| Nominatim geocoding from browser | Photon (komoot) | Ongoing | Nominatim prohibits autocomplete usage from browsers |

## Open Questions

1. **Valhalla FOSSGIS CORS for POST requests**
   - What we know: The server exists and is publicly accessible. GET requests likely work. POST with application/json triggers CORS preflight (OPTIONS).
   - What's unclear: Whether the OPTIONS preflight is handled by the FOSSGIS server. Some mailing list posts suggest CORS was configured, but no definitive proof for POST.
   - Recommendation: Try POST first. If it fails, fall back to GET with `?json={}` URL parameter. Both are documented by Valhalla.

2. **Photon CORS on photon.komoot.io**
   - What we know: CORS support was merged into the Photon codebase. The `-cors-any` flag exists. Issue #117 shows `Access-Control-Allow-Origin: *` was confirmed in curl tests.
   - What's unclear: Whether the current production deployment at photon.komoot.io has CORS enabled.
   - Recommendation: Test from browser. If it fails, fall back to Nominatim `/search?q=...&format=json` (which does support CORS) for submit-only geocoding (no autocomplete).

3. **Transit mode feasibility**
   - What we know: Matrix API does not support multimodal. Individual /route calls do support it. FOSSGIS server may or may not have GTFS data for Berlin (VBB).
   - What's unclear: Whether the FOSSGIS server actually has Berlin transit data loaded, and whether ~90 individual /route calls are fast enough for acceptable UX.
   - Recommendation: Ship v1 with bicycle instead of transit. Transit can be added as a follow-up if the /route approach proves viable.

## Project Constraints (from CLAUDE.md)

- **Tech stack:** Rust + Leptos -- static SPA compiled to WASM (no server-side proxy for APIs)
- **Deployment:** GitHub Pages -- all API calls must be client-side with CORS
- **Data format:** YAML files, one per school, in `data/schools/` (travel times are NOT stored, they are computed at runtime)
- **Build tool:** trunk for CSR/WASM
- **YAML parsing:** serde-saphyr (build-time only, not relevant to this phase)
- **Routing API:** Valhalla (FOSSGIS server) -- confirmed in CLAUDE.md
- **Geocoding:** Photon/Nominatim -- confirmed in CLAUDE.md
- **Existing patterns:** URL query params for all state, memo-based filtering/sorting, German UI labels, web-sys + js-sys for browser interop
- **No GSD bypass:** Changes must go through GSD workflow

## Sources

### Primary (HIGH confidence)
- [Valhalla Matrix API Reference](https://valhalla.github.io/valhalla/api/matrix/api-reference/) - Confirmed matrix does NOT support multimodal; documented request/response format
- [Valhalla Turn-by-Turn API Reference](https://valhalla.github.io/valhalla/api/turn-by-turn/api-reference/) - Confirmed /route supports multimodal with date_time; response has trip.summary.time in seconds
- [Photon API Reference (DeepWiki)](https://deepwiki.com/komoot/photon/5-api-reference) - Full endpoint documentation, GeoJSON response schema, query parameters
- [wasm-bindgen Fetch Example](https://rustwasm.github.io/docs/wasm-bindgen/examples/fetch.html) - Official pattern for web-sys fetch from WASM
- [leptos-use use_debounce_fn](https://leptos-use.rs/utilities/use_debounce_fn.html) - Debounce API with arg variant for input handling

### Secondary (MEDIUM confidence)
- [Photon CORS Issue #117](https://github.com/komoot/photon/issues/117) - CORS headers confirmed in curl; may or may not be active on current deployment
- [Photon CORS Issue #81](https://github.com/komoot/photon/issues/81) - CORS implementation history
- [Photon Deprecation Issue #516](https://github.com/komoot/photon/issues/516) - Migration from .de to .io domain
- [gloo-net on crates.io](https://crates.io/crates/gloo-net) - Lightweight WASM HTTP client
- [Valhalla FOSSGIS Demo](https://valhalla.openstreetmap.de/) - Public server with API at valhalla1.openstreetmap.de

### Tertiary (LOW confidence)
- FOSSGIS server transit/GTFS data availability for Berlin -- claimed in search results but not verified against the actual server
- gloo-net 0.6 exact version -- crates.io page did not render, version from search results

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - well-established crates (wasm-bindgen-futures, gloo-net, serde)
- Architecture: HIGH - patterns follow existing codebase conventions (signals, memos, URL params)
- API integration (Valhalla matrix): HIGH - official docs clearly document capabilities and limitations
- API integration (Photon): MEDIUM - CORS status on public endpoint is not definitively confirmed
- API integration (transit): LOW - matrix does not support it; /route feasibility for ~90 schools unverified
- Pitfalls: HIGH - identified from official documentation and codebase analysis

**Research date:** 2026-03-26
**Valid until:** 2026-04-26 (stable APIs, unlikely to change)
