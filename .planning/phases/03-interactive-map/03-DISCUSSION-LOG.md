# Phase 3: Interactive Map - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 03-interactive-map
**Areas discussed:** Map library, Map-listing integration, Pin styling, Popup content, Default map view
**Mode:** --auto (all recommended defaults selected)

---

## Map Library Approach

| Option | Description | Selected |
|--------|-------------|----------|
| Raw wasm-bindgen Leaflet.js | Direct JS interop, guaranteed to work | auto |
| leptos-leaflet | Idiomatic but 0.8 compat unverified | |
| MapLibre GL | WebGL-based, heavier, no Leptos wrapper | |

**User's choice:** [auto] Raw wasm-bindgen (recommended — leptos-leaflet 0.8 compat is a known blocker)

---

## Map-Listing Integration

| Option | Description | Selected |
|--------|-------------|----------|
| Tab toggle | Switch between list and map views | auto |
| Side-by-side | List + map simultaneously | |
| Embedded map above list | Map always visible, list below | |

**User's choice:** [auto] Tab toggle (recommended — simpler, better mobile UX)

---

## Pin Styling

| Option | Description | Selected |
|--------|-------------|----------|
| Color-coded by profile | Matches card badge colors | auto |
| Uniform pins | All same color | |
| Size-coded by student count | Larger = more students | |

**User's choice:** [auto] Color-coded by profile (recommended — visual consistency with cards)

---

## Popup Content

| Option | Description | Selected |
|--------|-------------|----------|
| Name + district + badges + detail link | Key info + navigation | auto |
| Name only + click to detail | Minimal | |
| Full card preview in popup | Rich but heavy | |

**User's choice:** [auto] Name + district + badges + detail link (recommended — enough to decide)

---

## Default Map View

| Option | Description | Selected |
|--------|-------------|----------|
| Berlin center, zoom 11, fitBounds on filter | Shows all districts, adjusts to pins | auto |
| Tight fitBounds only | No default center | |
| User location centered | Requires geolocation permission | |

**User's choice:** [auto] Berlin center + fitBounds (recommended — predictable, no permissions needed)

---

## Claude's Discretion

- wasm-bindgen binding approach
- Module structure (map.rs vs inline)
- Pin icon sizing
- Optional "locate me" button

## Deferred Ideas

None.
