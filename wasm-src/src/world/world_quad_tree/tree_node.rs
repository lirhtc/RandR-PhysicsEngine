use crate::shape::ConvexPolygon;
use std::ptr::null;
use wasm_bindgen::prelude::*;
use core::borrow::BorrowMut;


pub struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[wasm_bindgen]
struct TreeNode {
    level: i16,
    polygons: Vec<ConvexPolygon>,
    bounds: Rectangle,
    children: Option<[Box<TreeNode>; 4]>,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub fn get_width(&self) -> f64 {
        return self.width;
    }

    pub fn get_height(&self) -> f64 {
        return self.height;
    }

    pub fn get_x(&self) -> f64 {
        return self.x;
    }

    pub fn get_y(&self) -> f64 {
        return self.y;
    }
}

#[wasm_bindgen]
impl TreeNode {
    #[wasm_bindgen(constructor)]
    pub fn new(level: i16, bounds: Rectangle) -> TreeNode {
        TreeNode {
            level,
            polygons: Vec::new(),
            bounds,
            children: None,
        }
    }

    pub fn split(&mut self) {
        let sub_width = self.bounds.get_width() * 0.5;
        let sub_height = self.bounds.get_height() * 0.5;
        let x = self.bounds.get_x();
        let y = self.bounds.get_y();
        let level = self.level + 1;
        let node_1 = TreeNode::new(level, Rectangle::new(x, y, sub_width, sub_height));
        let node_2 = TreeNode::new(level, Rectangle::new(x + sub_width, y, sub_width, sub_height));
        let node_3 = TreeNode::new(level, Rectangle::new(x + sub_width, y + sub_height, sub_width, sub_height));
        let node_4 = TreeNode::new(level, Rectangle::new(x, y + sub_height, sub_width, sub_height));
        self.children = Option::Some([
            Box::new(node_1),
            Box::new(node_2),
            Box::new(node_3),
            Box::new(node_4)]);
    }

    pub fn get_quadrant_for_polygon(&self, polygon: &ConvexPolygon) -> i8 {
        /* check which quadrant a polygon belongs to.
        *  2  |   1
        * -----------
        *  3  |   4
        */
        let sub_width = self.bounds.get_width() * 0.5;
        let sub_height = self.bounds.get_height() * 0.5;
        let x = self.bounds.get_x();
        let y = self.bounds.get_y();
        let position = self.quadrant_position_check(x, y, sub_width, sub_height, polygon);
        match position {
            5 => return 1,
            9 => return 4,
            6 => return 2,
            10 => return 3,
            _ => return 0,
        }
    }

    pub fn insert(&mut self, polygon: &ConvexPolygon) {
        let check = self.children.is_some();
        let check_ga = self.children.is_none();
        if self.children.is_some() {
            let quadrant_index = (self.get_quadrant_for_polygon(polygon) - 1) as usize;
            let children: &mut [Box<TreeNode>; 4] = self.children.as_mut().unwrap().borrow_mut();
            let child: &mut TreeNode = children[quadrant_index].borrow_mut();
            child.insert(polygon);
        }
    }

    fn quadrant_position_check(&self, x: f64, y: f64, width: f64, height: f64, polygon: &ConvexPolygon) -> i8 {
//        The input x, y are the coordinate of parent's left top corner
//              width, height are 1/2 of parent block dimension
        let boundary = polygon.get_boundary();
        let min_x = boundary[0];
        let max_x = boundary[1];
        let min_y = boundary[2];
        let max_y = boundary[3];
        let mut position_check = 0;
        if min_x > (x + width) && max_x < (x + width + width) {
            // x in quadrant 1 or 4
            position_check |= 1;
        }
        if min_x > x && max_x < (x + width) {
            // x in quadrant 2 or 3
            position_check |= 1 << 1;
        }
        if min_y > y && max_y < (y + height) {
            // y in range 1 or 2
            position_check |= 1 << 2;
        }
        if min_y > (y + height) && max_y < (y + height + height) {
            // y in range 3 or 4
            position_check |= 1 << 3;
        }
        return position_check;
    }
}


// ********************************************************************
// ******************************* Test *******************************
// ********************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn polygon_on_quadrant_1() {
        let node = get_node();
        let polygon = get_polygon(6.0, 1.0, 2.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 1);
    }

    #[test]
    pub fn large_polygon_on_quadrant_1() {
        let node = get_node();
        let polygon = get_polygon(6.0, 1.0, 4.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 0);
    }

    #[test]
    pub fn polygon_on_quadrant_2() {
        let node = get_node();
        let polygon = get_polygon(1.0, 1.0, 2.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 2);
    }

    #[test]
    pub fn large_polygon_on_quadrant_2() {
        let node = get_node();
        let polygon = get_polygon(1.0, 1.0, 4.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 0);
    }

    #[test]
    pub fn polygon_on_quadrant_3() {
        let node = get_node();
        let polygon = get_polygon(1.0, 6.0, 2.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 3);
    }

    #[test]
    pub fn large_polygon_on_quadrant_3() {
        let node = get_node();
        let polygon = get_polygon(1.0, 6.0, 2.0, 4.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 0);
    }

    #[test]
    pub fn polygon_on_quadrant_4() {
        let node = get_node();
        let polygon = get_polygon(6.0, 6.0, 2.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 4);
    }

    #[test]
    pub fn large_polygon_on_quadrant_4() {
        let node = get_node();
        let polygon = get_polygon(6.0, 6.0, 2.0, 4.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 0);
    }

    fn get_node() -> TreeNode {
        TreeNode {
            level: 0,
            polygons: vec![],
            bounds: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
            },
            children: None,
        }
    }

    fn get_polygon(x: f64, y: f64, width: f64, height: f64) -> ConvexPolygon {
        let mut polygon = ConvexPolygon::new();
        polygon.add_vertex(x, y);
        polygon.add_vertex(x + width, y + height);
        polygon
    }
}