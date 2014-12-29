use reflect::{ReflectStatic, Reflect, Attribute, AnyAttribute, AttrResult, OwnerAttribute, TypeInfo, StaticTypeInfo};
use phf;

pub struct Foo {
  pub foo: i32
}

impl ReflectStatic for Foo {
  fn static_type_info(_: Option<Foo>) -> StaticTypeInfo<Foo> {
    #[allow(non_snake_case)]
    fn get_Foo_attribute_foo() -> &'static AnyAttribute {
      #[allow(non_camel_case_types)]
      struct Attr;
      impl Attribute<Foo, i32> for Attr {
        fn get_(&self, owner: &Foo) -> AttrResult<i32> {
          Ok(owner.foo)
        }
        fn set_(&self, owner: &mut Foo, new_value: i32) -> AttrResult<()> {
          owner.foo = new_value;
          Ok(())
        }
      }

      static FOO_ATTRIBUTE_FOO: Attr = Attr;
      &FOO_ATTRIBUTE_FOO as &AnyAttribute
    }

    static FOO_ATTRIBUTES: phf::Map<&'static str, fn() -> &'static AnyAttribute> = phf_map! {
      "foo" => get_Foo_attribute_foo as fn() -> &'static AnyAttribute
    };

    static FOO_TYPE_INFO: TypeInfo = TypeInfo {
      name: "Foo",
      attributes: &FOO_ATTRIBUTES
    };

    StaticTypeInfo(&FOO_TYPE_INFO)
  }
}
