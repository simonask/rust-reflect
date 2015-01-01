use reflect::{GetType, TypeInfo, Reflect, ReflectRefExt};
use std::default::Default;
use std::borrow::ToOwned;

#[reflect]
struct Foo {
  foo: i32,
  bar: String,
}

impl Default for Foo {
  fn default() -> Foo {
    Foo {
      foo: 123,
      bar: "Hello, World!".to_owned()
    }
  }
}

#[test]
fn get_name_of_type() {
  let t: &TypeInfo = GetType::of::<Foo>();
  assert!(t.name == "Foo");
}

#[test]
fn get_member_of_foo() {
  let foo: Foo = Default::default();
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
fn set_member_of_foo() {
  let mut foo: Foo = Default::default();
  let new_value: i32 = 456;
  let v = foo.set("foo", &new_value);
  match v {
    Ok(_) => assert!(foo.foo == new_value),
    Err(_) => assert!(false, "Could not set member!"),
  }
}

#[test]
fn set_nonexisting_member_should_fail() {
  let mut foo: Foo = Default::default();
  let new_value: i32 = 456;
  let v = foo.set("bar", &new_value);
  match v {
    Err(_) => (),
    Ok(_) => assert!(false, "It succeeded for some reason.")
  }
}

#[test]
fn set_member_of_wrong_type_should_fail() {
  let mut foo: Foo = Default::default();
  let new_value = "Hello, World!".to_owned();
  let v = foo.set("foo", &new_value);
  match v {
    Err(_) => (),
    Ok(_) => assert!(false, "It succeeded for some reason.")
  }
}
