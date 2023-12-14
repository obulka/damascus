extern crate num;
#[macro_use]
extern crate num_derive;

pub mod geometry;
pub mod lights;
pub mod materials;
pub mod math;
pub mod renderers;
pub mod scene;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
