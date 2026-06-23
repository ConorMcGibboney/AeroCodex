#!/usr/bin/env python3
"""Run the formula-vault resolution verifier and the existing xtask governance gate."""
from __future__ import annotations

import argparse
import os
import subprocess
import sys
from pathlib import Path


def run(command: list[str], repo: Path) -> int:
    print("governance command:", " ".join(command), flush=True)
    completed = subprocess.run(command, cwd=repo, env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"})
    print(f"governance exit code: {completed.returncode}", flush=True)
    return completed.returncode


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path("."), help="repository root")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo = args.repo.resolve()
    if not repo.is_dir():
        print(f"governance verification failed: repository does not exist: {repo}", file=sys.stderr)
        return 2
    resolution = repo / "scripts/verify_formula_vault_resolutions.py"
    if not resolution.is_file():
        print(f"governance verification failed: missing verifier: {resolution}", file=sys.stderr)
        return 2
    first = run([sys.executable, str(resolution), "--repo", str(repo)], repo)
    if first != 0:
        return first
    return run(["cargo", "run", "-p", "xtask", "--", "verify", "--all"], repo)


if __name__ == "__main__":
    raise SystemExit(main())
