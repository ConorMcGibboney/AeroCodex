#![forbid(unsafe_code)]
//! Numerical solver interfaces and starter algorithms.

use aero_codex_core::{AeroError, AeroResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bracket {
    pub lower: f64,
    pub upper: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RootResult {
    pub root: f64,
    pub residual: f64,
    pub iterations: usize,
    pub converged: bool,
    pub method: &'static str,
}

pub trait ScalarRootSolver {
    fn solve<F>(&self, f: F, bracket: Bracket) -> AeroResult<RootResult>
    where
        F: Fn(f64) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bisection {
    tolerance: f64,
    max_iterations: usize,
}

impl Bisection {
    #[must_use]
    pub const fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self {
            tolerance,
            max_iterations,
        }
    }
}

impl Default for Bisection {
    fn default() -> Self {
        Self::new(1.0e-12, 200)
    }
}

impl ScalarRootSolver for Bisection {
    fn solve<F>(&self, f: F, bracket: Bracket) -> AeroResult<RootResult>
    where
        F: Fn(f64) -> f64,
    {
        bisection(f, bracket, self.tolerance, self.max_iterations)
    }
}

pub fn bisection<F>(
    f: F,
    bracket: Bracket,
    tolerance: f64,
    max_iterations: usize,
) -> AeroResult<RootResult>
where
    F: Fn(f64) -> f64,
{
    if !bracket.lower.is_finite() || !bracket.upper.is_finite() || bracket.lower >= bracket.upper {
        return Err(AeroError::InvalidInput {
            parameter: "bracket",
            value: bracket.lower,
            reason: "bracket endpoints must be finite and ordered",
        });
    }
    if !tolerance.is_finite() || tolerance <= 0.0 {
        return Err(AeroError::InvalidInput {
            parameter: "tolerance",
            value: tolerance,
            reason: "tolerance must be finite and positive",
        });
    }

    let mut lo = bracket.lower;
    let mut hi = bracket.upper;
    let mut flo = f(lo);
    let fhi = f(hi);

    if !(flo.is_finite() && fhi.is_finite()) {
        return Err(AeroError::InvalidInput {
            parameter: "function",
            value: f64::NAN,
            reason: "function returned non-finite value at bracket endpoint",
        });
    }

    if flo == 0.0 {
        return Ok(RootResult {
            root: lo,
            residual: 0.0,
            iterations: 0,
            converged: true,
            method: "bisection",
        });
    }
    if fhi == 0.0 {
        return Ok(RootResult {
            root: hi,
            residual: 0.0,
            iterations: 0,
            converged: true,
            method: "bisection",
        });
    }
    if flo * fhi > 0.0 {
        return Err(AeroError::InvalidInput {
            parameter: "bracket",
            value: bracket.lower,
            reason: "function values at bracket endpoints must have opposite signs",
        });
    }

    for iterations in 1..=max_iterations {
        let mid = 0.5 * (lo + hi);
        let fmid = f(mid);
        if !fmid.is_finite() {
            return Err(AeroError::InvalidInput {
                parameter: "function",
                value: fmid,
                reason: "function returned non-finite value during iteration",
            });
        }
        if fmid.abs() <= tolerance || 0.5 * (hi - lo).abs() <= tolerance {
            return Ok(RootResult {
                root: mid,
                residual: fmid.abs(),
                iterations,
                converged: true,
                method: "bisection",
            });
        }
        if flo * fmid <= 0.0 {
            hi = mid;
        } else {
            lo = mid;
            flo = fmid;
        }
    }

    let mid = 0.5 * (lo + hi);
    let residual = f(mid).abs();
    Err(AeroError::NoConvergence {
        solver: "bisection",
        iterations: max_iterations,
        residual,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bisection_solves_square_root_two() {
        let result = bisection(
            |x| x * x - 2.0,
            Bracket {
                lower: 1.0,
                upper: 2.0,
            },
            1.0e-12,
            200,
        )
        .expect("root should converge");
        assert!((result.root - 2.0_f64.sqrt()).abs() < 1.0e-10);
        assert!(result.converged);
    }

    #[test]
    fn bisection_rejects_invalid_bracket() {
        let err = bisection(
            |x| x * x + 1.0,
            Bracket {
                lower: -1.0,
                upper: 1.0,
            },
            1.0e-12,
            10,
        )
        .unwrap_err();
        assert!(matches!(err, AeroError::InvalidInput { .. }));
    }
}
