use wasm_bindgen::prelude::*;
// mod World;
// /// Here is a duck-typed interface for any JavaScript object that has a `quack`
// /// method.
// ///
// /// Note that any attempts to check if an object is a `Quacks` with
// /// `JsCast::is_instance_of` (i.e. the `instanceof` operator) will fail because
// /// there is no JS class named `Quacks`

// static CIRCLE: i16 = 1;
// static G: f64 = 10.0;

// #[wasm_bindgen]
// pub struct Engine {
//     world: World::World,
// }

// #[wasm_bindgen]
// impl Engine {
//     #[wasm_bindgen(constructor)]
//     pub fn new(world: World::World) -> Engine {
//         Engine { world: world }
//     }
//     /*
//      * tick function will update all objects within the world by the time delta
//      * The parameter tick is defined in ms and it accept integer
//      * The momentum change is given by:
//      * P = Ft = GmM/(r^2) * delta / 1000 = Vm
//      */
//     pub fn tick(&mut self, delta: i16) {
//         let shapes = self.world.get_all_shapes();
//         let size = self.world.size();
//         let mut index_i = 0;
//         while index_i < size{
//         // for target_shape in shapes {
//             let mut target_shape = &shapes[index_i];
//             let mut delta_x_velocity = 0.0;
//             let mut delta_y_velocity = 0.0;
//             let mut index_j = 0;
//              while index_j < size{
//                 if (index_i == index_j){
//                     index_j += 1;
//                     continue;
//                 }
//                 let mut rest_shape = &shapes[index_j];
//                 let (delta_x, delta_y) =
//                     Engine::process_object_pair(delta, &target_shape, &rest_shape);
//                 delta_x_velocity += delta_x;
//                 delta_y_velocity += delta_y;
//                 index_j += 1;
//             }
//             // update the velocity in both directions (x and y)
//             target_shape.set_velocity_x(target_shape.get_velocity_x() + delta_x_velocity);
//             target_shape.set_velocity_y(target_shape.get_velocity_y() + delta_y_velocity);
//             // update the position in both directions (x and y)
//             target_shape.set_coordinate_x(
//                 target_shape.get_velocity_x() * f64::from(delta) * 0.001
//                     + target_shape.get_coordinate_x(),
//             );
//             target_shape.set_coordinate_y(
//                 target_shape.get_velocity_y() * f64::from(delta) * 0.001
//                     + target_shape.get_coordinate_y(),
//             );
//             index_i+=1;
//         }
//     }

//     fn process_object_pair(
//         delta: i16,
//         target: &World::Shape,
//         other: &World::Shape,
//     ) -> (f64, f64) {
//         if target as *const _ == other as *const _ {
//             return (0.0, 0.0);
//         // } else {
//         //     let delta_position_x = other.get_coordinate_x() - target.get_coordinate_x();
//         //     if (delta_position_x < 0.01){
//         //             let delta_velocity_x =0;
//         //     }else {
//         //     let delta_velocity_x =
//         //         G * other.get_mass() / (delta_position_x.powi(2)) * f64::from(delta) * 0.001;
//         //      }
//         //     let delta_position_y = target.get_coordinate_y() - other.get_coordinate_y();

//         //      if (delta_position_y < 0.01){
//         //             let delta_velocity_y =0;
//         //      }else {
//         //     let delta_velocity_y =
//         //         G * other.get_mass() / (delta_position_y.powi(2)) * f64::from(delta) * 0.001;
//         //      }
//         //     return (delta_velocity_x, delta_velocity_y);
//         // }
//         }
//         let posi_vec_x = other.get_coordinate_x() - target.get_coordinate_x();
//         let posi_vec_y = other.get_coordinate_y() - target.get_coordinate_y();
//         let base =  (posi_vec_x.powi(2) + posi_vec_y.powi(2)).sqrt();
//         if (base<1.1){
//             return (0.0, 0.0,);
//         }
//         let delta_v = G * other.get_mass() / (posi_vec_x.powi(2) + posi_vec_y.powi(2)) * f64::from(delta) * 0.001;
//         let delta_v_x = delta_v * posi_vec_x / base;
//         let delta_v_y = delta_v * posi_vec_y / base;
//         return (delta_v_x, delta_v_y);
//     }

//     pub fn add_one(&mut self, duck: World::Shape) {
//         self.world.add_one(duck);
//     }

//     pub fn clear_world(&mut self){
//         self.world.remove_all_shpaes();
//     }
// }
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }

//      #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }

// //https://stackoverflow.com/questions/24110970/tuple-struct-constructor-complains-about-private-fields

mod shape;
mod world;
mod collision;