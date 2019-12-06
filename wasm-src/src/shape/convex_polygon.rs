use wasm_bindgen::prelude::*;
use std::f64;


mod vertex {
    pub struct Vertex {
        pub coordinates: [f64; 2],
    }
}

#[wasm_bindgen]
pub struct ConvexPolygon {
    coordinates: [f64; 2],
    velocities: [f64; 2],
    mass: f64,
//  The boundary of a polygon is ordered as:
//    min_x, max_x, min_y, max_y
    boundary: [f64;4 ],
    vertices: Vec<vertex::Vertex>,
}

#[wasm_bindgen]
impl ConvexPolygon {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ConvexPolygon {
        ConvexPolygon {
            coordinates: [0.0, 0.0],
            velocities: [0.0, 0.0],
            mass: 1.0,
            boundary:  [f64::MAX, f64::MIN, f64::MAX, f64::MIN],
            vertices: Vec::new(),
        }
    }

    pub fn get_x(&self) -> f64 {
        self.coordinates[0]
    }

    pub fn set_x(&mut self, value: f64) {
        self.coordinates[0] = value;
    }

    pub fn get_y(&self) -> f64 {
        self.coordinates[1]
    }

    pub fn set_y(&mut self, value: f64) {
        self.coordinates[1] = value;
    }

    pub fn get_velocity_x(&self) -> f64 {
        self.velocities[0]
    }

    pub fn get_velocity_y(&self) -> f64 {
        self.velocities[1]
    }

    pub fn set_velocity_x(&mut self, new_v: f64) {
        self.velocities[0] = new_v;
    }

    pub fn set_velocity_y(&mut self, new_v: f64) {
        self.velocities[1] = new_v;
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn set_mass(&mut self, new_mass: f64) {
        self.mass = new_mass;
    }

    pub fn add_vertex(&mut self, x: f64, y: f64) {
        self.vertices.push(vertex::Vertex {
            coordinates: [x, y],
        });
        self.boundary[0] = f64::min(x, self.boundary[0]);
        self.boundary[1] = f64::max(x, self.boundary[1]);
        self.boundary[2] = f64::min(y, self.boundary[2]);
        self.boundary[3] = f64::max(y, self.boundary[3]);
    }
}

impl ConvexPolygon {
    pub fn get_all_vertices(&self) -> &Vec<vertex::Vertex> {
        return &self.vertices;
    }

    pub fn get_velocity(&self) -> [f64; 2] {
        self.velocities
    }

    pub fn set_velocities(&mut self, velocities: [f64; 2]) {
        self.velocities = velocities;
    }

    pub fn update(&mut self, delta: f64) {
        self.set_x(self.get_x() + self.get_velocity_x() * delta);
        self.set_y(self.get_y() + self.get_velocity_y() * delta);
    }

    pub fn get_boundary(&self)-> [f64; 4]{
        return self.boundary;
    }

    pub fn reverse_velocity_x(&mut self){
        self.velocities[0] = -self.velocities[0];
    }

    pub fn reverse_velocity_y(&mut self){
        self.velocities[1] = -self.velocities[1];
    }
}


// ********************************************************************
// ******************************* Test *******************************
// ********************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_polygon_update(){
        let mut polygon = ConvexPolygon{
            coordinates: [0.0, 2.0],
            velocities: [1.0, 2.0],
            mass: 11.0,
            vertices: vec![],
            boundary:[f64::MAX, f64::MIN, f64::MAX, f64::MIN],
        };
        polygon.update(1.0);
        assert_eq!(polygon.get_x(), 1.0);
        assert_eq!(polygon.get_y(), 4.0);
    }

}
