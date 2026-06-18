//! Deterministic friend-test smoke example for the astrodynamics foundation.
//!
//! Run with `cargo run -p aero-codex-astrodynamics --example astrodynamics_foundation_smoke`.
//!
//! The scenario is synthetic and uses SI units. It exercises label-preserving
//! time arithmetic, same-frame identity, finite Cartesian-state validation,
//! bounded non-singular elliptic element extraction, and a bounded elliptic
//! Kepler solve. It does not perform time-scale conversion, a non-identity frame
//! transform, TLE parsing, SGP4, perturbation modeling, orbit determination, or
//! external-oracle execution.

use aero_codex_astrodynamics::elements::{
    compute_orbital_elements_basic, compute_specific_angular_momentum,
    compute_specific_orbital_energy_from_state, OrbitalElementTolerances,
};
use aero_codex_astrodynamics::frames::{same_frame_identity_contract, AstroFrame};
use aero_codex_astrodynamics::kepler::{
    mean_anomaly_advance, mean_motion, radius_from_semimajor_eccentric_anomaly,
    solve_kepler_elliptic_newton, true_anomaly_from_eccentric_anomaly, KeplerSolveOptions,
};
use aero_codex_astrodynamics::state::{
    position_norm, validate_cartesian_state_for_orbital_use, velocity_norm, CartesianState,
    GravitationalParameter, StateWithFrameAndEpoch,
};
use aero_codex_astrodynamics::time::{AstroDuration, AstroEpoch, AstroTimeScale};
use std::error::Error;
use std::fmt::Write as _;

fn non_claim_caveat() -> &'static str {
    "research/preliminary-design smoke only; not operational tracking, flight, mission, navigation, or certification evidence"
}

fn earth_like_mu_m3_per_s2() -> f64 {
    3.986_004_418e14
}

fn initial_epoch_seconds() -> f64 {
    12_345.0
}

fn elapsed_seconds() -> f64 {
    120.0
}

fn initial_mean_anomaly_rad() -> f64 {
    0.4
}

#[derive(Debug, Clone, PartialEq)]
struct SmokeReportValues {
    frame_label: &'static str,
    time_scale_label: &'static str,
    initial_epoch_seconds: f64,
    later_epoch_seconds: f64,
    elapsed_seconds: f64,
    frame_contract_status: &'static str,
    position_norm_m: f64,
    velocity_norm_m_per_s: f64,
    specific_energy_m2_per_s2: f64,
    angular_momentum_m2_per_s: [f64; 3],
    semi_major_axis_m: f64,
    eccentricity: f64,
    inclination_rad: f64,
    raan_rad: f64,
    argument_of_periapsis_rad: f64,
    state_true_anomaly_rad: f64,
    mean_motion_rad_per_s: f64,
    advanced_mean_anomaly_rad: f64,
    solved_eccentric_anomaly_rad: f64,
    solver_iterations: u32,
    solver_absolute_residual_rad: f64,
    solved_true_anomaly_rad: f64,
    solved_radius_m: f64,
}

fn calculate_smoke_values() -> Result<SmokeReportValues, Box<dyn Error>> {
    let initial_epoch = AstroEpoch::new(AstroTimeScale::Tt, initial_epoch_seconds())?;
    let elapsed_duration = AstroDuration::seconds(elapsed_seconds())?;
    let later_epoch = initial_epoch.offset_by_duration(elapsed_duration)?;
    let measured_elapsed = later_epoch.duration_since_same_scale(initial_epoch)?;

    let frame = AstroFrame::InertialEciMeanEquator;
    let frame_contract = same_frame_identity_contract(frame);

    // Arbitrary hand-written synthetic state selected to be finite, elliptic,
    // non-circular, and non-equatorial. It is not an imported external fixture.
    let state = CartesianState::new(
        [7_000_000.0, 1_000_000.0, 2_000_000.0],
        [-1_000.0, 7_500.0, 1_000.0],
    )?;
    let labeled_state = StateWithFrameAndEpoch::new(state, frame, initial_epoch);
    let mu = GravitationalParameter::new(earth_like_mu_m3_per_s2())?;
    validate_cartesian_state_for_orbital_use(&labeled_state, mu)?;

    let position_velocity = labeled_state.position_velocity();
    let position_norm_m = position_norm(&position_velocity)?;
    let velocity_norm_m_per_s = velocity_norm(&position_velocity)?;
    let specific_energy_m2_per_s2 =
        compute_specific_orbital_energy_from_state(&position_velocity, mu)?;
    let angular_momentum_m2_per_s = compute_specific_angular_momentum(&position_velocity)?;

    // These are numerical exclusion bands for this synthetic friend test, not
    // accuracy claims or generally recommended mission-analysis tolerances.
    let element_tolerances = OrbitalElementTolerances::new(1.0, 1.0, 1.0e-12, 1.0e-6)?;
    let elements = compute_orbital_elements_basic(&position_velocity, mu, element_tolerances)?;

    let mean_motion_rad_per_s = mean_motion(mu, elements.semi_major_axis_m)?;
    let advanced_mean_anomaly_rad = mean_anomaly_advance(
        initial_mean_anomaly_rad(),
        mean_motion_rad_per_s,
        measured_elapsed,
    )?;
    let solve_options = KeplerSolveOptions::new(1.0e-12, 64)?;
    let solve_report = solve_kepler_elliptic_newton(
        advanced_mean_anomaly_rad,
        elements.eccentricity,
        solve_options,
    )?;
    let solved_true_anomaly_rad = true_anomaly_from_eccentric_anomaly(
        solve_report.eccentric_anomaly_rad,
        elements.eccentricity,
    )?;
    let solved_radius_m = radius_from_semimajor_eccentric_anomaly(
        elements.semi_major_axis_m,
        elements.eccentricity,
        solve_report.eccentric_anomaly_rad,
    )?;

    Ok(SmokeReportValues {
        frame_label: frame.label(),
        time_scale_label: initial_epoch.time_scale().label(),
        initial_epoch_seconds: initial_epoch.seconds_from_reference_epoch(),
        later_epoch_seconds: later_epoch.seconds_from_reference_epoch(),
        elapsed_seconds: measured_elapsed.as_seconds(),
        frame_contract_status: frame_contract.status().code(),
        position_norm_m,
        velocity_norm_m_per_s,
        specific_energy_m2_per_s2,
        angular_momentum_m2_per_s,
        semi_major_axis_m: elements.semi_major_axis_m,
        eccentricity: elements.eccentricity,
        inclination_rad: elements.inclination_rad,
        raan_rad: elements.raan_rad,
        argument_of_periapsis_rad: elements.argument_of_periapsis_rad,
        state_true_anomaly_rad: elements.true_anomaly_rad,
        mean_motion_rad_per_s,
        advanced_mean_anomaly_rad,
        solved_eccentric_anomaly_rad: solve_report.eccentric_anomaly_rad,
        solver_iterations: solve_report.iterations,
        solver_absolute_residual_rad: solve_report.absolute_residual_rad,
        solved_true_anomaly_rad,
        solved_radius_m,
    })
}

fn format_smoke_report(values: &SmokeReportValues) -> Result<String, std::fmt::Error> {
    let mut report = String::new();
    writeln!(report, "AeroCodex astrodynamics foundation smoke")?;
    writeln!(report, "status=research_required")?;
    writeln!(report, "scenario=synthetic_non_singular_elliptic_state")?;
    writeln!(report, "frame={}", values.frame_label)?;
    writeln!(report, "time_scale={}", values.time_scale_label)?;
    writeln!(
        report,
        "initial_epoch_seconds_from_reference={:.6}",
        values.initial_epoch_seconds
    )?;
    writeln!(
        report,
        "later_epoch_seconds_from_reference={:.6}",
        values.later_epoch_seconds
    )?;
    writeln!(
        report,
        "elapsed_scalar_seconds={:.6}",
        values.elapsed_seconds
    )?;
    writeln!(
        report,
        "frame_transform_contract={}",
        values.frame_contract_status
    )?;
    writeln!(report, "position_norm_m={:.6}", values.position_norm_m)?;
    writeln!(
        report,
        "velocity_norm_m_per_s={:.9}",
        values.velocity_norm_m_per_s
    )?;
    writeln!(
        report,
        "specific_orbital_energy_m2_per_s2={:.6}",
        values.specific_energy_m2_per_s2
    )?;
    writeln!(
        report,
        "specific_angular_momentum_m2_per_s=[{:.6},{:.6},{:.6}]",
        values.angular_momentum_m2_per_s[0],
        values.angular_momentum_m2_per_s[1],
        values.angular_momentum_m2_per_s[2]
    )?;
    writeln!(report, "semi_major_axis_m={:.6}", values.semi_major_axis_m)?;
    writeln!(report, "eccentricity={:.15}", values.eccentricity)?;
    writeln!(report, "inclination_rad={:.15}", values.inclination_rad)?;
    writeln!(report, "raan_rad={:.15}", values.raan_rad)?;
    writeln!(
        report,
        "argument_of_periapsis_rad={:.15}",
        values.argument_of_periapsis_rad
    )?;
    writeln!(
        report,
        "state_true_anomaly_rad={:.15}",
        values.state_true_anomaly_rad
    )?;
    writeln!(
        report,
        "mean_motion_rad_per_s={:.15}",
        values.mean_motion_rad_per_s
    )?;
    writeln!(
        report,
        "advanced_mean_anomaly_rad={:.15}",
        values.advanced_mean_anomaly_rad
    )?;
    writeln!(
        report,
        "solved_eccentric_anomaly_rad={:.15}",
        values.solved_eccentric_anomaly_rad
    )?;
    writeln!(
        report,
        "kepler_solver_iterations={}",
        values.solver_iterations
    )?;
    writeln!(
        report,
        "kepler_absolute_residual_rad={:.3e}",
        values.solver_absolute_residual_rad
    )?;
    writeln!(
        report,
        "solved_true_anomaly_rad={:.15}",
        values.solved_true_anomaly_rad
    )?;
    writeln!(report, "solved_radius_m={:.6}", values.solved_radius_m)?;
    writeln!(report, "caveat={}", non_claim_caveat())?;
    Ok(report)
}

fn build_smoke_report() -> Result<String, Box<dyn Error>> {
    let values = calculate_smoke_values()?;
    Ok(format_smoke_report(&values)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let report = build_smoke_report()?;
    print!("{report}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_report_is_byte_for_byte_stable() {
        let first = build_smoke_report().expect("first synthetic smoke report should build");
        let second = build_smoke_report().expect("second synthetic smoke report should build");
        assert_eq!(first, second);
    }

    #[test]
    fn smoke_report_contains_required_context_and_non_claim() {
        let report = build_smoke_report().expect("synthetic smoke report should build");
        assert!(report.contains("status=research_required"));
        assert!(report.contains("frame=ECI-mean-equator(label)"));
        assert!(report.contains("time_scale=TT"));
        assert!(report.contains("frame_transform_contract=identity_same_frame"));
        assert!(report.contains("kepler_absolute_residual_rad="));
        assert!(report.contains(non_claim_caveat()));
    }

    #[test]
    fn smoke_report_contains_no_nonfinite_values() {
        let report = build_smoke_report().expect("synthetic smoke report should build");
        assert!(!report.contains("NaN"));
        assert!(!report.contains("inf"));
        assert!(!report.contains("-inf"));
    }

    #[test]
    fn smoke_report_contains_no_environment_dependent_paths() {
        let report = build_smoke_report().expect("synthetic smoke report should build");
        let forbidden_prefixes = [
            ["/", "mnt", "/"].concat(),
            ["/", "home", "/"].concat(),
            ["C:", "\\", "Users", "\\"].concat(),
        ];
        for forbidden in forbidden_prefixes {
            assert!(!report.contains(&forbidden));
        }
    }
}
