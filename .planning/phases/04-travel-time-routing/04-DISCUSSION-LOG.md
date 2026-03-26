# Phase 4: Travel Time Routing - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-26
**Phase:** 04-travel-time-routing
**Areas discussed:** Geocoding, Address input UX, Travel time API, Display, Performance
**Mode:** --auto (all recommended defaults selected)

---

## Geocoding Service

| Option | Description | Selected |
|--------|-------------|----------|
| Photon (komoot) | CORS-enabled, no API key, browser-safe | auto |
| Nominatim direct | Prohibited from browser (User-Agent, no autocomplete) | |
| Google Geocoding | Paid, requires API key | |

**User's choice:** [auto] Photon (recommended)

## Address Input UX

| Option | Description | Selected |
|--------|-------------|----------|
| Manual entry + submit | Type address, submit, pick from suggestions | auto |
| Real-time autocomplete | Dropdown appears as you type | |

**User's choice:** [auto] Manual entry + submit (recommended — simpler)

## Travel Time Display

| Option | Description | Selected |
|--------|-------------|----------|
| Row on school card | Walk/transit/car times shown per card | auto |
| Separate column | Additional column in listing | |
| Expandable section | Click to reveal per school | |

**User's choice:** [auto] Row on school card (recommended — visible at a glance)

## API Call Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Valhalla matrix (sources_to_targets) | Single call per mode for all schools | auto |
| Per-school individual calls | 106 separate calls per mode | |

**User's choice:** [auto] Matrix endpoint (recommended — efficient)

## Rate Limiting & Error Handling

| Option | Description | Selected |
|--------|-------------|----------|
| Debounce + cache + error toast | 500ms debounce, in-memory cache, graceful degradation | auto |
| No special handling | Simple fetch, let it fail | |

**User's choice:** [auto] Debounce + cache + error toast (recommended)

## Claude's Discretion

- Fetch implementation approach
- Module structure
- Loading indicator styling
- Photon suggestion presentation

## Deferred Ideas

None.
