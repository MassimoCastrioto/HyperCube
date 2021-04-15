/// Massimo Castrioto
/// HyperCube

mod math;

fn main() {
   let vec = math::IVec3{ x: 1, y: 2, z: 3 };
   
   
   println! (
      "vector: {}, {}, {}",
      vec.x,
      vec.y,
      vec.z
   );
}
