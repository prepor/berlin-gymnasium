# Berlin Gymnasien

A reproducible data pipeline and static website for comparing all Berlin Gymnasien (~90+ schools). The pipeline scrapes official data sources and uses Claude AI agents to enrich school profiles. The website is a Rust/Leptos SPA deployed to GitHub Pages.

## Prerequisites

- **Python 3.13+** — required by the pipeline
- **uv** — Python package manager (`curl -LsSf https://astral.sh/uv/install.sh | sh`)
- **just** — task runner (`brew install just` or `cargo install just`)
- **ANTHROPIC_API_KEY** — required for the enrich step (Claude agent web research)

## Setup

```bash
git clone https://github.com/prepor/berlin-gymnasium.git
cd berlin-gymnasium

# Install Python dependencies
cd pipeline && uv sync && cd ..

# Set your Anthropic API key (required for enrich step)
export ANTHROPIC_API_KEY=sk-ant-...
```

## Running the Pipeline

Run the full pipeline (seed → scrape → enrich → validate → write):

```bash
just all
```

Run individual steps:

```bash
just seed      # Fetch school list from Berlin Open Data WFS + Eckdaten XLSX
just scrape    # Structured scrape (currently a no-op; WFS covers structured fields)
just enrich    # Claude agent web research for all schools (requires ANTHROPIC_API_KEY)
just validate  # Validate and merge structured + agent data
just write     # Write validated school YAML files
```

Force full re-run (ignore cache, re-enrich all schools):

```bash
just force
```

## Pipeline Steps

| Step       | What it does                                                     | Output                           |
|------------|------------------------------------------------------------------|----------------------------------|
| `seed`     | Fetches all Berlin Gymnasien from GDI WFS, merges student/teacher counts from Eckdaten XLSX | `data/schools_index.yaml` |
| `scrape`   | Structured scrape (no-op; WFS covers all structured fields)      | —                                |
| `enrich`   | Claude agent web research: profiles, languages, admission, ratings, Abitur averages | `pipeline/cache/{school_id}.json` |
| `validate` | Merges structured + agent data, runs Pydantic validation (structured source always wins) | In-memory                        |
| `write`    | Writes per-school YAML files, appends field-level changelog, regenerates conflicts.md | `data/schools/*.yaml`, `data/CHANGELOG.md`, `data/conflicts.md` |

## Configuration

All defaults are in `pipeline/pipeline.yaml`. Edit this file to change pipeline behavior.

| Key | Default | Description |
|-----|---------|-------------|
| `wfs_url` | `https://gdi.berlin.de/services/wfs/schulen` | Berlin Open Data WFS endpoint |
| `eckdaten_url` | `https://www.berlin.de/...` | Eckdaten XLSX URL for student/teacher counts |
| `include_private_schools` | `true` | Include private Gymnasien in output |
| `batch_size` | `8` | Number of schools per Claude agent batch |
| `semaphore_limit` | `8` | Max concurrent agent batches |
| `max_web_searches_per_school` | `5` | Max web search calls per school |
| `cache_dir` | `pipeline/cache` | Directory for per-school enrichment cache |
| `data_dir` | `data/schools` | Output directory for school YAML files |
| `index_file` | `data/schools_index.yaml` | Path to school index file |
| `log_level` | `INFO` | Logging level (DEBUG, INFO, WARNING, ERROR) |

**Secrets** are never stored in `pipeline.yaml`. Always set `ANTHROPIC_API_KEY` via environment variable.

## Data Output

After a full pipeline run:

- **`data/schools_index.yaml`** — Flat list of all schools with IDs, coordinates, and basic info
- **`data/schools/*.yaml`** — One YAML file per school with complete structured data (one file per BSN school number)
- **`data/CHANGELOG.md`** — Appended on each run; shows field-level changes (via deepdiff) from prior run
- **`data/conflicts.md`** — Regenerated each run; shows where agent data contradicted the structured source (WFS always wins)
- **`pipeline/cache/*.json`** — Per-school enrichment cache; delete to force re-enrichment

Example school file (`data/schools/12345.yaml`):

```yaml
school_id: "12345"
name: "Einstein-Gymnasium"
district: Reinickendorf
address: Musterstraße 1, 12345 Berlin
coords:
  lat: 52.567
  lng: 13.345
accepts_after_4th_grade: false
profile:
  - MINT
  - bilingual_english
languages:
  - name: Englisch
    from_grade: 5
  - name: Französisch
    from_grade: 7
ganztag: true
data_sources:
  - wfs_berlin
  - agent_research
last_updated: "2026-03-26"
completeness_score: 0.85
```
