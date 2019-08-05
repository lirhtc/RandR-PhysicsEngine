use crate::shape::ConvexPolygon;
use wasm_bindgen::prelude::*;
use std::cmp;
use std::f64;

#[wasm_bindgen]
pub struct CollisionDetectorAabb {}

#[wasm_bindgen]
impl CollisionDetectorAabb {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CollisionDetectorAabb {
        CollisionDetectorAabb {}
    }

    pub fn collision_detect_polygon_polygon(first: &ConvexPolygon, second: &ConvexPolygon) -> bool {
        let first_boundary = CollisionDetectorAabb::get_aabb_boundaries(first);
        let second_boundary = CollisionDetectorAabb::get_aabb_boundaries(second);
        let horizontal_cross = ((first_boundary[0] - second_boundary[0]) * (first_boundary[0] - second_boundary[1]) <= 0.0)
            || ((first_boundary[1] - second_boundary[0]) * (first_boundary[1] - second_boundary[1]) <= 0.0);
        let vertical_cross = ((first_boundary[2] - second_boundary[2]) * (first_boundary[2] - second_boundary[3]) <= 0.0)
            || ((first_boundary[3] - second_boundary[2]) * (first_boundary[3] - second_boundary[3]) <= 0.0);
        return horizontal_cross && vertical_cross;
    }
}

impl CollisionDetectorAabb {
    // for a given convex polygon, return a list of:
    // min_x, min_y, max_x, max_y
    // in the order specified above
    fn get_aabb_boundaries(polygon: &ConvexPolygon) -> [f64; 4] {
        let mut boundary = [f64::MAX, f64::MIN, f64::MAX, f64::MIN];
        let vertex_iter = polygon.get_all_vertices().iter();
        for vertex in vertex_iter {
            boundary[0] = f64::min(vertex.coordinates[0], boundary[0]);
            boundary[1] = f64::max(vertex.coordinates[0], boundary[1]);
            boundary[2] = f64::min(vertex.coordinates[1], boundary[2]);
            boundary[3] = f64::max(vertex.coordinates[1], boundary[3]);
        }
        boundary[0] += polygon.get_x();
        boundary[1] += polygon.get_x();
        boundary[2] += polygon.get_y();
        boundary[3] += polygon.get_y();
        return boundary;
    }
}


// ********************************************************************
// ******************************* Test *******************************
// ********************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_polygon_collision() {
        // two overlap polygons
        let first_polygon_overlap = get_polygon(0.0, 0.0);
        let second_polygon_overlap = get_polygon(0.0, 0.0);
        assert!(CollisionDetectorAabb::collision_detect_polygon_polygon(&first_polygon_overlap, &second_polygon_overlap));

        // two cross polygons
        let first_polygon_cross = get_polygon(0.0, 0.0);
        let second_polygon_cross = get_polygon(0.5, 0.5);
        assert!(CollisionDetectorAabb::collision_detect_polygon_polygon(&first_polygon_cross, &second_polygon_cross));

        // two non-cross polygons
        let first_polygon_non_cross = get_polygon(0.0, 0.0);
        let second_polygon_non_cross = get_polygon(1.1, 0.0);
        assert!(!CollisionDetectorAabb::collision_detect_polygon_polygon(&first_polygon_non_cross, &second_polygon_non_cross));
    }

    fn get_polygon(offset_x: f64, offset_y: f64) -> ConvexPolygon {
        let mut polygon = ConvexPolygon::new();
        polygon.add_vertex(0.0 + offset_x, 0.0 + offset_y);
        polygon.add_vertex(1.0 + offset_x, 0.0 + offset_y);
        polygon.add_vertex(1.0 + offset_x, 1.0 + offset_y);
        polygon.add_vertex(0.0 + offset_x, 1.0 + offset_y);
        return polygon;
    }
}
