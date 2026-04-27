use wasm_bindgen::prelude::*;

fn add(a: f64, b: f64) -> f64 { a + b }
fn sub(a: f64, b: f64) -> f64 { a - b }
fn mul(a: f64, b: f64) -> f64 { a * b }
fn div(a: f64, b: f64) -> f64 { a / b }
fn pow(a: f64, b: f64) -> f64 { a.powf(b) }
fn modulo(a: f64, b: f64) -> f64 { a % b }

#[wasm_bindgen]
pub fn calcular(op: &str, a: f64, b: f64) -> f64 {
    match op {
        "add" => add(a, b),
        "sub" => sub(a, b),
        "mul" => mul(a, b),
        "div" => 
        if b != 0.0 {
            div(a, b)
        } else {
            0.0
        },
        "pow" => pow(a, b),
        "modulo" => modulo(a, b),
        _ => 0.0,
    }
}