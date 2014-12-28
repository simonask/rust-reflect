#![feature(phase)]

extern crate phf;
#[phase(plugin)]
extern crate phf_mac;

use std::any::{Any, AnyRefExt, AnyMutRefExt};

pub use attributes::{OwnerAttribute, AnyAttribute, Attribute, AttrResult};
pub use type_info::{TypeInfo, Type};
pub use reflect::{Reflect, Reflectable};

pub mod attributes;
pub mod type_info;
pub mod reflect;

struct Foo {
  foo: i32
}

#[allow(non_camel_case_types)]
struct Foo_Attributes_foo;
impl Attribute<Foo, i32> for Foo_Attributes_foo {
  fn get_(&self, owner: &Foo) -> AttrResult<i32> {
    Ok(owner.foo)
  }
  fn set_(&self, owner: &mut Foo, new_value: i32) -> AttrResult<()> {
    owner.foo = new_value;
    Ok(())
  }
}
static FOO_ATTRIBUTE_FOO: Foo_Attributes_foo = Foo_Attributes_foo;
#[allow(non_snake_case)]
fn get_Foo_attribute_foo() -> &'static OwnerAttribute<Foo> { &FOO_ATTRIBUTE_FOO as &OwnerAttribute<Foo> }

static FOO_ATTRIBUTES: phf::Map<&'static str, fn() -> &'static OwnerAttribute<Foo>> = phf_map! {
  "foo" => get_Foo_attribute_foo as fn() -> &'static OwnerAttribute<Foo>
};

impl Reflect for Foo {
  fn type_info() -> TypeInfo<Foo> {
    TypeInfo { name: "Foo", attributes: &FOO_ATTRIBUTES }
  }
}

#[test]
fn get_name_of_type() {
  let t: TypeInfo<Foo> = Reflect::type_info();
  assert!(t.name == "Foo");
}

#[test]
fn get_member() {
  let foo = Foo { foo: 123 };
  let v = foo.get("foo");
  match v {
    Ok(b) => match (*b).downcast_ref::<i32>() {
      Some(n) => assert!(*n == 123),
      None => assert!(false, "Member was not i32")
    },
    Err(_) => assert!(false, "Member did not exist")
  };
}

#[test]
fn set_member() {
  let mut foo = Foo { foo: 123 };
  let new_value: i32 = 456;
  let v = foo.set("foo", &new_value);
  match v {
    Ok(_) => assert!(foo.foo == new_value),
    Err(_) => assert!(false, "Could not set member!"),
  }
}

#[test]
fn set_nonexisting_member_should_fail() {
  let mut foo = Foo { foo: 123 };
  let new_value: i32 = 456;
  let v = foo.set("bar", &new_value);
  match v {
    Err(_) => (),
    Ok(_) => assert!(false, "It succeeded for some reason.")
  }
}

#[test]
fn set_member_of_wrong_type_should_fail() {
  let mut foo = Foo { foo: 123 };
  let new_value = "Hello, World!";
  let v = foo.set("foo", &new_value);
  match v {
    Err(_) => (),
    Ok(_) => assert!(false, "It succeeded for some reason.")
  }
}
