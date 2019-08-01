use wasm_bindgen::prelude::*;

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
    vertices: Vec<vertex::Vertex>,
}

#[wasm_bindgen]
impl ConvexPolygon {
    pub fn new() -> ConvexPolygon {
        ConvexPolygon {
            coordinates: [0.0, 0.0],
            velocities: [0.0, 0.0],
            mass: 1.0,
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

    pub fn set_mass(mut self, new_mass: f64) {
        self.mass = new_mass;
    }

    pub fn add_vertex(&mut self, x: f64, y: f64) {
        self.vertices.push(vertex::Vertex {
            coordinates: [x, y],
        });
    }
}

impl ConvexPolygon {
    pub fn get_all_vertices(&self) -> &Vec<vertex::Vertex> {
        return &self.vertices;
    }

    pub fn get_velocity(&self) -> [f64; 2] {
        self.velocities
    }

    pub fn setVelocities(&mut self, velocities: [f64; 2]) {
        self.velocities = velocities;
    }
}
