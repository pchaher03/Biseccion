use crate::models::BisectionResult;
use meval::Expr;

pub fn evaluate_polynomial(poly: &str, x: f64) -> Option<f64> {
    let parsed_expr = poly.parse::<Expr>().ok()?;
    let func = parsed_expr.bind("x").ok()?;
    let result = func(x);

    if result.is_nan() || result.is_infinite() {
        None
    } else {
        Some(result)
    }
}

pub fn is_continuous_in_interval(poly: &str, a: f64, b: f64) -> bool {
    let check_points = [a, (2.0 * a + b) / 3.0, (a + b) / 2.0, (a + 2.0 * b) / 3.0, b];

    for &x in &check_points {
        if x <= 0.0 && poly.contains("log(x)") {
            println!("⚠ Discontinuidad detectada en x = {}", x);
            return false;
        }
        if evaluate_polynomial(poly, x).is_none() {
            println!("⚠ Discontinuidad detectada en x = {}", x);
            return false;
        }
    }
    true
}

pub fn bisection_method(poly: &str, mut a: f64, mut b: f64) -> Option<BisectionResult> {
    let mut mid;
    let mut iterations = 0;
    let tol = 0.01;

    let f_a = evaluate_polynomial(poly, a)?;
    let f_b = evaluate_polynomial(poly, b)?;

    // ✅ **Verificar discontinuidad SOLO en puntos clave**
    if !is_continuous_in_interval(poly, a, b) {
        return None;
    }

    // ✅ **Verificar cambio de signo**
    if f_a * f_b > 0.0 {
        return None;
    }

    // 🏁 **Método de bisección**
    while (b - a).abs() >= tol {
        mid = (a + b) / 2.0;
        let f_mid = evaluate_polynomial(poly, mid)?;

        if f_mid.abs() < tol {
            return Some(BisectionResult { root: mid, iterations, error: f_mid.abs() });
        }

        if f_a * f_mid < 0.0 {
            b = mid;
        } else {
            a = mid;
        }

        iterations += 1;
    }

    Some(BisectionResult { root: (a + b) / 2.0, iterations, error: (b - a).abs() })
}
