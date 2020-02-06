mod util;

use closest_pair::{closest_pair, Point};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct WasmApp {
    points: Vec<Point>,
}

#[wasm_bindgen(module = "/www/present.js")]
extern "C" {
    #[wasm_bindgen(js_name = presentResult)]
    fn present_result(dist: f64, p0_x: f64, p0_y: f64, p1_x: f64, p1_y: f64);
}

#[wasm_bindgen]
impl WasmApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    #[wasm_bindgen(js_name = addPoint)]
    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push((x, y).into());
    }

    #[wasm_bindgen(js_name = hasPoint)]
    pub fn has_point(&self) -> bool {
        !self.points.is_empty()
    }

    pub fn calculate(&mut self) -> Result<(), JsValue> {
        match self.points.len() {
            0 => Err(JsValue::from_str("No point given")),
            1 => Err(JsValue::from_str("Only one point")),
            _ => {
                let (dist, p0, p1) = closest_pair(&self.points);
                present_result(dist, p0.x, p0.y, p1.x, p1.y);
                Ok(())
            }
        }
    }

    pub fn clear(&mut self) {
        self.points.clear();
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    util::set_panic_hook();
}
