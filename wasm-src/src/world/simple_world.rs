// use super::shape::ConvexPolygon;
use super::super::shape::ConvexPolygon;
use wasm_bindgen::prelude::*;

struct WorldConfiguration {
    enable_gravity: bool,
    gravity_value: f64,
    unbound_world_size: bool,
    world_width: f64,
    world_height: f64,
}

#[wasm_bindgen]
pub struct SimpleWorld {
    id: i64,
    config: WorldConfiguration,
    polygons: Vec<ConvexPolygon>,
}

impl SimpleWorld {
    fn new(id: i64) -> SimpleWorld {
        SimpleWorld {
            id: id,
            config: WorldConfiguration {
                enable_gravity: true,
                gravity_value: 1.0,
                unbound_world_size: false,
                world_width: 800.0,
                world_height: 800.0,
            },
            polygons: Vec::new(),
        }
    }
}
