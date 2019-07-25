use super::super::shape::ConvexPolygon;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct collisionDetectorAabb {}
#[wasm_bindgen]
impl collisionDetectorAabb {
    fn new() -> collisionDetectorAabb {
        collisionDetectorAabb {}
    }

    fn collisionDetectPolygonPolygon(first: ConvexPolygon, second: ConvexPolygon) -> bool {}
}
