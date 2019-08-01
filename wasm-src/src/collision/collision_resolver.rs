use wasm_bindgen::prelude::*;
use crate::shape::ConvexPolygon;

#[wasm_bindgen]
pub struct CollisionResolver {}

#[wasm_bindgen]
impl CollisionResolver {
    pub fn collision_resolver_polygon_polygon(first: &mut ConvexPolygon, second: &mut ConvexPolygon, delta: f64) {
        let first_ux = first.get_velocity()[0];
        let first_uy = first.get_velocity()[1];
        let second_ux = second.get_velocity()[0];
        let second_uy = second.get_velocity()[1];

        let after_v_horizontal = CollisionResolver::resolve_one_dimension(first.get_mass(), second.get_mass(), first_ux, second_ux);
        let after_v_vertical = CollisionResolver::resolve_one_dimension(first.get_mass(), second.get_mass(), first_uy, second_uy);

        reverse_update(first, delta);
        reverse_update(second, delta);

        first.set_velocity_x(after_v_horizontal[0]);
        second.set_velocity_x(after_v_horizontal[1]);
        first.set_velocity_y(after_v_vertical[0]);
        second.set_velocity_y(after_v_vertical[1]);
    }
}

impl CollisionResolver {
    fn resolve_one_dimension(m1: f64, m2: f64, u1: f64, u2: f64) -> [f64; 2] {
        let v1 = (m1 - m2) / (m1 + m2) * u1 + 2.0 * m2 / (m1 + m2) * u2;
        let v2 = 2.0 * m1 / (m1 + m2) * u1 + (m2 - m1) / (m1 + m2) * u2;
        return [v1, v2];
    }

    fn reverse_update(polygon: &mut ConvexPolygon, delta: f64){
        let velocities = polygon.getVelocity();
        polygon.set_x(polygon.get_x() - velocities[0] * delta);
        polygon.set_y(polygon.get_y() - velocities[1] * delta);
    }
}




// ********************************************************************
// ******************************* Test *******************************
// ********************************************************************

#[cfg(test)]
mod tests {
    use super::*;


    fn get_polygon(mass: f64, velocity: [f64; 2], coordinates: [f64;2 ]) -> ConvexPolygon {

    }
}