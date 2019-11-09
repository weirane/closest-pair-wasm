use closest_pair::{closest_pair, Point};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ReturnType {
    pub dist: f64,
    pub p0_x: f64,
    pub p0_y: f64,
    pub p1_x: f64,
    pub p1_y: f64,
}

#[wasm_bindgen]
pub fn calculate(xs: &[f64], ys: &[f64]) -> ReturnType {
    let points: Vec<Point> = xs
        .iter()
        .zip(ys.iter())
        .map(|(&x, &y)| (x, y).into())
        .collect();
    let (dist, p0, p1) = closest_pair(&points);
    ReturnType {
        dist,
        p0_x: p0.x,
        p0_y: p0.y,
        p1_x: p1.x,
        p1_y: p1.y,
    }
}
