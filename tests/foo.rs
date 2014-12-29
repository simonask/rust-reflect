use reflect::{Reflect, Attribute, AnyAttribute, AttrResult, OwnerAttribute, TypeInfo};
use phf;

pub struct Foo {
  pub foo: i32
}

#[allow(non_snake_case)]
fn get_Foo_attribute_foo() -> &'static OwnerAttribute<Foo> {
  #[allow(non_camel_case_types)]
  struct Attr;
  impl Attribute<Foo, i32> for Attr {
    fn to_any_attr<'a>(&'a self) -> &'a AnyAttribute {
      self as &AnyAttribute
    }
    fn get_(&self, owner: &Foo) -> AttrResult<i32> {
      Ok(owner.foo)
    }
    fn set_(&self, owner: &mut Foo, new_value: i32) -> AttrResult<()> {
      owner.foo = new_value;
      Ok(())
    }
  }

  static FOO_ATTRIBUTE_FOO: Attr = Attr;
  &FOO_ATTRIBUTE_FOO as &OwnerAttribute<Foo>
}

impl Reflect<'static> for Foo {
  fn type_info() -> &'static TypeInfo<Foo> {
    static FOO_ATTRIBUTES: phf::Map<&'static str, fn() -> &'static OwnerAttribute<Foo>> = phf_map! {
      "foo" => get_Foo_attribute_foo as fn() -> &'static OwnerAttribute<Foo>
    };

    static FOO_TYPE_INFO: TypeInfo<Foo> = TypeInfo {
      name: "Foo",
      attributes: &FOO_ATTRIBUTES
    };

    &FOO_TYPE_INFO
  }
}
