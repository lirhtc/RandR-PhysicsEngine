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

    pub fn getX(&self) -> f64 {
        self.coordinates[0]
    }

    pub fn getY(&self) -> f64 {
        self.coordinates[1]
    }

    pub fn getVelocityX(&self) -> f64 {
        self.velocities[0]
    }

    pub fn getVelocityY(&self) -> f64 {
        self.velocities[1]
    }

    pub fn getMass(self) -> f64 {
        self.mass
    }

    pub fn setMass(&mut self, newMass: f64) {
        self.mass = newMass;
    }

    pub fn addVertex(&mut self, x: f64, y: f64) {
        self.vertices.push(vertex::Vertex {
            coordinates: [x, y],
        });
    }
}

impl ConvexPolygon {
    pub fn getAllVertices(&self) -> &Vec<vertex::Vertex> {
        return &self.vertices;
    }

    pub fn getVelocity(&self) -> [f64; 2] {
        self.velocities
    }
}
