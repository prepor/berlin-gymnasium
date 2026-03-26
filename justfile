# Berlin Gymnasien pipeline task runner
# Requires: ANTHROPIC_API_KEY env var for enrich step

default: all

# Seed: fetch WFS + Eckdaten XLSX, write data/schools_index.yaml
seed:
    uv run python pipeline/run.py --step seed

# Structured scrape (currently a no-op; WFS covers structured fields)
scrape:
    uv run python pipeline/run.py --step scrape

# Enrich: Claude agent enrichment for all schools
enrich:
    uv run python pipeline/run.py --step enrich

# Validate: Pydantic validation of all school records
validate:
    uv run python pipeline/run.py --step validate

# Write: serialize validated records to data/schools/*.yaml + changelog
write:
    uv run python pipeline/run.py --step write

# Run full pipeline: seed -> scrape -> enrich -> validate -> write
all:
    uv run python pipeline/run.py --step all

# Force full re-run even for cached schools
force:
    uv run python pipeline/run.py --step all --force
