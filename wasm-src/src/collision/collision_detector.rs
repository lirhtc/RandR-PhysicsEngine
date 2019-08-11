use crate::shape::ConvexPolygon;
use wasm_bindgen::prelude::*;
use std::cmp;
use std::f64;

#[wasm_bindgen]
pub struct CollisionDetectorAabb {}

#[wasm_bindgen]
pub struct CollisionDetectorSat {}

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
        let mut boundary = polygon.get_boundary();
        boundary[0] += polygon.get_x();
        boundary[1] += polygon.get_x();
        boundary[2] += polygon.get_y();
        boundary[3] += polygon.get_y();
        return boundary;
    }
}


#[wasm_bindgen]
impl CollisionDetectorSat {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CollisionDetectorSat {
        CollisionDetectorSat {}
    }

    pub fn collision_detect_polygon_polygon(first: &ConvexPolygon, second: &ConvexPolygon) -> bool {
        let polygon_side_vectors_first = CollisionDetectorSat::get_side_vectors_from_polygon(first);
        for one_side in polygon_side_vectors_first.iter() {
            let one_side_check = CollisionDetectorSat::detect_polygon_polygon_collision_by_one_side (first, second, *one_side);
            if one_side_check {
                return false;
            }
        }
        let polygon_side_vectors_second = CollisionDetectorSat::get_side_vectors_from_polygon(second);
        for one_side in polygon_side_vectors_second.iter() {
            let one_side_check = CollisionDetectorSat::detect_polygon_polygon_collision_by_one_side (first, second, *one_side);
            if one_side_check {
                return false;
            }
        }
        return true;
    }
}

impl CollisionDetectorSat {
    pub fn get_side_vectors_from_polygon(polygon: &ConvexPolygon) -> Vec<[f64; 2]> {
        let mut vectors = Vec::new();
        let vertices = polygon.get_all_vertices();
        if vertices.len() <= 1 {
            return vectors;
        }
        let length = vertices.len() - 1;
        for index in 0..length {
            let before = vertices.get(index);
            let after = vertices.get(index + 1);
            vectors.push([before.unwrap().coordinates[0] - after.unwrap().coordinates[0],
                before.unwrap().coordinates[1] - after.unwrap().coordinates[1]]);
        }
        let last = vertices.get(length).unwrap();
        let first = vertices.get(0).unwrap();
        vectors.push([last.coordinates[0] - first.coordinates[0],
            last.coordinates[1] - first.coordinates[1]]);
        return vectors;
    }

    pub fn detect_polygon_polygon_collision_by_one_side(first: &ConvexPolygon,
                                                        second: &ConvexPolygon,
                                                        side_vector: [f64; 2]) -> bool {
        // true if two polygons are seperate
        // false if it is not sure
        let orthogonal_vector = [ -side_vector[1], side_vector[0]];
        let first_vector_range = CollisionDetectorSat::get_vertex_min_max_for_vector(first, orthogonal_vector);
        let second_vector_range = CollisionDetectorSat::get_vertex_min_max_for_vector(second, orthogonal_vector);
        return first_vector_range[0]>second_vector_range[1] || first_vector_range[1]<second_vector_range[0];

    }

    pub fn get_vertex_min_max_for_vector(polygon: &ConvexPolygon, direction: [f64; 2]) -> [f64;2] {
        let mut min_position = f64::MAX;
        let mut max_position = f64::MIN;
        for vertex in polygon.get_all_vertices().iter() {
            let vector_position = (vertex.coordinates[0] + polygon.get_x()) * direction[0]
                + (vertex.coordinates[1] + polygon.get_y())* direction[1];
            min_position = f64::min(min_position, vector_position);
            max_position = f64::max(max_position, vector_position);
        }
        return [min_position, max_position];
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

    #[test]
    fn test_triangle_rectangle_not_collided(){
        let rect = get_rectangle(1.0, 0.0);
        let triangle = get_triangle(0.0,0.0);
        let separate_check = CollisionDetectorSat::collision_detect_polygon_polygon(&rect, &triangle);
        assert!(!separate_check);
    }


    #[test]
    fn test_triangle_rectangle_collided(){
        let rect = get_rectangle(0.0, 0.0);
        let triangle = get_triangle(0.0,0.0);
        let separate_check = CollisionDetectorSat::collision_detect_polygon_polygon(&rect, &triangle);
        assert!(separate_check);
    }

    fn get_rectangle(offset_x: f64, offset_y: f64) -> ConvexPolygon {
        let mut polygon = ConvexPolygon::new();
        polygon.add_vertex(0.0, 0.0);
        polygon.add_vertex(3.0, 0.0);
        polygon.add_vertex(3.0, 1.0);
        polygon.add_vertex(0.0, 1.0);
        polygon.set_x(offset_x);
        polygon.set_y(offset_y);
        return polygon;
    }

    fn get_triangle(offset_x: f64, offset_y: f64) -> ConvexPolygon{
        let mut polygon = ConvexPolygon::new();
        polygon.add_vertex(0.0, 0.0);
        polygon.add_vertex(2.0, 3.0);
        polygon.add_vertex(0.0, 5.0);
        polygon.set_x(offset_x);
        polygon.set_y(offset_y);
        return polygon;
    }
}
