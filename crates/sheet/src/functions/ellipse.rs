//! Partial, independently derived affine ellipse transforms for internal ShapeSheet helpers.
//!
//! Microsoft identifies these as internal functions and does not publish their branch contract:
//! https://learn.microsoft.com/en-us/office/client-developer/visio/about-functions
//! The geometry ratio describes ellipse axes:
//! https://learn.microsoft.com/en-us/office/client-developer/visio/d-cell-geometry-section
//!
//! Observed argument order: old angle, axis ratio, old width/height, new width/height,
//! plus the new axis angle for ECC. For S=diag(new_width/old_width,new_height/old_height),
//! Q=S R diag(ratio^2,1) R^T S describes the transformed ellipse. We derive its axes,
//! choosing a deterministic angle in (-pi,0], or zero for an exactly circular input.
//! ECC is the ratio of radii projected onto the supplied new angle and its perpendicular.
//!
//! Saved-cache numeric parity remains partial: identical THETA formulas in EXCOBJ_M.vssx,
//! master12 shape8 rows3/6 cache -pi and approximately zero; equivalent ellipse axes.
//! Identical ECC inputs in BPMN_ADDRESS_CHANGE_PROCESS_M.vstx master2 shape19 and
//! BPMN_M.vssx master6 shape19 cache reciprocal ratios. Neither discrepancy justifies
//! inferred hidden state or cached substitution; those oracle failures remain visible.
use crate::{Context, Error, Value, eval::eval, parser::Expr};
use std::f64::consts::PI;

pub const NAMES: &[&str] = &["_ELLIPSE_THETA", "_ELLIPSE_ECC"];
pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    if !NAMES.contains(&name) {
        return None;
    }
    Some(match calculate(name, args, context, budget, depth) {
        Ok(n) => Value::number(n),
        Err(e) => Value::Error(e),
    })
}
fn calculate(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<f64, Error> {
    let count = if name == "_ELLIPSE_THETA" { 6 } else { 7 };
    if args.len() != count {
        return Err(Error::Value);
    }
    let mut values = Vec::with_capacity(count);
    for arg in args {
        if matches!(arg, Expr::Missing) {
            return Err(Error::Value);
        }
        values.push(eval(arg, context, budget, depth + 1).numeric()?);
    }
    let [_, ratio, old_width, old_height, new_width, new_height, ..] = values.as_slice() else { return Err(Error::Value) };
    if !values.iter().all(|v| v.is_finite()) || [ratio, old_width, old_height].into_iter().any(|v| *v <= 0.) {
        return Err(Error::Num);
    }
    // Original synthetic M5.13e axis-collapse witnesses, including forced dependency
    // recalculation: THETA=0 and ECC=1 at the returned angle. Other degeneracies fail.
    if (*new_width == 0. && *new_height > 0.) || (*new_height == 0. && *new_width > 0.) {
        return if name == "_ELLIPSE_THETA" {
            Ok(0.)
        } else if values.get(6) == Some(&0.) {
            Ok(1.)
        } else {
            Err(Error::Num)
        };
    }
    let basis = Basis::new(&values)?;
    if name == "_ELLIPSE_THETA" { Ok(basis.angle()) } else { basis.ratio(*values.get(6).ok_or(Error::Value)?) }
}
#[derive(Clone, Copy)]
struct Basis {
    u: (f64, f64),
    v: (f64, f64),
    circle: bool,
}
impl Basis {
    fn new(values: &[f64]) -> Result<Self, Error> {
        let [angle, ratio, old_width, old_height, new_width, new_height, ..] = values else { return Err(Error::Value) };
        if !values.iter().all(|v| v.is_finite()) || [ratio, old_width, old_height, new_width, new_height].into_iter().any(|v| *v <= 0.) {
            return Err(Error::Num);
        }
        // A common scale cannot change eigenvectors or the axis ratio. Log scaling avoids
        // overflow in hostile dimension ratios without constructing a huge intermediate Q.
        let x = new_width.ln() - old_width.ln();
        let y = new_height.ln() - old_height.ln();
        let r = ratio.ln();
        let scale = (x + r).max(y + r).max(x).max(y);
        let (s, c) = angle.sin_cos();
        Ok(Self {
            u: (((x + r) - scale).exp() * c, ((y + r) - scale).exp() * s),
            v: (-(x - scale).exp() * s, (y - scale).exp() * c),
            circle: *ratio == 1.,
        })
    }
    fn angle(self) -> f64 {
        if self.circle {
            return 0.;
        }
        let xx = self.u.0 * self.u.0 + self.v.0 * self.v.0;
        let yy = self.u.1 * self.u.1 + self.v.1 * self.v.1;
        let xy = self.u.0 * self.u.1 + self.v.0 * self.v.1;
        let angle = (2. * xy).atan2(xx - yy) * 0.5;
        if angle > 0. { angle - PI } else { angle }
    }
    fn ratio(self, angle: f64) -> Result<f64, Error> {
        if !angle.is_finite() {
            return Err(Error::Num);
        }
        let (s, c) = angle.sin_cos();
        // Project the two independent ellipse basis vectors before taking their norm;
        // this avoids subtraction of nearly equal covariance terms for narrow ellipses.
        let along = (self.u.0 * c + self.u.1 * s).hypot(self.v.0 * c + self.v.1 * s);
        let across = (-self.u.0 * s + self.u.1 * c).hypot(-self.v.0 * s + self.v.1 * c);
        let ratio = along / across;
        if ratio.is_finite() && ratio > 0. { Ok(ratio) } else { Err(Error::Num) }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    struct Test;
    impl Context for Test {
        fn resolve(&mut self, _: &str) -> Value {
            Value::Error(Error::Ref)
        }
    }
    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-8, "{a} != {b}");
    }
    #[test]
    fn affine_axes_diagonalize_covariance_and_scale_cancels() {
        for angle in [-2.9, -1.3, -0.2, 0.3] {
            for ratio in [0.5, 1.1, 3.] {
                let b = Basis::new(&[angle, ratio, 2., 3., 7., 5.]).unwrap();
                let theta = b.angle();
                let (s, c) = theta.sin_cos();
                let cross = (b.u.0 * c + b.u.1 * s) * (-b.u.0 * s + b.u.1 * c) + (b.v.0 * c + b.v.1 * s) * (-b.v.0 * s + b.v.1 * c);
                near(cross, 0.);
                assert!(b.ratio(theta).unwrap() >= 1. - 1e-12);
                let scaled = Basis::new(&[angle, ratio, 2., 3., 70., 50.]).unwrap();
                near(theta, scaled.angle());
                near(b.ratio(theta).unwrap(), scaled.ratio(scaled.angle()).unwrap());
            }
        }
        near(Basis::new(&[0., 1., 1., 1., 2., 1.]).unwrap().angle(), 0.);
        near(Basis::new(&[0., 1., 1., 1., 2., 1.]).unwrap().ratio(0.).unwrap(), 2.);
    }
    #[test]
    #[allow(clippy::approx_constant)] // Preserve the exact rounded angle in the unmodified witness formula.
    fn matching_observed_noncircular_and_circle_witnesses() {
        // ADO_M.vssx master18 shape12 Geometry1.C2/D2.
        let b =
            Basis::new(&[-1.5707963267949, 1.1870703302777, 0.89172794117647, 0.52069672131147, 0.1350610114308932, 0.08588544821320392]).unwrap();
        near(b.angle(), -PI / 2.);
        near(b.ratio(b.angle()).unwrap(), 1.292747096369485);
        // ADO_M.vssx master19 shape8 Geometry1.C4/D4: horizontal minor axis retained.
        let b = Basis::new(&[PI, 1., 0.16272208691205, 0.096986043456095, 0.05729339019739676, 0.03572422488577366]).unwrap();
        near(b.angle(), 0.);
        near(b.ratio(b.angle()).unwrap(), 0.9558823529411764);
        assert_eq!(crate::formula("_ELLIPSE_THETA(0,1,1,1,2,1)", &mut Test), Value::Number(0.));
        assert_eq!(crate::formula("_ELLIPSE_ECC(0,1,1,1,2,1,0)", &mut Test), Value::Number(2.));
    }
    #[test]
    fn contradictory_saved_caches_remain_explicit_mismatches() {
        let b = Basis::new(&[-1.2566186986967, 1., 0.17529687499994, 0.17448437500004, 0.1527777777777778, 0.1527777777777778]).unwrap();
        let ratio = b.ratio(b.angle()).unwrap();
        near(ratio, 0.9953650057943116); // BPMN_M master6 shape19 D3.
        assert!((ratio - 1.004656577415021).abs() > 1e-6); // Contradictory BPMN template cache remains a failure.
    }
    #[test]
    fn hostile_and_degenerate_dimensions_return_errors() {
        for bad in [0., -1., f64::INFINITY, f64::NAN] {
            assert!(Basis::new(&[0., bad, 1., 1., 1., 1.]).is_err());
            assert!(Basis::new(&[0., 1., bad, 1., 1., 1.]).is_err());
        }
        let b = Basis::new(&[0., 2., 1e-300, 1e-300, 1e300, 1e300]).unwrap();
        near(b.angle(), 0.);
        near(b.ratio(0.).unwrap(), 2.);
        assert!(b.ratio(f64::NAN).is_err());
        for formula in ["_ELLIPSE_THETA(0,1,1,1,1)", "_ELLIPSE_ECC(0,1,1,1,1,1)", "_ELLIPSE_THETA(,1,1,1,1,1)", "_ELLIPSE_THETA(0,1,0,1,1,1)"] {
            assert!(matches!(crate::formula(formula, &mut Test), Value::Error(_)));
        }
    }
    #[test]
    fn witnessed_axis_collapse_returns_zero_theta_and_unit_ecc() {
        for dimensions in ["0,2", "2,0"] {
            for ratio in ["1", "1.0000202209969"] {
                assert_eq!(crate::formula(&format!("_ELLIPSE_THETA(0,{ratio},19,2,{dimensions})"), &mut Test), Value::Number(0.));
                assert_eq!(crate::formula(&format!("_ELLIPSE_ECC(0,{ratio},19,2,{dimensions},0)"), &mut Test), Value::Number(1.));
            }
        }
        for formula in [
            "_ELLIPSE_THETA(0,1,19,2,0,0)",
            "_ELLIPSE_THETA(0,1,0,2,0,2)",
            "_ELLIPSE_THETA(0,-1,19,2,0,2)",
            "_ELLIPSE_THETA(0,1,19,2,-1,2)",
            "_ELLIPSE_ECC(0,1,19,2,0,2,0.5)",
        ] {
            assert_eq!(crate::formula(formula, &mut Test), Value::Error(Error::Num), "{formula}");
        }
    }
}
