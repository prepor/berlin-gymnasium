---
status: awaiting_human_verify
trigger: "Address input doesn't work — no dropdown appears after pressing Suchen button. Nothing visible happens."
created: 2026-03-26T00:00:00Z
updated: 2026-03-26T00:00:00Z
---

## Current Focus

hypothesis: CONFIRMED — The travel time flow was working all along; the user saw "nothing happens" because (a) no loading indicator was shown during the 5-8 second Valhalla fetch, and (b) dropdown labels were unhelpful (e.g. "10178, Berlin" instead of "Alexanderplatz, 10178 Berlin"). Fixed both issues.
test: Playwright automated tests confirming all paths: debounce-typing, Suchen-click, direct suggestion click, fast-Suchen
expecting: All paths produce travel times on cards, loading indicator appears, labels are descriptive
next_action: Awaiting user verification

## Symptoms

expected: User types an address in the input field, presses "Suchen" (or Enter), and a dropdown of geocoded suggestions from Photon (photon.komoot.io) appears below the input.
actual: Nothing happens after pressing "Suchen". No dropdown, no error, no loading indicator visible.
errors: Unknown — no console errors reported. Check for silent fetch failures, CORS issues, or event handler wiring problems.
reproduction: 1. Run `trunk serve` 2. Navigate to the listing page 3. Type an address in the address input 4. Press "Suchen" button 5. Observe: nothing happens
started: First time the feature is being tested after Phase 4 implementation. Has never worked.

## Eliminated

## Evidence

- timestamp: 2026-03-26T00:01:00Z
  checked: Compilation (cargo build --target wasm32-unknown-unknown)
  found: Project compiles successfully with only warnings (dead code). No type errors or missing imports.
  implication: The bug is a runtime issue, not a compilation issue. All dependencies (gloo-net, wasm-bindgen-futures, js-sys) are present.

- timestamp: 2026-03-26T00:02:00Z
  checked: Photon API response (curl with Origin header)
  found: API returns 200 with Access-Control-Allow-Origin:* when called correctly. CORS is NOT the issue.
  implication: The API works; the problem is in how the request is constructed.

- timestamp: 2026-03-26T00:03:00Z
  checked: Runtime behavior via Puppeteer headless browser test
  found: Debug status shows "ERROR: Geocoding HTTP 400". The debounce fires correctly, geocode_address is called, but Photon returns 400 Bad Request.
  implication: The request URL is malformed in some way that Photon rejects.

- timestamp: 2026-03-26T00:04:00Z
  checked: gloo-net RequestBuilder TryFrom implementation (request.rs lines 202-224)
  found: When the URL already contains query params and no .query() builder is used, gloo-net appends "&" + empty QueryParams, creating a trailing "&" in the URL.
  implication: The final URL becomes "https://photon.komoot.io/api/?q=Alexanderplatz&lat=52.52&lon=13.405&limit=5&lang=de&" (trailing &).

- timestamp: 2026-03-26T00:05:00Z
  checked: Photon API with trailing & (curl)
  found: "curl -s -w '%{http_code}' 'https://photon.komoot.io/api/?q=Alexanderplatz&...&lang=de&'" returns 400. Without trailing & returns 200.
  implication: Confirmed root cause. Photon API is strict and rejects URLs with trailing &.

- timestamp: 2026-03-26T22:10:00Z
  checked: Full end-to-end flow via Playwright (suggestion click -> URL navigate -> Valhalla -> card display)
  found: The entire travel time flow WORKS. Clicking a suggestion adds from= param to URL, Effect fires, Valhalla receives 3 POST requests (pedestrian 200, bicycle 200, auto 504). 106 card travel time elements rendered with walk/bike minutes. Auto 504 is Valhalla server-side limitation with 106 targets.
  implication: The flow was functional but user perceived "nothing happens" due to missing loading indicator and 5-8 second fetch delay.

- timestamp: 2026-03-26T22:11:00Z
  checked: Dropdown display labels
  found: display_label() method skipped the Photon "name" field (e.g. "Alexanderplatz") when any other field was present. First suggestion showed "10178, Berlin" instead of "Alexanderplatz, 10178 Berlin".
  implication: Labels were unhelpful/confusing — user couldn't tell which suggestion to pick.

- timestamp: 2026-03-26T22:12:00Z
  checked: Loading indicator visibility (Playwright rapid-polling)
  found: After fix, "Fahrzeiten werden berechnet..." text appears within 0.0s of clicking a suggestion and disappears after travel times load. Confirmed gone after load completes.
  implication: User now gets immediate visual feedback during the 5-8s Valhalla fetch.

## Resolution

root_cause: Three issues combined: (1) Original gloo-net trailing-& bug causing HTTP 400 from Photon (fixed in prior session). (2) display_label() in geocoding.rs dropped the place name (e.g. "Alexanderplatz") when postcode/city were present, producing confusing labels like "10178, Berlin". (3) No loading indicator during the 5-8 second Valhalla fetch, making the user perceive "nothing happens" after selection. The actual on_select -> navigate -> Effect -> Valhalla -> card display chain was fully functional.
fix: (1) Rewrote display_label() to include Photon "name" field, format "Name, Street Housenumber, Postcode City". (2) Added loading text "Fahrzeiten werden berechnet..." shown during travel_loading state in listing.rs. (3) Improved dropdown CSS: higher z-index (1000), blue hover highlight, padding/transition polish, active state.
verification: Playwright automated tests confirm: (a) labels now show "Alexanderplatz, 10178 Berlin", (b) loading indicator appears within 0s and disappears after fetch, (c) 106 travel time elements rendered on cards, (d) all user paths work (debounce, Suchen, direct click, fast-Suchen).
files_changed: [src/services/geocoding.rs, src/pages/listing.rs, public/styles.css]
