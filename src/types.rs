use reflect::{Reflect};
use type_info::{TypeInfo};
use phf;

impl Reflect<'static> for i32 {
  fn type_info() -> &'static TypeInfo<i32> {
    static TYPE_INFO: TypeInfo<i32> = TypeInfo {
      name: "i32",
      attributes: &phf_map!()
    };
    &TYPE_INFO
  }
}

impl Reflect<'static> for &'static str {
  fn type_info() -> &'static TypeInfo<&'static str> {
    static TYPE_INFO: TypeInfo<&'static str> = TypeInfo {
      name: "&str",
      attributes: &phf_map!()
    };
    &TYPE_INFO
  }
}
