#!/usr/bin/env python3
"""Run the fail-closed AeroCodex Beta 1 automated qualification gate.

The gate restores the repository's no-root-Cargo.lock policy by deleting only a
verified untracked transient root Cargo.lock before it captures the source
baseline. It then snapshots the current tracked and untracked source state into a
temporary Git repository, executes repository, CLI, packaging, archive, and
tamper checks, and writes machine-readable JSON, JUnit XML, and a full text log.
It does not modify tracked source files.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
import shutil
import subprocess
import sys
import tempfile
import time
import xml.etree.ElementTree as ET
from datetime import datetime, timezone
from pathlib import Path, PurePosixPath
from typing import Any

GATE_SCHEMA_VERSION = "1.0"
VALIDATION_STATUS = "research_required"
SUPPORTED_FORMULA_COUNT = 10
SELF_CHECK_CASES = 14
DEFAULT_TIMEOUT_SECONDS = 1800


class GateError(RuntimeError):
    """Raised when a fail-closed gate condition is not satisfied."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise GateError(message)


def utc_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def safe_timestamp() -> str:
    return datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")


def stable_json(value: Any) -> str:
    return json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n"


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def command_text(command: list[str]) -> str:
    return subprocess.list2cmdline(command) if os.name == "nt" else " ".join(shlex_quote(item) for item in command)


def shlex_quote(value: str) -> str:
    if not value or any(character.isspace() or character in "'\"$`\\!&;()[]{}<>|*?" for character in value):
        return "'" + value.replace("'", "'\"'\"'") + "'"
    return value


def run_raw(command: list[str], *, cwd: Path, env: dict[str, str] | None = None, timeout: int = 120) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command,
        cwd=cwd,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        timeout=timeout,
        check=False,
    )


def git(repo: Path, args: list[str], *, timeout: int = 120) -> str:
    completed = run_raw(["git", *args], cwd=repo, timeout=timeout)
    require(completed.returncode == 0, f"git {' '.join(args)} failed: {completed.stderr.strip()}")
    return completed.stdout.strip()


def remove_transient_root_cargo_lock(repo: Path) -> dict[str, str]:
    """Restore the repository's no-root-lock policy without deleting tracked data."""
    cargo_lock = repo / "Cargo.lock"
    if not cargo_lock.exists():
        return {"result": "PASS", "action": "not_present"}

    tracked = run_raw(["git", "ls-files", "--error-unmatch", "--", "Cargo.lock"], cwd=repo)
    require(tracked.returncode != 0, "tracked root Cargo.lock is present contrary to repository policy")
    status = git(repo, ["status", "--porcelain=v1", "--untracked-files=all", "--", "Cargo.lock"])
    require(
        status == "?? Cargo.lock",
        f"root Cargo.lock is present but is not a removable untracked file: {status!r}",
    )
    cargo_lock.unlink()
    require(not cargo_lock.exists(), "failed to remove transient untracked root Cargo.lock")
    return {"result": "PASS", "action": "removed_untracked"}


def discover_python() -> str:
    configured = os.environ.get("PYTHON")
    candidates = [configured] if configured else []
    candidates.extend([sys.executable, "python", "python3"])
    seen: set[str] = set()
    for candidate in candidates:
        if not candidate or candidate in seen:
            continue
        seen.add(candidate)
        try:
            completed = subprocess.run([candidate, "--version"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, timeout=30)
        except (OSError, subprocess.SubprocessError):
            continue
        if completed.returncode == 0:
            return candidate
    raise GateError("no usable Python interpreter was found")


def copy_working_tree(repo: Path, destination: Path) -> list[str]:
    completed = subprocess.run(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"],
        cwd=repo,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=120,
        check=False,
    )
    require(completed.returncode == 0, f"git ls-files failed: {completed.stderr.decode(errors='replace')}")
    relative_paths = sorted(path.decode("utf-8") for path in completed.stdout.split(b"\0") if path)
    require(relative_paths, "working-tree snapshot contains no files")
    for relative in relative_paths:
        source = repo / Path(*PurePosixPath(relative).parts)
        if not source.exists():
            continue
        target = destination / Path(*PurePosixPath(relative).parts)
        target.parent.mkdir(parents=True, exist_ok=True)
        if source.is_symlink():
            os.symlink(os.readlink(source), target)
        elif source.is_file():
            shutil.copy2(source, target)
        else:
            raise GateError(f"unsupported repository entry type: {relative}")
    return relative_paths


def initialize_snapshot_git(snapshot: Path) -> str:
    commands = [
        ["git", "init", "-q"],
        ["git", "config", "user.name", "AeroCodex Automated Gate"],
        ["git", "config", "user.email", "automation@aerocodex.invalid"],
        ["git", "add", "-A"],
    ]
    for command in commands:
        completed = run_raw(command, cwd=snapshot)
        require(completed.returncode == 0, f"snapshot command failed: {command!r}: {completed.stderr.strip()}")
    commit_env = dict(os.environ)
    commit_env.update(
        {
            "GIT_AUTHOR_DATE": "2000-01-01T00:00:00Z",
            "GIT_COMMITTER_DATE": "2000-01-01T00:00:00Z",
        }
    )
    completed = run_raw(["git", "commit", "-q", "-m", "AeroCodex automated gate snapshot"], cwd=snapshot, env=commit_env)
    require(completed.returncode == 0, f"snapshot commit failed: {completed.stderr.strip()}")
    return git(snapshot, ["rev-parse", "HEAD"])


def verify_checksum_manifest(root: Path) -> dict[str, Any]:
    manifest = root / "checksums/SHA256SUMS"
    require(manifest.is_file(), "checksums/SHA256SUMS is missing")
    rows: list[dict[str, Any]] = []
    for line_number, raw in enumerate(manifest.read_text(encoding="utf-8").splitlines(), 1):
        if not raw.strip():
            continue
        parts = raw.split(None, 1)
        require(len(parts) == 2, f"malformed SHA256SUMS line {line_number}")
        expected, relative = parts[0].lower(), parts[1].lstrip("*")
        require(len(expected) == 64 and all(character in "0123456789abcdef" for character in expected), f"invalid SHA-256 at line {line_number}")
        path = root / Path(*PurePosixPath(relative).parts)
        require(path.is_file(), f"checksummed file is missing: {relative}")
        actual = sha256_file(path)
        require(actual == expected, f"checksum mismatch for {relative}")
        rows.append({"path": relative, "sha256": actual})
    require(rows, "SHA256SUMS has no entries")
    return {"result": "PASS", "entry_count": len(rows), "entries": rows}


def parse_json_object(text: str, label: str) -> dict[str, Any]:
    try:
        value = json.loads(text)
    except json.JSONDecodeError as error:
        raise GateError(f"{label} emitted invalid JSON: {error}: {text!r}") from error
    require(isinstance(value, dict), f"{label} JSON is not an object")
    return value


def close_enough(actual: float, expected: float, *, rel_tol: float = 1e-12, abs_tol: float = 1e-12) -> bool:
    return math.isclose(actual, expected, rel_tol=rel_tol, abs_tol=abs_tol)


def cli_case_definitions(binary: Path) -> list[dict[str, Any]]:
    prefix = [str(binary)]
    return [
        {"name": "version_json", "command": [*prefix, "version", "--json"], "exit": 0, "stream": "stdout", "expect": {"ok": True, "supported_formula_count": 10, "validation_status": VALIDATION_STATUS}},
        {"name": "formula_catalog", "command": [*prefix, "formulas", "--json"], "exit": 0, "stream": "stdout", "expect": {"ok": True, "count": 10, "validation_status": VALIDATION_STATUS}},
        {"name": "time_unit_from_mu_du", "command": [*prefix, "run", "formula_vault.m00.canonical.time_unit_from_mu_du", "mu=1", "distance_unit=1", "--json"], "exit": 0, "stream": "stdout", "value": 1.0, "output": "time_unit"},
        {"name": "speed_unit_from_du_tu", "command": [*prefix, "run", "formula_vault.m00.canonical.speed_unit_from_du_tu", "distance_unit=14", "time_unit=2", "--json"], "exit": 0, "stream": "stdout", "value": 7.0, "output": "speed_unit"},
        {"name": "speed_unit_from_mu_du", "command": [*prefix, "run", "formula_vault.m00.canonical.speed_unit_from_mu_du", "mu=49", "distance_unit=1", "--json"], "exit": 0, "stream": "stdout", "value": 7.0, "output": "speed_unit"},
        {"name": "mu_from_units", "command": [*prefix, "run", "formula_vault.m00.canonical.mu_from_units", "mu=2", "distance_unit=2", "time_unit=2", "--json"], "exit": 0, "stream": "stdout", "value": 1.0, "output": "canonical_mu"},
        {"name": "distance_to_negative", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_to_canonical", "distance=-42", "distance_unit=7", "--json"], "exit": 0, "stream": "stdout", "value": -6.0, "output": "canonical_distance"},
        {"name": "distance_from_negative", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_from_canonical", "canonical_distance=-6", "distance_unit=7", "--json"], "exit": 0, "stream": "stdout", "value": -42.0, "output": "distance"},
        {"name": "time_to_negative", "command": [*prefix, "run", "formula_vault.m00.canonical.time_to_canonical", "time=-18", "time_unit=3", "--json"], "exit": 0, "stream": "stdout", "value": -6.0, "output": "canonical_time"},
        {"name": "time_from_negative", "command": [*prefix, "run", "formula_vault.m00.canonical.time_from_canonical", "canonical_time=-6", "time_unit=3", "--json"], "exit": 0, "stream": "stdout", "value": -18.0, "output": "time"},
        {"name": "speed_to_negative", "command": [*prefix, "run", "formula_vault.m00.canonical.speed_to_canonical", "speed=-14", "distance_unit=7", "time_unit=3", "--json"], "exit": 0, "stream": "stdout", "value": -6.0, "output": "canonical_speed"},
        {"name": "speed_from_negative", "command": [*prefix, "run", "formula_vault.m00.canonical.speed_from_canonical", "canonical_speed=-6", "distance_unit=7", "time_unit=3", "--json"], "exit": 0, "stream": "stdout", "value": -14.0, "output": "speed"},
        {"name": "zero_quantity_allowed", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_to_canonical", "distance=0", "distance_unit=7", "--json"], "exit": 0, "stream": "stdout", "value": 0.0, "output": "canonical_distance"},
        {"name": "nonfinite_quantity_rejected", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_to_canonical", "distance=NaN", "distance_unit=7", "--json"], "exit": 4, "stream": "stderr", "error": "out_of_domain"},
        {"name": "nonfinite_scale_rejected", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_to_canonical", "distance=1", "distance_unit=inf", "--json"], "exit": 4, "stream": "stderr", "error": "out_of_domain"},
        {"name": "zero_scale_rejected", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_to_canonical", "distance=1", "distance_unit=0", "--json"], "exit": 4, "stream": "stderr", "error": "non_positive_input"},
        {"name": "negative_scale_rejected", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_to_canonical", "distance=1", "distance_unit=-1", "--json"], "exit": 4, "stream": "stderr", "error": "non_positive_input"},
        {"name": "overflow_rejected", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_from_canonical", "canonical_distance=1e308", "distance_unit=1e308", "--json"], "exit": 4, "stream": "stderr", "error": "numerical_failure"},
        {"name": "missing_input", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_to_canonical", "distance=1", "--json"], "exit": 2, "stream": "stderr", "error": "missing_input"},
        {"name": "unexpected_input", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_to_canonical", "distance=1", "distance_unit=1", "extra=2", "--json"], "exit": 2, "stream": "stderr", "error": "unexpected_input"},
        {"name": "duplicate_input", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_to_canonical", "distance=1", "distance=2", "distance_unit=1", "--json"], "exit": 2, "stream": "stderr", "error": "duplicate_input"},
        {"name": "invalid_number", "command": [*prefix, "run", "formula_vault.m00.canonical.distance_to_canonical", "distance=abc", "distance_unit=1", "--json"], "exit": 2, "stream": "stderr", "error": "invalid_number"},
        {"name": "unknown_formula", "command": [*prefix, "describe", "formula_vault.m00.canonical.unknown", "--json"], "exit": 3, "stream": "stderr", "error": "unknown_formula"},
        {"name": "duplicate_json_flag", "command": [*prefix, "version", "--json", "--json"], "exit": 2, "stream": "stderr", "error": "usage_error"},
        {"name": "self_check", "command": [*prefix, "self-check", "--json"], "exit": 0, "stream": "stdout", "expect": {"ok": True, "passed": SELF_CHECK_CASES, "failed": 0}},
    ]


def evaluate_cli_case(case: dict[str, Any], *, cwd: Path, log_handle: Any) -> dict[str, Any]:
    started = time.monotonic()
    completed = run_raw(case["command"], cwd=cwd, timeout=120)
    duration_ms = round((time.monotonic() - started) * 1000)
    log_handle.write(f"\n===== CLI CASE {case['name']} =====\n")
    log_handle.write(f"command: {command_text(case['command'])}\nexit_code: {completed.returncode}\n")
    log_handle.write("--- stdout ---\n" + completed.stdout)
    log_handle.write("--- stderr ---\n" + completed.stderr)
    require(completed.returncode == case["exit"], f"CLI case {case['name']} exit mismatch: {completed.returncode} != {case['exit']}")
    stream_text = completed.stdout if case["stream"] == "stdout" else completed.stderr
    value = parse_json_object(stream_text, f"CLI case {case['name']}")
    for key, expected in case.get("expect", {}).items():
        require(value.get(key) == expected, f"CLI case {case['name']} field {key!r} mismatch")
    if "value" in case:
        actual_value = value.get("value")
        require(isinstance(actual_value, (int, float)), f"CLI case {case['name']} value is not numeric")
        require(close_enough(float(actual_value), float(case["value"])), f"CLI case {case['name']} value mismatch: {actual_value}")
        require(value.get("output_variable") == case["output"], f"CLI case {case['name']} output variable mismatch")
    if "error" in case:
        require(value.get("ok") is False, f"CLI case {case['name']} did not report ok=false")
        require(value.get("error", {}).get("code") == case["error"], f"CLI case {case['name']} error code mismatch")
    return {"name": case["name"], "result": "PASS", "exit_code": completed.returncode, "duration_ms": duration_ms}


def run_determinism_case(binary: Path, *, cwd: Path, log_handle: Any) -> dict[str, Any]:
    command = [str(binary), "run", "formula_vault.m00.canonical.speed_to_canonical", "speed=-14", "distance_unit=7", "time_unit=3", "--json"]
    outputs: list[str] = []
    for index in range(5):
        completed = run_raw(command, cwd=cwd, timeout=120)
        require(completed.returncode == 0, f"determinism run {index + 1} failed")
        outputs.append(completed.stdout)
    require(len(set(outputs)) == 1, "same-build/platform CLI output was not byte-for-byte deterministic")
    log_handle.write("\n===== CLI CASE deterministic_repeatability =====\n" + outputs[0])
    return {"name": "deterministic_repeatability", "result": "PASS", "repetitions": len(outputs)}


def run_round_trip_case(binary: Path, *, cwd: Path, log_handle: Any) -> dict[str, Any]:
    forward = run_raw([str(binary), "run", "formula_vault.m00.canonical.speed_to_canonical", "speed=-123.5", "distance_unit=7", "time_unit=3", "--json"], cwd=cwd)
    require(forward.returncode == 0, "round-trip forward conversion failed")
    forward_json = parse_json_object(forward.stdout, "round-trip forward")
    canonical = float(forward_json["value"])
    backward = run_raw([str(binary), "run", "formula_vault.m00.canonical.speed_from_canonical", f"canonical_speed={canonical:.17g}", "distance_unit=7", "time_unit=3", "--json"], cwd=cwd)
    require(backward.returncode == 0, "round-trip backward conversion failed")
    backward_json = parse_json_object(backward.stdout, "round-trip backward")
    actual = float(backward_json["value"])
    require(close_enough(actual, -123.5, rel_tol=1e-14, abs_tol=1e-12), f"round-trip value mismatch: {actual}")
    log_handle.write("\n===== CLI CASE bounded_round_trip =====\n" + forward.stdout + backward.stdout)
    return {"name": "bounded_round_trip", "result": "PASS", "expected": -123.5, "actual": actual}


def write_junit(path: Path, test_rows: list[dict[str, Any]], elapsed_seconds: float) -> None:
    failures = sum(1 for row in test_rows if row.get("result") != "PASS")
    suite = ET.Element(
        "testsuite",
        {
            "name": "AeroCodex Beta 1 automated gate",
            "tests": str(len(test_rows)),
            "failures": str(failures),
            "errors": "0",
            "time": f"{elapsed_seconds:.3f}",
        },
    )
    for row in test_rows:
        case = ET.SubElement(suite, "testcase", {"classname": row.get("category", "beta1"), "name": row["name"], "time": f"{row.get('duration_ms', 0) / 1000:.3f}"})
        if row.get("result") != "PASS":
            failure = ET.SubElement(case, "failure", {"message": row.get("error", "failed")})
            failure.text = row.get("detail", row.get("error", "failed"))
    tree = ET.ElementTree(suite)
    ET.indent(tree, space="  ")
    tree.write(path, encoding="utf-8", xml_declaration=True)


def self_test(output_dir: Path) -> dict[str, Any]:
    output_dir.mkdir(parents=True)
    fixture = output_dir / "fixture.txt"
    fixture.write_text("AeroCodex automated gate self-test\n", encoding="utf-8", newline="\n")
    digest = sha256_file(fixture)
    tests = [{"name": "stable_json", "category": "self-test", "result": "PASS"}, {"name": "sha256", "category": "self-test", "result": "PASS", "sha256": digest}]
    report = {"schema_version": GATE_SCHEMA_VERSION, "result": "PASS", "mode": "self-test", "tests": tests}
    (output_dir / "beta1-test-report.json").write_text(stable_json(report), encoding="utf-8", newline="\n")
    (output_dir / "beta1-test.log").write_text("AeroCodex automated gate self-test: PASS\n", encoding="utf-8", newline="\n")
    write_junit(output_dir / "beta1-test-report.junit.xml", tests, 0.0)
    fixture.unlink()
    return report


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--timeout-seconds", type=int, default=DEFAULT_TIMEOUT_SECONDS)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    repo = args.repo.resolve()
    output_dir = (args.output_dir.resolve() if args.output_dir else repo / "target" / "beta1-automated" / safe_timestamp())
    report_path = output_dir / "beta1-test-report.json"
    junit_path = output_dir / "beta1-test-report.junit.xml"
    log_path = output_dir / "beta1-test.log"
    started_wall = utc_now()
    started = time.monotonic()
    tests: list[dict[str, Any]] = []
    report: dict[str, Any] = {
        "schema_version": GATE_SCHEMA_VERSION,
        "phase": "AeroCodex Beta 1 automated qualification gate",
        "result": "FAIL",
        "started_utc": started_wall,
        "repository": str(repo),
        "output_directory": str(output_dir),
        "validation_status": VALIDATION_STATUS,
        "supported_formula_count": SUPPORTED_FORMULA_COUNT,
        "self_check_cases": SELF_CHECK_CASES,
        "claims": {
            "certification": False,
            "operational_readiness": False,
            "full_equation_inventory_complete": False,
        },
    }
    try:
        require(repo.is_dir(), f"repository does not exist: {repo}")
        require(not output_dir.exists(), f"output directory already exists: {output_dir}")
        require(output_dir != repo and output_dir not in repo.parents, "output directory must not contain the repository")
        if args.self_test:
            report = self_test(output_dir)
            tests = list(report["tests"])
            print(stable_json(report), end="")
            return 0
        require((repo / ".git").exists(), f"repository has no .git directory: {repo}")
        output_dir.mkdir(parents=True)

        root_cargo_lock_cleanup = remove_transient_root_cargo_lock(repo)
        python = discover_python()
        base_head = git(repo, ["rev-parse", "HEAD"])
        baseline_status = git(repo, ["status", "--porcelain=v1", "--untracked-files=all"])
        require(run_raw(["git", "diff", "--cached", "--quiet"], cwd=repo).returncode == 0, "source repository index is not clean")
        require(not (repo / "Cargo.lock").exists(), "root Cargo.lock is present contrary to repository policy")
        report["source"] = {
            "base_head": base_head,
            "baseline_status": baseline_status.splitlines(),
            "index_clean": True,
            "root_cargo_lock_cleanup": root_cargo_lock_cleanup,
        }

        with log_path.open("w", encoding="utf-8", newline="\n") as log_handle, tempfile.TemporaryDirectory(prefix="aerocodex-beta1-gate-") as temporary_name:
            temporary = Path(temporary_name)
            snapshot = temporary / "snapshot"
            snapshot.mkdir()
            copied_paths = copy_working_tree(repo, snapshot)
            snapshot_commit = initialize_snapshot_git(snapshot)
            report["source"].update({"snapshot_commit": snapshot_commit, "snapshot_file_count": len(copied_paths)})
            cargo_target = temporary / "cargo-target"
            command_env = dict(os.environ)
            command_env.update({"CARGO_TARGET_DIR": str(cargo_target), "PYTHONDONTWRITEBYTECODE": "1"})

            def execute_step(name: str, command: list[str], *, expected_exit: int = 0, timeout: int | None = None) -> None:
                step_started = time.monotonic()
                completed = run_raw(command, cwd=snapshot, env=command_env, timeout=timeout or args.timeout_seconds)
                duration_ms = round((time.monotonic() - step_started) * 1000)
                log_handle.write(f"\n===== STEP {name} =====\ncommand: {command_text(command)}\nexit_code: {completed.returncode}\n")
                log_handle.write("--- stdout ---\n" + completed.stdout)
                log_handle.write("--- stderr ---\n" + completed.stderr)
                row = {"name": name, "category": "repository", "result": "PASS" if completed.returncode == expected_exit else "FAIL", "exit_code": completed.returncode, "expected_exit_code": expected_exit, "duration_ms": duration_ms, "command": command}
                tests.append(row)
                require(completed.returncode == expected_exit, f"step {name} failed with exit code {completed.returncode}")

            execute_step("release_verifier_self_test", [python, "scripts/verify_beta1_release.py", "--self-test"])
            execute_step("git_diff_check", ["git", "diff", "--check"])
            checksum_started = time.monotonic()
            checksum_result = verify_checksum_manifest(snapshot)
            tests.append({"name": "checksum_manifest", "category": "repository", "result": "PASS", "duration_ms": round((time.monotonic() - checksum_started) * 1000), "entry_count": checksum_result["entry_count"]})
            execute_step("cargo_fmt", ["cargo", "fmt", "--all", "--", "--check"])
            execute_step("cargo_check", ["cargo", "check", "--workspace", "--all-targets", "--all-features"])
            execute_step("cargo_clippy", ["cargo", "clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings"])
            execute_step("cargo_test", ["cargo", "test", "--workspace", "--all-targets", "--all-features"])
            execute_step("xtask_verify_all", [python, "scripts/verify_governance.py", "--repo", "."])
            execute_step("dependency_policy", ["cargo", "run", "-p", "xtask", "--", "dependency-policy"])
            execute_step("thinfilm_verification", [python, "scripts/verify_thinfilm_artifact.py"])
            execute_step("nomenclature_lint", [python, "nomenclature/tooling/aerocodex_nom_lint.py", "--root", "nomenclature"])
            execute_step("acronym_inventory", [python, "nomenclature/tooling/aerocodex_acronym_inventory.py", "--repo-root", ".", "--nomenclature-root", "nomenclature", "--check-new", "--baseline", "nomenclature/generated/current_repo_acronym_baseline.json"])
            execute_step("terminology_export", [python, "nomenclature/tooling/aerocodex_terminology.py", "--root", "nomenclature", "export-jsonl", "--output", "nomenclature/generated/terminology/index.jsonl"])
            execute_step("terminology_diff", ["git", "diff", "--exit-code", "nomenclature/generated/terminology/index.jsonl"])
            rustdoc_env = dict(command_env)
            rustdoc_env["RUSTDOCFLAGS"] = "-D warnings"
            step_started = time.monotonic()
            completed = run_raw(["cargo", "doc", "--workspace", "--all-features", "--no-deps"], cwd=snapshot, env=rustdoc_env, timeout=args.timeout_seconds)
            tests.append({"name": "cargo_doc", "category": "repository", "result": "PASS" if completed.returncode == 0 else "FAIL", "exit_code": completed.returncode, "duration_ms": round((time.monotonic() - step_started) * 1000)})
            log_handle.write("\n===== STEP cargo_doc =====\n" + completed.stdout + completed.stderr)
            require(completed.returncode == 0, "cargo_doc failed")

            execute_step("cli_build", ["cargo", "build", "-p", "aero-codex-cli"])
            binary = cargo_target / "debug" / ("aerocodex.exe" if os.name == "nt" else "aerocodex")
            require(binary.is_file(), f"debug CLI binary is missing: {binary}")
            cli_results: list[dict[str, Any]] = []
            for case in cli_case_definitions(binary):
                try:
                    result = evaluate_cli_case(case, cwd=snapshot, log_handle=log_handle)
                    result["category"] = "cli"
                    cli_results.append(result)
                    tests.append(result)
                except Exception as error:
                    row = {"name": case["name"], "category": "cli", "result": "FAIL", "error": str(error)}
                    cli_results.append(row)
                    tests.append(row)
                    raise
            determinism = run_determinism_case(binary, cwd=snapshot, log_handle=log_handle)
            determinism["category"] = "cli"
            tests.append(determinism)
            cli_results.append(determinism)
            round_trip = run_round_trip_case(binary, cwd=snapshot, log_handle=log_handle)
            round_trip["category"] = "cli"
            tests.append(round_trip)
            cli_results.append(round_trip)
            report["cli"] = {"result": "PASS", "case_count": len(cli_results), "cases": cli_results}

            cargo_lock = snapshot / "Cargo.lock"
            if cargo_lock.exists():
                cargo_lock.unlink()
                tests.append({"name": "remove_transient_snapshot_cargo_lock", "category": "snapshot", "result": "PASS"})
            require(git(snapshot, ["status", "--porcelain=v1", "--untracked-files=all"]) == "", "snapshot is not clean before packaging")

            package_output = output_dir / "native-package"
            execute_step("native_package", [python, "scripts/package_beta1_release.py", "--repo", str(snapshot), "--output-dir", str(package_output)], timeout=max(args.timeout_seconds, 2400))
            package_report_path = package_output / "package-report.json"
            require(package_report_path.is_file(), "package-report.json is missing")
            package_report = parse_json_object(package_report_path.read_text(encoding="utf-8"), "package report")
            require(package_report.get("result") == "PASS", "package report did not report PASS")
            require(package_report.get("archive_verification", {}).get("result") == "PASS", "archive verification did not report PASS")
            smoke_checks = package_report.get("smoke_checks", [])
            require(len(smoke_checks) == 6 and all(row.get("result") == "PASS" for row in smoke_checks), "packaged-binary smoke contract did not pass 6/6")
            archive = Path(package_report["archive"])
            require(archive.is_file(), "candidate archive is missing")
            report["package"] = {"result": "PASS", "report": str(package_report_path), "archive": str(archive), "archive_sha256": sha256_file(archive), "target": package_report.get("target"), "smoke_check_count": len(smoke_checks)}

            tamper_root = temporary / "tamper"
            tamper_root.mkdir()
            import zipfile
            with zipfile.ZipFile(archive, "r") as handle:
                handle.extractall(tamper_root)
            roots = [path for path in tamper_root.iterdir() if path.is_dir()]
            require(len(roots) == 1, "candidate archive did not extract to exactly one root")
            tampered_bundle = roots[0]
            readme = tampered_bundle / "README.md"
            require(readme.is_file(), "tamper fixture README.md is missing")
            readme.write_text(readme.read_text(encoding="utf-8") + "tampered\n", encoding="utf-8", newline="\n")
            tamper = run_raw([python, "scripts/verify_beta1_release.py", "--bundle-dir", str(tampered_bundle)], cwd=snapshot, env=command_env, timeout=300)
            log_handle.write("\n===== STEP actual_archive_tamper_rejection =====\n" + tamper.stdout + tamper.stderr)
            tamper_json = parse_json_object(tamper.stderr, "tamper rejection")
            require(tamper.returncode == 1 and tamper_json.get("result") == "FAIL", "tampered candidate bundle was not rejected")
            tests.append({"name": "actual_archive_tamper_rejection", "category": "package", "result": "PASS", "exit_code": tamper.returncode})
            report["package"]["actual_tamper_rejected"] = True

        final_head = git(repo, ["rev-parse", "HEAD"])
        final_status = git(repo, ["status", "--porcelain=v1", "--untracked-files=all"])
        require(final_head == base_head, "source repository HEAD changed during gate")
        require(final_status == baseline_status, "source repository status changed during gate")
        require(not (repo / "Cargo.lock").exists(), "gate created a root Cargo.lock")
        report["repository_unchanged"] = True
        report["result"] = "PASS"
    except (OSError, GateError, subprocess.SubprocessError, json.JSONDecodeError) as error:
        report["error"] = str(error)
        tests.append({"name": "gate_failure", "category": "gate", "result": "FAIL", "error": str(error)})
    finally:
        elapsed = time.monotonic() - started
        report["finished_utc"] = utc_now()
        report["elapsed_seconds"] = round(elapsed, 3)
        report["test_count"] = len(tests)
        report["passed_tests"] = sum(1 for row in tests if row.get("result") == "PASS")
        report["failed_tests"] = sum(1 for row in tests if row.get("result") != "PASS")
        report["tests"] = tests
        if output_dir.exists():
            report_path.write_text(stable_json(report), encoding="utf-8", newline="\n")
            write_junit(junit_path, tests, elapsed)
            if not log_path.exists():
                log_path.write_text("AeroCodex Beta 1 automated gate failed before command logging began.\n", encoding="utf-8", newline="\n")
    print(stable_json(report), end="" if report.get("result") == "PASS" else "", file=sys.stdout if report.get("result") == "PASS" else sys.stderr)
    return 0 if report.get("result") == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
