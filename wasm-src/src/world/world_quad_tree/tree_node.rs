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
    capacity: usize,
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
    pub fn new(level: i16, bounds: Rectangle, capacity: usize) -> TreeNode {
        TreeNode {
            level,
            polygons: Vec::new(),
            bounds,
            children: None,
            capacity,
        }
    }

    pub fn split(&mut self) {
        let sub_width = self.bounds.get_width() * 0.5;
        let sub_height = self.bounds.get_height() * 0.5;
        let x = self.bounds.get_x();
        let y = self.bounds.get_y();
        let level = self.level + 1;
        let node_1 = TreeNode::new(level, Rectangle::new(x, y, sub_width, sub_height), self.capacity);
        let node_2 = TreeNode::new(level, Rectangle::new(x + sub_width, y, sub_width, sub_height), self.capacity);
        let node_3 = TreeNode::new(level, Rectangle::new(x + sub_width, y + sub_height, sub_width, sub_height), self.capacity);
        let node_4 = TreeNode::new(level, Rectangle::new(x, y + sub_height, sub_width, sub_height), self.capacity);
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

    pub fn insert(&mut self, polygon: ConvexPolygon) {
        println!("Prepare to insert");
        if self.children.is_some() {
            let child_index = self.get_quadrant_for_polygon(&polygon);
            if child_index != 0 {
                let quadrant_index = (child_index - 1) as usize;
                let children: &mut [Box<TreeNode>; 4] = self.children.as_mut().unwrap().borrow_mut();
                let child: &mut TreeNode = children[quadrant_index].borrow_mut();
                child.insert(polygon);
                return;
            }
        }
        println!("Prepare to insert into polygons, current size {}", self.polygons.len());
        self.polygons.push(polygon);
        println!("After insertion size {}", self.polygons.len());
        if self.polygons.len() > self.capacity && self.children.is_none(){
            self.split();
            let current_polygons = self.polygons.to_owned().clone();
            self.polygons =  Vec::new();
            for polygon in current_polygons {
//                let new_clone = polygon.clone();
                self.insert(polygon);
            }
        }
    }

    pub fn update(&mut self, delta:f64) {
        if self.children.is_some() {
            let  children: &mut[Box<TreeNode>; 4] =  self.children.as_mut().unwrap().borrow_mut();
            for mut child in children{
                child.update(delta);
            }
        } else {
            let polygons: &mut Vec<ConvexPolygon> = self.polygons.borrow_mut();
            for mut polygon in polygons {
                polygon.update(delta);
            }
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
    use core::borrow::Borrow;

    #[test]
    pub fn polygon_on_quadrant_1() {
        let node = get_node(4);
        let polygon = get_polygon(6.0, 1.0, 2.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 1);
    }

    #[test]
    pub fn large_polygon_on_quadrant_1() {
        let node = get_node(4);
        let polygon = get_polygon(6.0, 1.0, 4.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 0);
    }

    #[test]
    pub fn polygon_on_quadrant_2() {
        let node = get_node(4);
        let polygon = get_polygon(1.0, 1.0, 2.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 2);
    }

    #[test]
    pub fn large_polygon_on_quadrant_2() {
        let node = get_node(4);
        let polygon = get_polygon(1.0, 1.0, 4.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 0);
    }

    #[test]
    pub fn polygon_on_quadrant_3() {
        let node = get_node(4);
        let polygon = get_polygon(1.0, 6.0, 2.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 3);
    }

    #[test]
    pub fn large_polygon_on_quadrant_3() {
        let node = get_node(4);
        let polygon = get_polygon(1.0, 6.0, 2.0, 4.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 0);
    }

    #[test]
    pub fn polygon_on_quadrant_4() {
        let node = get_node(4);
        let polygon = get_polygon(6.0, 6.0, 2.0, 2.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 4);
    }

    #[test]
    pub fn large_polygon_on_quadrant_4() {
        let node = get_node(4);
        let polygon = get_polygon(6.0, 6.0, 2.0, 4.0);
        let index = node.get_quadrant_for_polygon(&polygon);
        assert_eq!(index, 0);
    }

    #[test]
    pub fn insert_polygon_belongs_to_parent() {
        let mut root_node = get_node(4);
        let polygon = get_polygon(6.0, 6.0, 2.0, 4.0);
        root_node.insert(polygon);
        assert_eq!(root_node.polygons.len(), 1);
        assert!(root_node.children.is_none());
    }

    #[test]
    pub fn insert_polygons_parent_should_split_1() {
        let mut root_node = get_node(4);

        let polygon = get_polygon(1.0, 1.0, 1.0, 1.0);
        root_node.insert(polygon);

        let polygon = get_polygon(6.0, 1.0, 1.0, 1.0);
        root_node.insert(polygon);

        let polygon = get_polygon(6.0, 6.0, 1.0, 1.0);
        root_node.insert(polygon);

        let polygon = get_polygon(1.0, 6.0, 1.0, 1.0);
        root_node.insert(polygon);

        let polygon = get_polygon(6.0, 1.0, 1.0, 1.0);
        root_node.insert(polygon);

//        Small polygons can fit into sub nodes. There should be no polygons left after parent split.
        assert_eq!(root_node.polygons.len(), 0);
//        Parent should split
        assert!(root_node.children.is_some());
//        Child node should not split as the polygon counts in each node is smaller than capacity (4)
        for i in 0..4 {
            assert!(  root_node.children.as_ref().unwrap()[i].children.borrow().is_none());
        }
//        Child node should have 1 polygon in each quadrant except quadrant 1.
        for i in 1..4 {
            assert_eq!(root_node.children.as_ref().unwrap()[i].polygons.len(), 1 );
        }
        assert_eq!(root_node.children.as_ref().unwrap()[0].polygons.len(), 2 );
    }

    #[test]
    pub fn insert_polygons_parent_should_split_2() {
        let mut root_node = get_node(4);
        for i in 0..5{
            println!("{}", i);
            let polygon = get_polygon(1.0, 1.0, 1.0, 1.0);
            root_node.insert(polygon);
        }
//        Small polygons can fit into sub nodes. There should be no polygons left after parent split.
        assert_eq!(root_node.polygons.len(), 0);
//        Parent should split
        assert!(root_node.children.is_some());
//        Child node on quadrant 2 should split again
        assert!( root_node.children.as_ref().unwrap()[1].children.is_some());
//        Child node on quadrant 2 should have all 5 polygons after split
        assert_eq!(root_node.borrow().children.as_ref().unwrap()[1].polygons.len(), 5);
    }

    fn get_node(capacity: usize) -> TreeNode {
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
            capacity: 4,
        }
    }

    fn get_polygon(x: f64, y: f64, width: f64, height: f64) -> ConvexPolygon {
        let mut polygon = ConvexPolygon::new();
        polygon.add_vertex(x, y);
        polygon.add_vertex(x + width, y + height);
        polygon
    }
}