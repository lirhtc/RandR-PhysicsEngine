

use wasm_bindgen::prelude::*;
use std::ptr;

/*
 * Defination of the Shape type. It defined in a js file
 * 
 */
#[wasm_bindgen]
extern "C" {
    // Shape will be stored as a ref to a js object
    // The clone method thus copy the ref addres and should be fast
    #[derive(Clone)]
    pub type Shape;

    /*
     * getter to the paramters:
     * coorindate_x: f64
     * coorindate_y: f64
     * mass: f64
     * velocity_x: f64
     * velocity_y: f64
     */
    #[wasm_bindgen(method, getter, structural)]
    pub fn get_coordinate_x(this: &Shape) -> f64;

    #[wasm_bindgen(method, getter, structural)]
    pub fn get_coordinate_y(this: &Shape) -> f64;

    #[wasm_bindgen(method, getter, structural)]
    pub fn get_mass(this: &Shape) -> f64;

    #[wasm_bindgen(method, getter, structural)]
    pub fn get_velocity_x(this: &Shape) -> f64;

    #[wasm_bindgen(method, getter, structural)]
    pub fn get_velocity_y(this: &Shape) -> f64;

    /*
     * setter to the paramters:
     * coorindate_x: f64
     * coorindate_y: f64
     * mass: f64
     * velocity_x: f64
     * velocity_y: f64
     */
    #[wasm_bindgen(method, structural)]
    pub fn set_coordinate_x(this: &Shape, value: f64);

    #[wasm_bindgen(method, structural)]
    pub fn set_coordinate_y(this: &Shape, value: f64);

    #[wasm_bindgen(method, structural)]
    pub fn set_mass(this: &Shape, value: f64);

    #[wasm_bindgen(method, structural)]
    pub fn set_velocity_x(this: &Shape, value: f64);

    #[wasm_bindgen(method, structural)]
    pub fn set_velocity_y(this: &Shape, value: f64);
}

#[wasm_bindgen]
pub struct World {
    shapes: Vec<Shape>,
   
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        World {
            shapes: Vec::<Shape>::new(),
        
        }
    }

    pub fn add_one(&mut self, duck: Shape) {
        self.shapes.push(duck);
    }

    pub fn size(&mut self) -> usize {
        return self.shapes.len();
    }

    pub fn say(&self) -> f64 {
        return self.shapes[0].get_coordinate_x();
    }

    pub fn think(&self) {
        return self.shapes[0].set_coordinate_x(self.shapes[0].get_coordinate_x()+10.0);
    }
}

// The get all shapes is not usefull to JS world
impl World {
    pub fn get_all_shapes(& self) -> Vec<Shape> {
        return self.shapes.to_vec();
    }

    pub fn remove_all_shpaes(&mut self){
        self.shapes.clear();
    }
}


