use wasm_bindgen::prelude::*;
use super::shape;
/*
 * Defination of the Shape type. It defined in a js file
 * 
 */
#[wasm_bindgen]
#[derive(Clone)]
pub struct World {
    shapes: Vec<shape::Circle::Circle>,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        World {
            shapes: Vec::<shape::Circle::Circle>::new(),
        }
    }

    pub fn add_one(&mut self) {
        self.shapes.push(shape::Circle::Circle {
            radius: 10.0,
            positionX: 1.0,
            positionY: 1.0,
        });
    }

    pub fn size(&mut self) -> usize {
        return self.shapes.len();
    }

    pub fn say(&self) -> f64 {
        return self.shapes[0].get_radius();
    }

    pub fn think(&mut self) {
        let r = self.shapes[0].get_radius();
        return self.shapes[0].set_radius(r + 10.0);
    }
}