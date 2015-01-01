use reflect::{StaticReflection};
use type_info::{TypeInfo, TypeInfoFor};
use phf;

macro_rules! reflection_for {
  ($ty:ty, $name:expr) => {
    impl StaticReflection for $ty {
      fn type_info_for(_: Option<$ty>) -> TypeInfoFor<$ty> {
        static TYPE_INFO: TypeInfo = TypeInfo {
          name: $name,
          attributes: &phf_map!()
        };
        TypeInfoFor(&TYPE_INFO)
      }
    }
  }
}

reflection_for!(i8, "i8");
reflection_for!(i16, "i16");
reflection_for!(i32, "i32");
reflection_for!(i64, "i64");
reflection_for!(u8, "u8");
reflection_for!(u16, "u16");
reflection_for!(u32, "u32");
reflection_for!(u64, "u64");

reflection_for!(char, "char");
reflection_for!(String, "String");

reflection_for!(f32, "f32");
reflection_for!(f64, "f64");
