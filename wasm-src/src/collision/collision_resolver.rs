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

        CollisionResolver::reverse_update(first, delta);
        CollisionResolver::reverse_update(second, delta);

        first.set_velocity_x(after_v_horizontal[0]);
        second.set_velocity_x(after_v_horizontal[1]);
        first.set_velocity_y(after_v_vertical[0]);
        second.set_velocity_y(after_v_vertical[1]);

//        first.update(delta);
//        second.update(delta);
    }
}

impl CollisionResolver {
    fn resolve_one_dimension(m1: f64, m2: f64, u1: f64, u2: f64) -> [f64; 2] {
        let v1 = (m1 - m2) / (m1 + m2) * u1 + 2.0 * m2 / (m1 + m2) * u2;
        let v2 = 2.0 * m1 / (m1 + m2) * u1 + (m2 - m1) / (m1 + m2) * u2;
        return [v1, v2];
    }

    fn reverse_update(polygon: &mut ConvexPolygon, delta: f64) {
        let velocities = polygon.get_velocity();
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

    #[test]
    fn two_polypon_collision_test_same_mass_move_static() {
        // two polygons, same mass, one has initi velocity and one static
        let mut polygon_1 = get_polygon(10.0, [10.0, 0.0]);
        let mut polygon_2 = get_polygon(10.0, [0.0, 0.0]);
        CollisionResolver::collision_resolver_polygon_polygon(&mut polygon_1, &mut polygon_2, 0.0);
        assert_eq!(polygon_1.get_velocity_x(), 0.0);
        assert_eq!(polygon_2.get_velocity_x(), 10.0);
    }

    #[test]
    fn two_polypon_collision_test_same_mass_fast_opposite() {
        // same mass. One is 10 and other is -5
        let mut polygon_1 = get_polygon(10.0, [10.0, 0.0]);
        let mut polygon_2 = get_polygon(10.0, [-5.0, 0.0]);
        CollisionResolver::collision_resolver_polygon_polygon(&mut polygon_1, &mut polygon_2, 0.0);
        assert_eq!(polygon_1.get_velocity_x(), -5.0);
        assert_eq!(polygon_2.get_velocity_x(), 10.0);
    }

    #[test]
    fn two_polypon_collision_test_same_mass_fast_slow() {
        // same mass. One is 10 and other is -5
        let mut polygon_1 = get_polygon(10.0, [10.0, 0.0]);
        let mut polygon_2 = get_polygon(10.0, [5.0, 0.0]);
        CollisionResolver::collision_resolver_polygon_polygon(&mut polygon_1, &mut polygon_2, 0.0);
        assert_eq!(polygon_1.get_velocity_x(), 5.0);
        assert_eq!(polygon_2.get_velocity_x(), 10.0);
    }

    #[test]
    fn two_polypon_collision_test_double_mass_opposite() {
        // same mass. One is 10 and other is -5
        let mut polygon_1 = get_polygon(10.0, [10.0, 0.0]);
        let mut polygon_2 = get_polygon(5.0, [-10.0, 0.0]);
        CollisionResolver::collision_resolver_polygon_polygon(&mut polygon_1, &mut polygon_2, 0.0);
        let delta_polygon_1_ = f64::abs(polygon_1.get_velocity_x() - (-10.0 / 3.0));
        let delta_polygon_2_ = f64::abs(polygon_2.get_velocity_x() - (50.0 / 3.0));
        assert!( delta_polygon_1_< 0.00001);
        assert!( delta_polygon_2_< 0.00001);
    }

    fn get_polygon(mass: f64, velocity: [f64; 2]) -> ConvexPolygon {
        let mut polygon = ConvexPolygon::new();
        let mut polygon = ConvexPolygon::new();
        let mut polygon = ConvexPolygon::new();
        polygon.set_mass(mass);
        polygon.set_velocities(velocity);
        return polygon;
    }
}