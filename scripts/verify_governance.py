#!/usr/bin/env python3
"""Run formula-vault, external-resolution, and xtask governance gates."""
from __future__ import annotations

import argparse
import os
import subprocess
import sys
from pathlib import Path


def run(command: list[str], repo: Path) -> int:
    print("governance command:", " ".join(command), flush=True)
    completed = subprocess.run(
        command,
        cwd=repo,
        env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"},
    )
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
        print(
            f"governance verification failed: repository does not exist: {repo}",
            file=sys.stderr,
        )
        return 2

    formula_vault = repo / "scripts/verify_formula_vault_resolutions.py"
    if not formula_vault.is_file():
        print(
            f"governance verification failed: missing verifier: {formula_vault}",
            file=sys.stderr,
        )
        return 2
    result = run([sys.executable, str(formula_vault), "--repo", str(repo)], repo)
    if result != 0:
        return result

    external_verifiers = sorted((repo / "scripts").glob("verify_external_m07_*.py"))
    if not external_verifiers:
        print(
            "governance verification failed: no external M07 resolution verifier found",
            file=sys.stderr,
        )
        return 2
    for verifier in external_verifiers:
        result = run([sys.executable, str(verifier), "--repo", str(repo)], repo)
        if result != 0:
            return result

    return run(["cargo", "run", "-p", "xtask", "--", "verify", "--all"], repo)


if __name__ == "__main__":
    raise SystemExit(main())
