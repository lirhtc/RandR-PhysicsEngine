use wasm_bindgen::prelude::*;
use crate::shape::ConvexPolygon;

#[wasm_bindgen]
pub struct CollisionResolver {}

#[wasm_bindgen]
impl CollisionResolver {
    pub fn collision_resolver_polygon_polygon(first: ConvexPolygon, second: ConvexPolygon, delta: f64) {
        let first_ux = first.getVelocity()[0];
        let first_uy = first.getVelocity()[1];
        let second_ux = second.getVelocity()[0];
        let second_uy = second.getVelocity()[1];

        let after_v_horizontal = CollisionResolver::resolve_one_deminssion(first.getMass(), second.getMass(), first_ux, second_ux);
        let after_v_vertical = CollisionResolver::resolve_one_deminssion(first.getMass(), second.getMass(), first_uy, second_uy);


    }
}

impl CollisionResolver {
    fn resolve_one_deminssion(m1: f64, m2: f64, u1: f64, u2: f64) -> [f64; 2] {
        let v1 = (m1 - m2) / (m1 + m2) * u1 + 2.0 * m2 / (m1 + m2) * u2;
        let v2 = 2.0 * m1 / (m1 + m2) * u1 + (m2 - m1) / (m1 + m2) * u2;
        return [v1, v2];
    }

    fn reverse_update(polygon: ConvexPolygon, delta: f64){
        let velocities = polygon.getVelocity();
    }
}
