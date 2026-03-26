"""
pipeline/run.py — Berlin Gymnasien pipeline orchestrator.

Entry point for all justfile targets. Dispatches to step modules.
Usage: uv run python pipeline/run.py --step {seed|scrape|enrich|validate|write|all} [--force]
"""
from __future__ import annotations

import argparse
import asyncio
import logging
import os
import sys
from pathlib import Path

from ruamel.yaml import YAML


def load_config(config_path: str) -> dict:
    """Load pipeline.yaml config. Returns empty dict if file missing."""
    p = Path(config_path)
    if not p.exists():
        # Try relative path from pipeline/ directory
        p = Path("pipeline") / config_path
    if not p.exists():
        logging.warning("Config file not found at %s — using defaults", config_path)
        return {}
    yaml = YAML()
    return dict(yaml.load(p) or {})


def check_api_key(step: str) -> None:
    """Fail fast if ANTHROPIC_API_KEY missing for steps that need it (D-27)."""
    if step in ("enrich", "all"):
        key = os.environ.get("ANTHROPIC_API_KEY")
        if not key:
            print(
                "ERROR: ANTHROPIC_API_KEY environment variable is not set.\n"
                "Set it before running the enrich step:\n"
                "  export ANTHROPIC_API_KEY=sk-ant-...\n"
                "Secrets are never stored in files (D-27).",
                file=sys.stderr,
            )
            sys.exit(1)


async def step_seed(config: dict) -> list[dict]:
    from pipeline.seed import run_seed
    return await run_seed(config)


async def step_scrape(config: dict) -> None:
    """Structured scrape step. WFS already covers all structured fields.
    This step is a no-op placeholder for future additional structured sources."""
    print("Scrape step: WFS covers all structured fields. No additional scraping needed.")


async def step_enrich(schools: list[dict], config: dict, force: bool) -> list[dict]:
    from pipeline.agent import run_enrich
    return await run_enrich(schools, config, force)


def step_validate_and_merge(
    structured_list: list[dict],
    agent_list: list[dict],
) -> tuple:
    from pipeline.validate import merge_all
    return merge_all(structured_list, agent_list)


def step_write(records, all_conflicts: list[dict], config: dict) -> None:
    from pipeline.writer import write_all, write_conflicts
    data_dir = config.get("data_dir", "data/schools")
    write_all(records, data_dir)
    write_conflicts(all_conflicts, data_dir)


def load_schools_index(config: dict) -> list[dict]:
    """Load schools_index.yaml. Fails with clear error if not found (run seed first)."""
    index_path = Path(config.get("index_file", "data/schools_index.yaml"))
    if not index_path.exists():
        print(
            f"ERROR: {index_path} not found. Run 'just seed' first.",
            file=sys.stderr,
        )
        sys.exit(1)
    yaml = YAML()
    return list(yaml.load(index_path) or [])


def load_enriched_cache(schools: list[dict], config: dict) -> list[dict]:
    """Load cached enrichment results for schools that have been enriched."""
    import json
    cache_dir = config.get("cache_dir", "pipeline/cache")
    results = []
    for school in schools:
        sid = school["school_id"]
        p = Path(cache_dir) / f"{sid}.json"
        if p.exists():
            try:
                cached = json.loads(p.read_text(encoding="utf-8"))
                results.append({"school_id": sid, **cached.get("data", {})})
            except (json.JSONDecodeError, OSError):
                pass
    return results


async def run_all(config: dict, force: bool) -> None:
    """Run complete pipeline: seed → scrape → enrich → validate+merge → write."""
    print("=== Berlin Gymnasien Pipeline ===")

    # Step 1: Seed
    print("\n[1/4] Seeding school list from WFS + Eckdaten XLSX...")
    schools = await step_seed(config)

    # Step 2: Scrape (no-op)
    print("\n[2/4] Structured scrape...")
    await step_scrape(config)

    # Step 3: Enrich
    print(f"\n[3/4] Enriching {len(schools)} schools with Claude agents...")
    agent_results = await step_enrich(schools, config, force)

    # Step 4: Validate + Merge + Write
    print("\n[4/4] Validating, merging, and writing YAML files...")
    records, all_conflicts = step_validate_and_merge(schools, agent_results)
    step_write(records, all_conflicts, config)

    print(f"\n=== Pipeline complete: {len(records)} school YAML files in {config.get('data_dir', 'data/schools')} ===")


async def main() -> None:
    parser = argparse.ArgumentParser(description="Berlin Gymnasien scraping pipeline")
    parser.add_argument(
        "--step",
        choices=["seed", "scrape", "enrich", "validate", "write", "all"],
        required=True,
        help="Pipeline step to run",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="Force re-enrichment even for cached schools (D-09)",
    )
    parser.add_argument(
        "--config",
        default="pipeline/pipeline.yaml",
        help="Path to pipeline config YAML (default: pipeline/pipeline.yaml)",
    )
    args = parser.parse_args()

    config = load_config(args.config)

    # Configure logging
    log_level = config.get("log_level", "INFO")
    logging.basicConfig(
        level=getattr(logging, log_level, logging.INFO),
        format="%(asctime)s %(levelname)s %(name)s: %(message)s",
    )

    # Fail fast on missing API key for steps that need it
    check_api_key(args.step)

    if args.step == "seed":
        await step_seed(config)

    elif args.step == "scrape":
        await step_scrape(config)

    elif args.step == "enrich":
        schools = load_schools_index(config)
        await step_enrich(schools, config, args.force)

    elif args.step == "validate":
        schools = load_schools_index(config)
        agent_results = load_enriched_cache(schools, config)
        records, all_conflicts = step_validate_and_merge(schools, agent_results)
        print(f"Validation: {len(records)} valid records, {len(all_conflicts)} conflicts")

    elif args.step == "write":
        schools = load_schools_index(config)
        agent_results = load_enriched_cache(schools, config)
        records, all_conflicts = step_validate_and_merge(schools, agent_results)
        step_write(records, all_conflicts, config)

    elif args.step == "all":
        await run_all(config, args.force)


if __name__ == "__main__":
    asyncio.run(main())
