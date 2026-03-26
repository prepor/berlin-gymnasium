# Berlin Gymnasien pipeline task runner
# Requires: ANTHROPIC_API_KEY env var for enrich step

default: all

# Seed: fetch WFS + Eckdaten XLSX, write data/schools_index.yaml
seed:
    uv run --project pipeline python pipeline/run.py --step seed

# Structured scrape (currently a no-op; WFS covers structured fields)
scrape:
    uv run --project pipeline python pipeline/run.py --step scrape

# Enrich: Claude agent enrichment for all schools
enrich:
    uv run --project pipeline python pipeline/run.py --step enrich

# Photos: dedicated photo enrichment for schools missing images
photos:
    uv run --project pipeline python pipeline/run.py --step photos

# Photos (force): re-search photos for all schools
photos-force:
    uv run --project pipeline python pipeline/run.py --step photos --force

# Validate: Pydantic validation of all school records
validate:
    uv run --project pipeline python pipeline/run.py --step validate

# Write: serialize validated records to data/schools/*.yaml + changelog
write:
    uv run --project pipeline python pipeline/run.py --step write

# Run full pipeline: seed -> scrape -> enrich -> validate -> write
all:
    uv run --project pipeline python pipeline/run.py --step all

# Force full re-run even for cached schools
force:
    uv run --project pipeline python pipeline/run.py --step all --force
