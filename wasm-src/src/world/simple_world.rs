use crate::shape::ConvexPolygon;
use wasm_bindgen::prelude::*;
use crate::collision::{CollisionDetectorAabb, CollisionResolver};

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

    pub fn update(&mut self, delta: f64) {
        let length = self.polygons.len();
        for i in 0..length {
            self.polygons[i].update(delta);
        }
        self.polygon_collision_resolv();
    }
}


impl SimpleWorld {
    pub fn get_polygons(&mut self) -> &mut Vec<ConvexPolygon> {
        return &mut self.polygons;
    }

    pub fn polygon_collision_resolv(&mut self) {
        let length = self.polygons.len();
        for i in 0..length - 1 {
            let (first,second)  = self.polygons.as_mut_slice().split_at_mut(i + 1);
            let collided = CollisionDetectorAabb::collision_detect_polygon_polygon(&mut first[i], &second[0]);
            if collided {
                CollisionResolver::collision_resolver_polygon_polygon(&mut first[i], &mut second[0], self.config.delta);
            }
        }
    }
}