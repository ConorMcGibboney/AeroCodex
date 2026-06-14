#![forbid(unsafe_code)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const FORBIDDEN_CARGO_PATTERNS: &[&str] = &[
    "cc =",
    "\"cc\"",
    "cmake",
    "bindgen",
    "pkg-config",
    "vcpkg",
    "blas",
    "lapack",
    "openblas",
    "netlib",
    "suitesparse",
    "refprop",
    "coolprop",
    "cantera",
    "openfoam",
    "su2",
    "spice",
    "_sys",
];

fn main() {
    let mut args = env::args().skip(1);
    let command = args.next();
    let result = match command.as_deref() {
        Some("verify") => verify(),
        Some("dependency-policy") => dependency_policy(),
        Some("help") | Some("--help") | Some("-h") | None => {
            print_help();
            Ok(())
        }
        Some(other) => Err(format!("unknown xtask command: {other}")),
    };

    if let Err(error) = result {
        eprintln!("xtask failed: {error}");
        process::exit(1);
    }
}

fn print_help() {
    println!("AeroCodex xtask commands:");
    println!("  verify [--all]          verify baseline evidence-card files");
    println!("  dependency-policy       scan Cargo.toml files for denied native dependencies");
}

fn repo_root() -> Result<PathBuf, String> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "could not determine repository root".to_string())
}

fn verify() -> Result<(), String> {
    let root = repo_root()?;
    let schema = root.join("validation/codex_card.schema.yaml");
    let levels = root.join("validation/evidence_levels.yaml");
    let cards_dir = root.join("validation/evidence-cards");

    require_file(&schema)?;
    require_file(&levels)?;
    require_dir(&cards_dir)?;

    let mut card_paths = Vec::new();
    collect_files_with_extensions(&cards_dir, &["yaml", "yml"], &mut card_paths)?;
    if card_paths.is_empty() {
        return Err("no evidence cards found in validation/evidence-cards".to_string());
    }

    for path in &card_paths {
        let content = fs::read_to_string(path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        for required in [
            "id:",
            "source:",
            "tests:",
            "failure_modes:",
            "verification_status:",
        ] {
            if !content.contains(required) {
                return Err(format!(
                    "{} is missing required field {required}",
                    path.display()
                ));
            }
        }
    }

    println!("verified {} evidence card(s)", card_paths.len());
    Ok(())
}

fn dependency_policy() -> Result<(), String> {
    let root = repo_root()?;
    let mut cargo_tomls = Vec::new();
    collect_cargo_tomls(&root, &mut cargo_tomls)?;

    let mut violations = Vec::new();
    for path in &cargo_tomls {
        let content = fs::read_to_string(path)
            .map_err(|err| format!("could not read {}: {err}", path.display()))?;
        let lowered = content.to_lowercase();
        for pattern in FORBIDDEN_CARGO_PATTERNS {
            if lowered.contains(pattern) {
                violations.push(format!(
                    "{} contains denied pattern `{pattern}`",
                    path.display()
                ));
            }
        }
    }

    if !violations.is_empty() {
        return Err(violations.join("\n"));
    }

    println!(
        "dependency policy passed for {} Cargo.toml file(s)",
        cargo_tomls.len()
    );
    Ok(())
}

fn require_file(path: &Path) -> Result<(), String> {
    if path.is_file() {
        Ok(())
    } else {
        Err(format!("required file missing: {}", path.display()))
    }
}

fn require_dir(path: &Path) -> Result<(), String> {
    if path.is_dir() {
        Ok(())
    } else {
        Err(format!("required directory missing: {}", path.display()))
    }
}

fn collect_cargo_tomls(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|err| format!("could not read {}: {err}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|err| format!("could not read directory entry: {err}"))?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("");
        if path.is_dir() {
            if matches!(name, ".git" | "target") {
                continue;
            }
            collect_cargo_tomls(&path, out)?;
        } else if name == "Cargo.toml" {
            out.push(path);
        }
    }
    Ok(())
}

fn collect_files_with_extensions(
    dir: &Path,
    extensions: &[&str],
    out: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|err| format!("could not read {}: {err}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|err| format!("could not read directory entry: {err}"))?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_with_extensions(&path, extensions, out)?;
        } else if let Some(ext) = path.extension().and_then(|value| value.to_str()) {
            if extensions.contains(&ext) {
                out.push(path);
            }
        }
    }
    Ok(())
}
