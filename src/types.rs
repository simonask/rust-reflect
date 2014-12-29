use reflect::{ReflectStatic, StaticTypeInfo};
use type_info::{TypeInfo};
use phf;

impl ReflectStatic for i32 {
  fn static_type_info(_: Option<i32>) -> StaticTypeInfo<i32> {
    static TYPE_INFO: TypeInfo = TypeInfo {
      name: "i32",
      attributes: &phf_map!()
    };
    StaticTypeInfo(&TYPE_INFO)
  }
}

impl ReflectStatic for &'static str {
  fn static_type_info(_: Option<&'static str>) -> StaticTypeInfo<&'static str> {
    static TYPE_INFO: TypeInfo = TypeInfo {
      name: "&str",
      attributes: &phf_map!()
    };
    StaticTypeInfo(&TYPE_INFO)
  }
}
