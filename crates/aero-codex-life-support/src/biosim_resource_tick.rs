use std::collections::BTreeMap;

use aero_codex_core::{
    validation, AeroError, AeroResult, EngineeringResult, ValidityStatus, VerificationRecord,
};

/// Codex identifier for the clean-room BioSim-RS resource catalog gate.
#[must_use]
pub fn biosim_resource_catalog_codex_id() -> &'static str {
    "life_support.biosim_rs.resource_catalog_identity"
}

/// Codex identifier for the clean-room BioSim-RS tick-semantics gate.
#[must_use]
pub fn biosim_tick_validation_codex_id() -> &'static str {
    "life_support.biosim_rs.tick_validation"
}

/// Codex identifier for the clean-room BioSim-RS atomic transaction commit gate.
#[must_use]
pub fn biosim_transaction_commit_codex_id() -> &'static str {
    "life_support.biosim_rs.atomic_transaction_commit"
}

/// Codex identifier for the clean-room BioSim-RS deterministic ordering/digest gate.
#[must_use]
pub fn biosim_deterministic_replay_codex_id() -> &'static str {
    "life_support.biosim_rs.deterministic_ordering_digest_replay"
}

/// Source-registry seed for the Chunk 6A clean-room resource/tick slice.
#[must_use]
pub fn biosim_resource_tick_clean_room_source_id() -> &'static str {
    "source.life_support.biosim_rs.resource_tick_clean_room.research_required"
}

/// Source-registry seed for the Chunk 6B clean-room transaction-commit slice.
#[must_use]
pub fn biosim_resource_transaction_clean_room_source_id() -> &'static str {
    "source.life_support.biosim_rs.transaction_commit_clean_room.research_required"
}

/// Source-registry seed for the Chunk 6C clean-room deterministic replay slice.
#[must_use]
pub fn biosim_resource_replay_clean_room_source_id() -> &'static str {
    "source.life_support.biosim_rs.deterministic_replay_clean_room.research_required"
}

fn biosim_resource_tick_sources() -> &'static [&'static str] {
    &["source.life_support.biosim_rs.resource_tick_clean_room.research_required"]
}

fn biosim_transaction_commit_sources() -> &'static [&'static str] {
    &[
        "source.life_support.biosim_rs.resource_tick_clean_room.research_required",
        "source.life_support.biosim_rs.transaction_commit_clean_room.research_required",
    ]
}

fn biosim_deterministic_replay_sources() -> &'static [&'static str] {
    &[
        "source.life_support.biosim_rs.resource_tick_clean_room.research_required",
        "source.life_support.biosim_rs.transaction_commit_clean_room.research_required",
        "source.life_support.biosim_rs.deterministic_replay_clean_room.research_required",
    ]
}

/// Minimal clean-room resource families for the first BioSim-RS validation slice.
///
/// These are intentionally generic resource identities, not a translation of
/// GPL BioSim Java classes or the GPL-bound BioSim-RS scaffold crates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BioSimResourceKind {
    OxygenGas,
    CarbonDioxideGas,
    PotableWater,
    WasteWater,
    EdibleBiomass,
    DryWaste,
    ElectricalEnergy,
    ThermalEnergy,
}

/// Canonical identity and unit metadata for one BioSim-RS resource kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BioSimResourceIdentity {
    pub kind: BioSimResourceKind,
    pub canonical_id: &'static str,
    pub canonical_unit: &'static str,
}

/// Positive-duration simulation tick after syntactic validation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BioSimTick {
    pub index: u64,
    pub duration_seconds: f64,
}

/// Consecutive tick transition accepted by the Chunk 6A validation gate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BioSimTickAdvance {
    pub previous_index: u64,
    pub next_index: u64,
    pub next_duration_seconds: f64,
}

/// Resource quantity in a caller-supplied clean-room transaction state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BioSimResourceQuantity {
    pub kind: BioSimResourceKind,
    pub amount: f64,
}

/// Resource delta staged for one atomic clean-room transaction commit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BioSimResourceDelta {
    pub kind: BioSimResourceKind,
    pub delta_amount: f64,
}

/// Result of applying a complete resource-delta set at one validated tick boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimResourceTransactionCommit {
    pub tick: BioSimTickAdvance,
    pub balances: Vec<BioSimResourceQuantity>,
    pub delta_count: usize,
}

/// Deterministic clean-room digest over an ordered BioSim-RS resource snapshot.
///
/// This is a dependency-free fnv-1a smoke-test digest for replay comparison, not
/// a cryptographic hash, not a persisted ledger key, and not validation evidence
/// for habitat-control, medical, or operational use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BioSimResourceDigest {
    pub algorithm: &'static str,
    pub tick_index: u64,
    pub value: String,
}

/// Deterministic before/after replay proof for one clean-room resource commit.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimResourceReplayProof {
    pub tick: BioSimTickAdvance,
    pub before_digest: BioSimResourceDigest,
    pub after_digest: BioSimResourceDigest,
    pub delta_digest: BioSimResourceDigest,
    pub ordered_delta_count: usize,
}

/// Conservative built-in resource catalog for future BioSim-RS slices.
#[must_use]
pub fn biosim_resource_catalog() -> &'static [BioSimResourceKind] {
    &[
        BioSimResourceKind::OxygenGas,
        BioSimResourceKind::CarbonDioxideGas,
        BioSimResourceKind::PotableWater,
        BioSimResourceKind::WasteWater,
        BioSimResourceKind::EdibleBiomass,
        BioSimResourceKind::DryWaste,
        BioSimResourceKind::ElectricalEnergy,
        BioSimResourceKind::ThermalEnergy,
    ]
}

/// Canonical metadata for a clean-room BioSim-RS resource kind.
#[must_use]
pub const fn biosim_resource_identity(kind: BioSimResourceKind) -> BioSimResourceIdentity {
    match kind {
        BioSimResourceKind::OxygenGas => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.oxygen_gas",
            canonical_unit: "kg",
        },
        BioSimResourceKind::CarbonDioxideGas => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.carbon_dioxide_gas",
            canonical_unit: "kg",
        },
        BioSimResourceKind::PotableWater => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.potable_water",
            canonical_unit: "kg",
        },
        BioSimResourceKind::WasteWater => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.waste_water",
            canonical_unit: "kg",
        },
        BioSimResourceKind::EdibleBiomass => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.edible_biomass",
            canonical_unit: "kg",
        },
        BioSimResourceKind::DryWaste => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.dry_waste",
            canonical_unit: "kg",
        },
        BioSimResourceKind::ElectricalEnergy => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.electrical_energy",
            canonical_unit: "kWh",
        },
        BioSimResourceKind::ThermalEnergy => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.thermal_energy",
            canonical_unit: "kWh",
        },
    }
}

/// Conservative traceability metadata for the clean-room BioSim-RS resource/tick slices.
#[must_use]
pub fn biosim_resource_tick_verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        id if id == biosim_resource_catalog_codex_id() => Some(VerificationRecord::research_required(
            biosim_resource_catalog_codex_id(),
            biosim_resource_tick_sources(),
            "Clean-room resource identity validation implemented; no GPL BioSim source, scaffold code, fixtures, or habitat-operation evidence is included.",
        )),
        id if id == biosim_tick_validation_codex_id() => Some(VerificationRecord::research_required(
            biosim_tick_validation_codex_id(),
            biosim_resource_tick_sources(),
            "Clean-room tick validation implemented for positive-duration and consecutive-index checks; no transaction commit, ledger, replay, or external validation evidence is included.",
        )),
        id if id == biosim_transaction_commit_codex_id() => Some(
            VerificationRecord::research_required(
                biosim_transaction_commit_codex_id(),
                biosim_transaction_commit_sources(),
                "Clean-room atomic resource-delta commit implemented for one validated tick boundary; no ledger persistence, replay proof, conservation model, or external BioSim validation evidence is included.",
            ),
        ),
        id if id == biosim_deterministic_replay_codex_id() => Some(
            VerificationRecord::research_required(
                biosim_deterministic_replay_codex_id(),
                biosim_deterministic_replay_sources(),
                "Clean-room deterministic resource ordering plus fnv-1a before/after digest proof implemented for one caller-supplied transaction; no persistent ledger, scenario engine, conservation model, or external BioSim validation evidence is included.",
            ),
        ),
        _ => None,
    }
}

fn resource_tick_record(codex_id: &'static str) -> VerificationRecord {
    biosim_resource_tick_verification_record(codex_id).unwrap_or_else(|| {
        VerificationRecord::research_required(
            codex_id,
            biosim_resource_tick_sources(),
            "BioSim-RS Chunk 6A helper is present but has no upgraded source-validation status.",
        )
    })
}

fn out_of_domain(parameter: &'static str, value: f64, expected: &'static str) -> AeroError {
    AeroError::OutOfDomain {
        parameter,
        value,
        expected,
    }
}

/// Validates that a clean-room BioSim-RS resource catalog is nonempty and unique.
///
/// The enum carries the allowed resource identity vocabulary; this function only
/// checks catalog structure for the first implementation slice. It does not
/// create resource ledgers, execute transactions, or validate biological control
/// behavior.
pub fn validate_biosim_resource_catalog(
    catalog: &[BioSimResourceKind],
) -> AeroResult<EngineeringResult<usize>> {
    if catalog.is_empty() {
        return Err(out_of_domain(
            "resource_catalog",
            0.0,
            "at least one clean-room resource identity",
        ));
    }

    for (index, kind) in catalog.iter().enumerate() {
        if catalog[..index].contains(kind) {
            return Err(out_of_domain(
                "resource_catalog",
                index as f64,
                "unique resource identities",
            ));
        }
    }

    Ok(EngineeringResult::new(
        catalog.len(),
        biosim_resource_catalog_codex_id(),
        resource_tick_record(biosim_resource_catalog_codex_id()),
    )
    .with_assumption(
        "biosim_rs.clean_room_chunk6a",
        "resource identities are clean-room generic mass/energy buckets, not GPL source translations",
    )
    .with_assumption(
        "biosim_rs.resource_identity_only",
        "catalog validation does not imply transaction, ledger, replay, or biological model validation",
    )
    .with_validity(ValidityStatus::WithinDocumentedDomain))
}

/// Validates a positive-duration BioSim-RS simulation tick.
pub fn validate_biosim_tick(
    index: u64,
    duration_seconds: f64,
) -> AeroResult<EngineeringResult<BioSimTick>> {
    validation::ensure_positive("tick_duration_seconds", duration_seconds)?;

    let validity = if index == 0 {
        ValidityStatus::BoundaryCase
    } else {
        ValidityStatus::WithinDocumentedDomain
    };

    let mut result = EngineeringResult::new(
        BioSimTick {
            index,
            duration_seconds,
        },
        biosim_tick_validation_codex_id(),
        resource_tick_record(biosim_tick_validation_codex_id()),
    )
    .with_assumption(
        "biosim_rs.discrete_tick",
        "tick duration is finite, strictly positive seconds and index is caller-supplied",
    )
    .with_assumption(
        "biosim_rs.tick_validation_only",
        "tick validation does not commit resource deltas or prove deterministic replay",
    )
    .with_validity(validity);

    if index == 0 {
        result = result.with_warning(
            "biosim_rs.tick.initial_boundary",
            "tick index zero is treated as an initialization boundary before dynamic replay evidence exists",
        );
    }

    Ok(result)
}

/// Validates that the next BioSim-RS tick is exactly one index after the previous tick.
pub fn validate_biosim_tick_advance(
    previous: BioSimTick,
    next: BioSimTick,
) -> AeroResult<EngineeringResult<BioSimTickAdvance>> {
    validation::ensure_positive("previous_tick_duration_seconds", previous.duration_seconds)?;
    validation::ensure_positive("next_tick_duration_seconds", next.duration_seconds)?;

    let expected_next = previous.index.checked_add(1).ok_or_else(|| {
        out_of_domain(
            "previous_tick_index",
            previous.index as f64,
            "previous tick index with an available successor",
        )
    })?;

    if next.index != expected_next {
        return Err(out_of_domain(
            "next_tick_index",
            next.index as f64,
            "exactly previous_tick_index + 1",
        ));
    }

    Ok(EngineeringResult::new(
        BioSimTickAdvance {
            previous_index: previous.index,
            next_index: next.index,
            next_duration_seconds: next.duration_seconds,
        },
        biosim_tick_validation_codex_id(),
        resource_tick_record(biosim_tick_validation_codex_id()),
    )
    .with_assumption(
        "biosim_rs.tick_consecutive_index",
        "accepted transitions advance one discrete tick index at a time",
    )
    .with_assumption(
        "biosim_rs.tick_validation_only",
        "tick-advance validation records ordering only; transaction commits require commit_biosim_resource_transaction",
    )
    .with_validity(ValidityStatus::WithinDocumentedDomain))
}

fn biosim_resource_state_digest_algorithm() -> &'static str {
    "fnv1a64:biosim_resource_state:v1"
}

fn biosim_resource_delta_digest_algorithm() -> &'static str {
    "fnv1a64:biosim_resource_delta:v1"
}

fn fnv1a64_offset_basis() -> u64 {
    0xcbf2_9ce4_8422_2325
}

fn fnv1a64_prime() -> u64 {
    0x0000_0100_0000_01b3
}

fn fnv1a64_update(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(fnv1a64_prime());
    }
}

fn fnv1a64_field(hash: &mut u64, label: &str, value: &str) {
    fnv1a64_update(hash, label.as_bytes());
    fnv1a64_update(hash, b"=\0");
    fnv1a64_update(hash, value.as_bytes());
    fnv1a64_update(hash, b"\n");
}

fn canonical_f64_bits(value: f64) -> u64 {
    if value == 0.0 {
        0.0_f64.to_bits()
    } else {
        value.to_bits()
    }
}

fn ordered_biosim_resource_state(
    state: &[BioSimResourceQuantity],
) -> AeroResult<Vec<BioSimResourceQuantity>> {
    validate_biosim_resource_state(state)?;
    let mut ordered = BTreeMap::new();
    for balance in state {
        ordered.insert(
            biosim_resource_identity(balance.kind).canonical_id,
            *balance,
        );
    }
    Ok(ordered.into_values().collect())
}

fn ordered_biosim_resource_deltas(
    deltas: &[BioSimResourceDelta],
) -> AeroResult<Vec<BioSimResourceDelta>> {
    validate_biosim_resource_deltas(deltas)?;
    let mut ordered = BTreeMap::new();
    for delta in deltas {
        ordered.insert(biosim_resource_identity(delta.kind).canonical_id, *delta);
    }
    Ok(ordered.into_values().collect())
}

fn digest_ordered_resource_state(
    tick_index: u64,
    ordered_state: &[BioSimResourceQuantity],
) -> BioSimResourceDigest {
    let mut hash = fnv1a64_offset_basis();
    fnv1a64_field(
        &mut hash,
        "algorithm",
        biosim_resource_state_digest_algorithm(),
    );
    fnv1a64_field(&mut hash, "schema", "resource_state_v1");
    fnv1a64_field(&mut hash, "tick_index", &tick_index.to_string());
    for balance in ordered_state {
        let identity = biosim_resource_identity(balance.kind);
        fnv1a64_field(&mut hash, "resource_id", identity.canonical_id);
        fnv1a64_field(&mut hash, "unit", identity.canonical_unit);
        fnv1a64_field(
            &mut hash,
            "amount_bits",
            &format!("{:016x}", canonical_f64_bits(balance.amount)),
        );
    }

    BioSimResourceDigest {
        algorithm: biosim_resource_state_digest_algorithm(),
        tick_index,
        value: format!("{hash:016x}"),
    }
}

fn digest_ordered_resource_deltas(
    tick_index: u64,
    ordered_deltas: &[BioSimResourceDelta],
) -> BioSimResourceDigest {
    let mut hash = fnv1a64_offset_basis();
    fnv1a64_field(
        &mut hash,
        "algorithm",
        biosim_resource_delta_digest_algorithm(),
    );
    fnv1a64_field(&mut hash, "schema", "resource_delta_v1");
    fnv1a64_field(&mut hash, "tick_index", &tick_index.to_string());
    for delta in ordered_deltas {
        let identity = biosim_resource_identity(delta.kind);
        fnv1a64_field(&mut hash, "resource_id", identity.canonical_id);
        fnv1a64_field(&mut hash, "unit", identity.canonical_unit);
        fnv1a64_field(
            &mut hash,
            "delta_bits",
            &format!("{:016x}", canonical_f64_bits(delta.delta_amount)),
        );
    }

    BioSimResourceDigest {
        algorithm: biosim_resource_delta_digest_algorithm(),
        tick_index,
        value: format!("{hash:016x}"),
    }
}

/// Computes a deterministic digest for a caller-supplied clean-room resource state.
///
/// Resource balances are ordered by canonical resource ID before hashing so that
/// callers can compare exact replay evidence without relying on input slice order
/// or hash-map iteration order. The digest is fnv-1a 64-bit and non-cryptographic.
pub fn digest_biosim_resource_state(
    tick: BioSimTick,
    state: &[BioSimResourceQuantity],
) -> AeroResult<EngineeringResult<BioSimResourceDigest>> {
    validation::ensure_positive("tick_duration_seconds", tick.duration_seconds)?;
    let ordered_state = ordered_biosim_resource_state(state)?;
    let digest = digest_ordered_resource_state(tick.index, &ordered_state);

    Ok(EngineeringResult::new(
        digest,
        biosim_deterministic_replay_codex_id(),
        resource_tick_record(biosim_deterministic_replay_codex_id()),
    )
    .with_assumption(
        "biosim_rs.deterministic_resource_order",
        "resource states are canonicalized by static resource ID before digest generation",
    )
    .with_assumption(
        "biosim_rs.non_cryptographic_digest",
        "fnv-1a 64-bit digest supports replay smoke comparisons only and is not a security checksum",
    )
    .with_validity(ValidityStatus::WithinDocumentedDomain))
}

/// Produces deterministic before/after replay evidence for one clean-room commit.
///
/// This helper composes Chunk 6A tick validation and Chunk 6B atomic commit with
/// deterministic ordering plus before/after state digests. It does not persist a
/// ledger, execute a scenario, validate conservation, or import GPL BioSim code.
pub fn prove_biosim_resource_replay(
    previous: BioSimTick,
    next: BioSimTick,
    current_state: &[BioSimResourceQuantity],
    deltas: &[BioSimResourceDelta],
) -> AeroResult<EngineeringResult<BioSimResourceReplayProof>> {
    let tick = validate_biosim_tick_advance(previous, next)?.value;
    let before_digest = digest_biosim_resource_state(previous, current_state)?.value;
    let ordered_deltas = ordered_biosim_resource_deltas(deltas)?;
    let delta_digest = digest_ordered_resource_deltas(next.index, &ordered_deltas);
    let commit = commit_biosim_resource_transaction(previous, next, current_state, deltas)?.value;
    let after_digest = digest_biosim_resource_state(next, &commit.balances)?.value;

    Ok(EngineeringResult::new(
        BioSimResourceReplayProof {
            tick,
            before_digest,
            after_digest,
            delta_digest,
            ordered_delta_count: ordered_deltas.len(),
        },
        biosim_deterministic_replay_codex_id(),
        resource_tick_record(biosim_deterministic_replay_codex_id()),
    )
    .with_assumption(
        "biosim_rs.deterministic_replay_digest_only",
        "proof covers deterministic ordering and before/after digests for one caller-supplied resource transaction only",
    )
    .with_assumption(
        "biosim_rs.no_ledger_or_conservation_claim",
        "proof is not a persistent ledger, O2-loop conservation check, scenario execution, or habitat-control validation",
    )
    .with_validity(ValidityStatus::WithinDocumentedDomain))
}

/// Applies all resource deltas at one validated tick boundary or rejects the whole commit.
///
/// This helper is intentionally a pure, caller-state-in/caller-state-out atomic
/// operation. It validates the input state and complete delta set before exposing
/// a committed output snapshot. It does not persist a ledger, prove deterministic
/// replay, validate mass conservation, execute scenarios, or model biological
/// control behavior.
pub fn commit_biosim_resource_transaction(
    previous: BioSimTick,
    next: BioSimTick,
    current_state: &[BioSimResourceQuantity],
    deltas: &[BioSimResourceDelta],
) -> AeroResult<EngineeringResult<BioSimResourceTransactionCommit>> {
    let tick = validate_biosim_tick_advance(previous, next)?.value;
    validate_biosim_resource_state(current_state)?;
    validate_biosim_resource_deltas(deltas)?;

    let mut balances = current_state.to_vec();
    for delta in deltas {
        let balance = balances
            .iter_mut()
            .find(|balance| balance.kind == delta.kind)
            .ok_or_else(|| {
                out_of_domain(
                    "resource_delta",
                    deltas.len() as f64,
                    "delta resource must exist in the current resource state",
                )
            })?;
        let committed_amount = balance.amount + delta.delta_amount;
        validation::ensure_finite("resource_balance", committed_amount)?;
        if committed_amount < 0.0 {
            return Err(out_of_domain(
                "resource_balance",
                committed_amount,
                "nonnegative post-commit resource balance",
            ));
        }
        balance.amount = committed_amount;
    }

    Ok(EngineeringResult::new(
        BioSimResourceTransactionCommit {
            tick,
            balances,
            delta_count: deltas.len(),
        },
        biosim_transaction_commit_codex_id(),
        resource_tick_record(biosim_transaction_commit_codex_id()),
    )
    .with_assumption(
        "biosim_rs.atomic_commit_only",
        "all resource deltas are applied to an output snapshot or the transaction is rejected before exposing a commit",
    )
    .with_assumption(
        "biosim_rs.no_ledger_or_replay_proof",
        "commit output is not a persistent ledger entry, deterministic replay proof, or conservation validation",
    )
    .with_validity(ValidityStatus::WithinDocumentedDomain))
}

fn validate_biosim_resource_state(state: &[BioSimResourceQuantity]) -> AeroResult<()> {
    if state.is_empty() {
        return Err(out_of_domain(
            "resource_state",
            0.0,
            "at least one resource balance before transaction commit",
        ));
    }

    for (index, balance) in state.iter().enumerate() {
        validation::ensure_nonnegative("resource_balance", balance.amount)?;
        if state[..index]
            .iter()
            .any(|prior| prior.kind == balance.kind)
        {
            return Err(out_of_domain(
                "resource_state",
                index as f64,
                "unique resource balances before transaction commit",
            ));
        }
    }
    Ok(())
}

fn validate_biosim_resource_deltas(deltas: &[BioSimResourceDelta]) -> AeroResult<()> {
    if deltas.is_empty() {
        return Err(out_of_domain(
            "resource_delta",
            0.0,
            "at least one resource delta in an atomic transaction commit",
        ));
    }

    for (index, delta) in deltas.iter().enumerate() {
        validation::ensure_finite("resource_delta", delta.delta_amount)?;
        if deltas[..index].iter().any(|prior| prior.kind == delta.kind) {
            return Err(out_of_domain(
                "resource_delta",
                index as f64,
                "at most one delta per resource in this clean-room transaction slice",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aero_codex_core::{ValidityStatus, VerificationStatus};

    #[test]
    fn resource_identity_catalog_rejects_duplicates() {
        let duplicate = [BioSimResourceKind::OxygenGas, BioSimResourceKind::OxygenGas];

        let err = validate_biosim_resource_catalog(&duplicate).unwrap_err();

        assert_eq!(err.code(), "out_of_domain");
        assert_eq!(err.parameter(), Some("resource_catalog"));
    }

    #[test]
    fn resource_identity_catalog_accepts_unique_clean_room_resources() {
        let catalog = [
            BioSimResourceKind::OxygenGas,
            BioSimResourceKind::CarbonDioxideGas,
            BioSimResourceKind::PotableWater,
        ];

        let result = validate_biosim_resource_catalog(&catalog).unwrap();

        assert_eq!(result.value, 3);
        assert_eq!(result.codex_id, biosim_resource_catalog_codex_id());
        assert_eq!(
            result.verification_status(),
            VerificationStatus::ResearchRequired
        );
        assert_eq!(result.validity, ValidityStatus::WithinDocumentedDomain);
        assert!(result
            .assumptions
            .iter()
            .any(|item| item.id == "biosim_rs.clean_room_chunk6a"));
        assert_eq!(
            biosim_resource_identity(BioSimResourceKind::OxygenGas).canonical_id,
            "biosim_rs.resource.oxygen_gas"
        );
    }

    #[test]
    fn tick_validation_rejects_nonpositive_duration() {
        let err = validate_biosim_tick(0, 0.0).unwrap_err();

        assert_eq!(err.code(), "non_positive_input");
        assert_eq!(err.parameter(), Some("tick_duration_seconds"));
    }

    #[test]
    fn tick_validation_marks_zero_as_boundary_case() {
        let result = validate_biosim_tick(0, 60.0).unwrap();

        assert_eq!(result.value.index, 0);
        assert_eq!(result.value.duration_seconds, 60.0);
        assert_eq!(result.codex_id, biosim_tick_validation_codex_id());
        assert_eq!(result.validity, ValidityStatus::BoundaryCase);
        assert!(result.has_warnings());
    }

    #[test]
    fn tick_advance_requires_consecutive_ticks() {
        let previous = validate_biosim_tick(3, 60.0).unwrap().value;
        let next = validate_biosim_tick(5, 60.0).unwrap().value;

        let err = validate_biosim_tick_advance(previous, next).unwrap_err();

        assert_eq!(err.code(), "out_of_domain");
        assert_eq!(err.parameter(), Some("next_tick_index"));
    }

    #[test]
    fn tick_advance_accepts_consecutive_ticks_with_positive_duration() {
        let previous = validate_biosim_tick(3, 60.0).unwrap().value;
        let next = validate_biosim_tick(4, 30.0).unwrap().value;

        let result = validate_biosim_tick_advance(previous, next).unwrap();

        assert_eq!(result.value.previous_index, 3);
        assert_eq!(result.value.next_index, 4);
        assert_eq!(result.value.next_duration_seconds, 30.0);
        assert_eq!(result.validity, ValidityStatus::WithinDocumentedDomain);
        assert_eq!(
            result.verification_status(),
            VerificationStatus::ResearchRequired
        );
    }

    #[test]
    fn atomic_transaction_commit_applies_all_resource_deltas_at_one_tick_boundary() {
        let previous = validate_biosim_tick(6, 60.0).unwrap().value;
        let next = validate_biosim_tick(7, 60.0).unwrap().value;
        let state = [
            BioSimResourceQuantity {
                kind: BioSimResourceKind::OxygenGas,
                amount: 10.0,
            },
            BioSimResourceQuantity {
                kind: BioSimResourceKind::PotableWater,
                amount: 5.0,
            },
        ];
        let deltas = [
            BioSimResourceDelta {
                kind: BioSimResourceKind::OxygenGas,
                delta_amount: -1.5,
            },
            BioSimResourceDelta {
                kind: BioSimResourceKind::PotableWater,
                delta_amount: 2.0,
            },
        ];

        let result = commit_biosim_resource_transaction(previous, next, &state, &deltas).unwrap();

        assert_eq!(result.codex_id, biosim_transaction_commit_codex_id());
        assert_eq!(
            result.verification_status(),
            VerificationStatus::ResearchRequired
        );
        assert_eq!(result.value.tick.previous_index, 6);
        assert_eq!(result.value.tick.next_index, 7);
        assert_eq!(result.value.delta_count, 2);
        assert_eq!(
            result
                .value
                .balances
                .iter()
                .find(|balance| balance.kind == BioSimResourceKind::OxygenGas)
                .unwrap()
                .amount,
            8.5
        );
        assert_eq!(
            result
                .value
                .balances
                .iter()
                .find(|balance| balance.kind == BioSimResourceKind::PotableWater)
                .unwrap()
                .amount,
            7.0
        );
        assert!(result
            .assumptions
            .iter()
            .any(|item| item.id == "biosim_rs.atomic_commit_only"));
    }

    #[test]
    fn atomic_transaction_commit_rejects_overdraft_without_mutating_caller_state() {
        let previous = validate_biosim_tick(10, 60.0).unwrap().value;
        let next = validate_biosim_tick(11, 60.0).unwrap().value;
        let state = [BioSimResourceQuantity {
            kind: BioSimResourceKind::OxygenGas,
            amount: 1.0,
        }];
        let deltas = [BioSimResourceDelta {
            kind: BioSimResourceKind::OxygenGas,
            delta_amount: -2.0,
        }];

        let err = commit_biosim_resource_transaction(previous, next, &state, &deltas).unwrap_err();

        assert_eq!(err.code(), "out_of_domain");
        assert_eq!(err.parameter(), Some("resource_balance"));
        assert_eq!(state[0].amount, 1.0);
    }

    #[test]
    fn atomic_transaction_commit_rejects_unknown_delta_resource() {
        let previous = validate_biosim_tick(1, 60.0).unwrap().value;
        let next = validate_biosim_tick(2, 60.0).unwrap().value;
        let state = [BioSimResourceQuantity {
            kind: BioSimResourceKind::OxygenGas,
            amount: 1.0,
        }];
        let deltas = [BioSimResourceDelta {
            kind: BioSimResourceKind::PotableWater,
            delta_amount: 1.0,
        }];

        let err = commit_biosim_resource_transaction(previous, next, &state, &deltas).unwrap_err();

        assert_eq!(err.code(), "out_of_domain");
        assert_eq!(err.parameter(), Some("resource_delta"));
    }

    #[test]
    fn atomic_transaction_commit_rejects_duplicate_state_resources() {
        let previous = validate_biosim_tick(2, 60.0).unwrap().value;
        let next = validate_biosim_tick(3, 60.0).unwrap().value;
        let state = [
            BioSimResourceQuantity {
                kind: BioSimResourceKind::OxygenGas,
                amount: 1.0,
            },
            BioSimResourceQuantity {
                kind: BioSimResourceKind::OxygenGas,
                amount: 2.0,
            },
        ];
        let deltas = [BioSimResourceDelta {
            kind: BioSimResourceKind::OxygenGas,
            delta_amount: 1.0,
        }];

        let err = commit_biosim_resource_transaction(previous, next, &state, &deltas).unwrap_err();

        assert_eq!(err.code(), "out_of_domain");
        assert_eq!(err.parameter(), Some("resource_state"));
    }

    #[test]
    fn atomic_transaction_commit_rejects_duplicate_delta_resources() {
        let previous = validate_biosim_tick(4, 60.0).unwrap().value;
        let next = validate_biosim_tick(5, 60.0).unwrap().value;
        let state = [BioSimResourceQuantity {
            kind: BioSimResourceKind::OxygenGas,
            amount: 3.0,
        }];
        let deltas = [
            BioSimResourceDelta {
                kind: BioSimResourceKind::OxygenGas,
                delta_amount: 1.0,
            },
            BioSimResourceDelta {
                kind: BioSimResourceKind::OxygenGas,
                delta_amount: -1.0,
            },
        ];

        let err = commit_biosim_resource_transaction(previous, next, &state, &deltas).unwrap_err();

        assert_eq!(err.code(), "out_of_domain");
        assert_eq!(err.parameter(), Some("resource_delta"));
    }

    #[test]
    fn atomic_transaction_commit_rejects_empty_delta_set() {
        let previous = validate_biosim_tick(6, 60.0).unwrap().value;
        let next = validate_biosim_tick(7, 60.0).unwrap().value;
        let state = [BioSimResourceQuantity {
            kind: BioSimResourceKind::OxygenGas,
            amount: 3.0,
        }];

        let err = commit_biosim_resource_transaction(previous, next, &state, &[]).unwrap_err();

        assert_eq!(err.code(), "out_of_domain");
        assert_eq!(err.parameter(), Some("resource_delta"));
    }

    #[test]
    fn resource_state_digest_is_stable_for_unsorted_equivalent_state() {
        let tick = validate_biosim_tick(12, 60.0).unwrap().value;
        let state_a = [
            BioSimResourceQuantity {
                kind: BioSimResourceKind::PotableWater,
                amount: 4.25,
            },
            BioSimResourceQuantity {
                kind: BioSimResourceKind::OxygenGas,
                amount: 8.5,
            },
        ];
        let state_b = [state_a[1], state_a[0]];

        let digest_a = digest_biosim_resource_state(tick, &state_a).unwrap();
        let digest_b = digest_biosim_resource_state(tick, &state_b).unwrap();

        assert_eq!(digest_a.codex_id, biosim_deterministic_replay_codex_id());
        assert_eq!(digest_a.value, digest_b.value);
        assert_eq!(digest_a.value.algorithm, "fnv1a64:biosim_resource_state:v1");
        assert_eq!(digest_a.value.tick_index, 12);
        assert_eq!(digest_a.value.value.len(), 16);
        assert_eq!(
            digest_a.verification_status(),
            VerificationStatus::ResearchRequired
        );
    }

    #[test]
    fn resource_state_digest_changes_when_initial_level_changes() {
        let tick = validate_biosim_tick(0, 60.0).unwrap().value;
        let base = [BioSimResourceQuantity {
            kind: BioSimResourceKind::OxygenGas,
            amount: 8.5,
        }];
        let changed = [BioSimResourceQuantity {
            kind: BioSimResourceKind::OxygenGas,
            amount: 8.75,
        }];

        let base_digest = digest_biosim_resource_state(tick, &base).unwrap();
        let changed_digest = digest_biosim_resource_state(tick, &changed).unwrap();

        assert_ne!(base_digest.value.value, changed_digest.value.value);
    }

    #[test]
    fn replay_proof_is_stable_for_equivalent_unsorted_inputs() {
        let previous = validate_biosim_tick(7, 60.0).unwrap().value;
        let next = validate_biosim_tick(8, 60.0).unwrap().value;
        let state_a = [
            BioSimResourceQuantity {
                kind: BioSimResourceKind::PotableWater,
                amount: 4.25,
            },
            BioSimResourceQuantity {
                kind: BioSimResourceKind::OxygenGas,
                amount: 8.5,
            },
        ];
        let state_b = [state_a[1], state_a[0]];
        let deltas_a = [
            BioSimResourceDelta {
                kind: BioSimResourceKind::OxygenGas,
                delta_amount: -1.0,
            },
            BioSimResourceDelta {
                kind: BioSimResourceKind::PotableWater,
                delta_amount: 2.0,
            },
        ];
        let deltas_b = [deltas_a[1], deltas_a[0]];

        let proof_a = prove_biosim_resource_replay(previous, next, &state_a, &deltas_a).unwrap();
        let proof_b = prove_biosim_resource_replay(previous, next, &state_b, &deltas_b).unwrap();

        assert_eq!(proof_a.codex_id, biosim_deterministic_replay_codex_id());
        assert_eq!(proof_a.value, proof_b.value);
        assert_eq!(proof_a.value.tick.previous_index, 7);
        assert_eq!(proof_a.value.tick.next_index, 8);
        assert_eq!(proof_a.value.before_digest.tick_index, 7);
        assert_eq!(proof_a.value.after_digest.tick_index, 8);
        assert_eq!(proof_a.value.ordered_delta_count, 2);
        assert_ne!(proof_a.value.before_digest, proof_a.value.after_digest);
        assert!(proof_a
            .assumptions
            .iter()
            .any(|item| item.id == "biosim_rs.deterministic_replay_digest_only"));
    }

    #[test]
    fn failed_replay_proof_preserves_caller_state_digest_and_tick() {
        let previous = validate_biosim_tick(9, 60.0).unwrap().value;
        let next = validate_biosim_tick(10, 60.0).unwrap().value;
        let state = [BioSimResourceQuantity {
            kind: BioSimResourceKind::OxygenGas,
            amount: 1.0,
        }];
        let deltas = [BioSimResourceDelta {
            kind: BioSimResourceKind::OxygenGas,
            delta_amount: -2.0,
        }];
        let before_digest = digest_biosim_resource_state(previous, &state).unwrap();

        let err = prove_biosim_resource_replay(previous, next, &state, &deltas).unwrap_err();
        let after_digest = digest_biosim_resource_state(previous, &state).unwrap();

        assert_eq!(err.code(), "out_of_domain");
        assert_eq!(before_digest.value, after_digest.value);
        assert_eq!(previous.index, 9);
        assert_eq!(state[0].amount, 1.0);
    }
}
