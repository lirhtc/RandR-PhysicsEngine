use wasm_bindgen::prelude::*;
use crate::shape::ConvexPolygon;

#[wasm_bindgen]
pub struct CollisionResolver {}

#[wasm_bindgen]
impl CollisionResolver {
    pub fn collision_resolver_polygon_polygon(first: ConvexPolygon, second: ConvexPolygon) -> bool {
        let u1 = first.getVelocity()[0];
        let u2
    }
}

impl CollisionResolver {
    fn resolve_one_deminssion (m1: f64, m2: f64, u1:f64, u2:f64) -> [2; 64] {
    return [0.0,0.0];
    }
}