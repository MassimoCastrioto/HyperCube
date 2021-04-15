/// Massimo Castrioto
/// HyperCube


mod vector {
   
   pub struct Data<T> {
      pub x: T,
      pub y: T,
      pub z: T
   }
   
}

pub type IVec3 = vector::Data<i32>;

mod tests;