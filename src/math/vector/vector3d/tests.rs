/// Massimo Castrioto
/// HyperCube

#[ cfg( test ) ]
mod tests {
   
   use crate::math::IVec3;
   
   #[ test ]
   fn element_ctor() {
      let vec = IVec3{ x: 1, y: 2, z: 3 };
      
      assert_eq!( vec.x, 1 );
      assert_eq!( vec.y, 2 );
      assert_eq!( vec.z, 3 );
   }
   
}