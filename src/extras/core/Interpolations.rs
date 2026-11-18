/*!
 * Interpolations contains spline and Bézier functions internally used by concrete curve classes.
 *
 * Bezier Curves formulas obtained from: https://en.wikipedia.org/wiki/B%C3%A9zier_curve
 *
 * @module Interpolations
 */

/**
 * Computes a point on a Catmull-Rom spline.
 *
 * @param {number} t - The interpolation factor.
 * @param {number} p0 - The first control point.
 * @param {number} p1 - The second control point.
 * @param {number} p2 - The third control point.
 * @param {number} p3 - The fourth control point.
 * @return {number} The calculated point on a Catmull-Rom spline.
 */
#[cfg_attr(rust_js, rust_js::name = "CatmullRom")]
pub fn catmull_rom(t: f64, p0: f64, p1: f64, p2: f64, p3: f64) -> f64 {
    let v0 = (p2 - p0) * 0.5;
    let v1 = (p3 - p1) * 0.5;
    let t2 = t * t;
    let t3 = t * t2;
    (2.0 * p1 - 2.0 * p2 + v0 + v1) * t3 + (-3.0 * p1 + 3.0 * p2 - 2.0 * v0 - v1) * t2 + v0 * t + p1
}

//

#[cfg_attr(rust_js, rust_js::name = "QuadraticBezierP0")]
fn quadratic_bezier_p0(t: f64, p: f64) -> f64 {
    let k = 1.0 - t;
    k * k * p
}

#[cfg_attr(rust_js, rust_js::name = "QuadraticBezierP1")]
fn quadratic_bezier_p1(t: f64, p: f64) -> f64 {
    2.0 * (1.0 - t) * t * p
}

#[cfg_attr(rust_js, rust_js::name = "QuadraticBezierP2")]
fn quadratic_bezier_p2(t: f64, p: f64) -> f64 {
    t * t * p
}

/**
 * Computes a point on a Quadratic Bezier curve.
 *
 * @param {number} t - The interpolation factor.
 * @param {number} p0 - The first control point.
 * @param {number} p1 - The second control point.
 * @param {number} p2 - The third control point.
 * @return {number} The calculated point on a Quadratic Bezier curve.
 */
#[cfg_attr(rust_js, rust_js::name = "QuadraticBezier")]
pub fn quadratic_bezier(t: f64, p0: f64, p1: f64, p2: f64) -> f64 {
    quadratic_bezier_p0(t, p0) + quadratic_bezier_p1(t, p1) + quadratic_bezier_p2(t, p2)
}

//

#[cfg_attr(rust_js, rust_js::name = "CubicBezierP0")]
fn cubic_bezier_p0(t: f64, p: f64) -> f64 {
    let k = 1.0 - t;
    k * k * k * p
}

#[cfg_attr(rust_js, rust_js::name = "CubicBezierP1")]
fn cubic_bezier_p1(t: f64, p: f64) -> f64 {
    let k = 1.0 - t;
    3.0 * k * k * t * p
}

#[cfg_attr(rust_js, rust_js::name = "CubicBezierP2")]
fn cubic_bezier_p2(t: f64, p: f64) -> f64 {
    3.0 * (1.0 - t) * t * t * p
}

#[cfg_attr(rust_js, rust_js::name = "CubicBezierP3")]
fn cubic_bezier_p3(t: f64, p: f64) -> f64 {
    t * t * t * p
}

/**
 * Computes a point on a Cubic Bezier curve.
 *
 * @param {number} t - The interpolation factor.
 * @param {number} p0 - The first control point.
 * @param {number} p1 - The second control point.
 * @param {number} p2 - The third control point.
 * @param {number} p3 - The fourth control point.
 * @return {number} The calculated point on a Cubic Bezier curve.
 */
#[cfg_attr(rust_js, rust_js::name = "CubicBezier")]
pub fn cubic_bezier(t: f64, p0: f64, p1: f64, p2: f64, p3: f64) -> f64 {
    cubic_bezier_p0(t, p0)
        + cubic_bezier_p1(t, p1)
        + cubic_bezier_p2(t, p2)
        + cubic_bezier_p3(t, p3)
}
