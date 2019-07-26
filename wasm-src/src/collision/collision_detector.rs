use super::super::shape::ConvexPolygon;
use wasm_bindgen::prelude::*;
use std::cmp;

#[wasm_bindgen]
pub struct CollisionDetectorAabb {}

#[wasm_bindgen]
impl CollisionDetectorAabb {
    fn new() -> CollisionDetectorAabb {
        CollisionDetectorAabb {}
    }

    fn collision_detect_polygon_polygon(self, first: ConvexPolygon, second: ConvexPolygon) -> bool {
        let first_boundary = self.get_aabb_boundaries(first);
    }
}

impl CollisionDetectorAabb {
    // for a given convex polygon, return a list of:
    // min_x, min_y, max_x, max_y
    // in the order specified above
    fn get_aabb_boundaries(&self, polygon: ConvexPolygon)-> [f64; 4]{
        let mut boundary = [0.0, 0.0, 0.0, 0.0];
        let vertex_iter = polygon.getAllVertices().iter();
        for vertex in vertex_iter {
            boundary[0] = f64::min(vertex.coordinates[0], boundary[0]);
            boundary[1] = f64::max(vertex.coordinates[0], boundary[1]);
            boundary[2] = f64::min(vertex.coordinates[1], boundary[2]);
            boundary[3] = f64::max(vertex.coordinates[1], boundary[3]);
        }
        return boundary;
    }
}