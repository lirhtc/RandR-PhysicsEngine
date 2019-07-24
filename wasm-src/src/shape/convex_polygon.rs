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
    vertices: Vec<vertex::Vertex>,
}

#[wasm_bindgen]
impl ConvexPolygon {
    fn new() -> ConvexPolygon {
        ConvexPolygon {
            coordinates: [0.0, 0.0],
            velocities: [0.0, 0.0],
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
}

impl ConvexPolygon {
    pub fn gett(&self) -> &vertex::Vertex {
        return &self.vertices[0];
    }
}
