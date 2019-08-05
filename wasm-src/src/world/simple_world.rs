use crate::shape::ConvexPolygon;
use wasm_bindgen::prelude::*;
use crate::collision::{CollisionDetectorAabb, CollisionResolver};
use wasm_bindgen::__rt::core::ptr::null;

struct WorldConfiguration {
    enable_gravity: bool,
    gravity_value: f64,
    unbound_world_size: bool,
    world_width: f64,
    world_height: f64,
    delta: f64,
}

#[wasm_bindgen]
pub struct SimpleWorld {
    id: i32,
    config: WorldConfiguration,
    polygons: Vec<ConvexPolygon>,
}

#[wasm_bindgen]
impl SimpleWorld {
    #[wasm_bindgen(constructor)]
    pub fn new(id: i32) -> SimpleWorld {
        SimpleWorld {
            id,
            config: WorldConfiguration {
                enable_gravity: true,
                gravity_value: 1.0,
                unbound_world_size: false,
                world_width: 800.0,
                world_height: 800.0,
                delta: 0.016,
            },
            polygons: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        let length = self.polygons.len();
        for i in 0..length {
            self.polygons[i].update(self.config.delta);
        }
        self.polygon_collision_resolve();
    }

    pub fn add_convex_polygon(&mut self, polygon: ConvexPolygon) -> usize {
        self.polygons.push(polygon);
        self.polygons.len()
    }

    pub fn set_update_delta(&mut self, delta: f64) {
        self.config.delta = delta;
    }

    pub fn get_polygon_x_at(&self, idx: usize) -> f64 {
        self.polygons[idx].get_x()
    }

    pub fn get_polygon_y_at(&self, idx: usize) -> f64 {
        self.polygons[idx].get_y()
    }
}


impl SimpleWorld {
    pub fn get_polygons(&mut self) -> &mut Vec<ConvexPolygon> {
        return &mut self.polygons;
    }

    pub fn polygon_collision_resolve(&mut self) {
        let mut length = self.polygons.len();
        if length <= 1 {
            return;
        }
        length = length - 1;
        for i in 0..length {
            let (first, second) = self.polygons.as_mut_slice().split_at_mut(i + 1);
            let collided = CollisionDetectorAabb::collision_detect_polygon_polygon(&mut first[i], &second[0]);
            if collided {
                CollisionResolver::collision_resolver_polygon_polygon(&mut first[i], &mut second[0], self.config.delta);
            }
        }
    }
}


// ********************************************************************
// ******************************* Test *******************************
// ********************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn world_updates_all_polygons_position_change() {
        let mut world = SimpleWorld::new(1);
        world.set_update_delta(1.0);

        let mut polygon_1 = ConvexPolygon::new();
        polygon_1.set_velocities([2.0, 3.0]);

        let mut polygon_2 = ConvexPolygon::new();
        polygon_2.set_velocities([5.0, 4.0]);

        world.add_convex_polygon(polygon_1);
        world.add_convex_polygon(polygon_2);
        world.update();

        assert_eq!(world.polygons.len(), 2);
        assert_eq!(world.polygons[0].get_x(), 2.0);
        assert_eq!(world.polygons[0].get_y(), 3.0);
        assert_eq!(world.polygons[1].get_x(), 5.0);
        assert_eq!(world.polygons[1].get_y(), 4.0);
    }

    #[test]
    pub fn world_updates_collision_resolve() {
        let mut world = SimpleWorld::new(1);
        world.set_update_delta(1.0);

        let mut polygon_1 = ConvexPolygon::new();
        polygon_1.set_velocities([1.0, 0.0]);
        polygon_1.add_vertex(0.0, 0.0);
        polygon_1.add_vertex(2.0, 0.0);
        polygon_1.add_vertex(2.0, 2.0);
        polygon_1.add_vertex(0.0, 2.0);

        let mut polygon_2 = ConvexPolygon::new();
        polygon_2.set_velocities([-1.0, 0.0]);
        polygon_2.add_vertex(0.0, 0.0);
        polygon_2.add_vertex(2.0, 0.0);
        polygon_2.add_vertex(2.0, 2.0);
        polygon_2.add_vertex(0.0, 2.0);
        polygon_2.set_x(2.0);

        world.add_convex_polygon(polygon_1);
        world.add_convex_polygon(polygon_2);
        world.update();
        assert_eq!(world.polygons.len(), 2);
        assert_eq!(world.polygons[0].get_x(), 0.0);
        assert_eq!(world.polygons[0].get_velocity_x(), -1.0);
        assert_eq!(world.polygons[0].get_y(), 0.0);
        assert_eq!(world.polygons[0].get_velocity_y(), 0.0);

        assert_eq!(world.polygons[1].get_x(), 2.0);
        assert_eq!(world.polygons[1].get_velocity_x(), 1.0);
        assert_eq!(world.polygons[1].get_y(), 0.0);
        assert_eq!(world.polygons[1].get_velocity_y(), 0.0);
    }
}