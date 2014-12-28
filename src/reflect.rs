use type_info::{TypeInfo};
use attributes::{AttrResult, AttrError};
use std::any::{Any};

pub trait Reflect {
  fn type_info() -> TypeInfo<Self>;
}

pub trait Reflectable {
  fn get(&self, name: &str) -> AttrResult<Box<Any>>;
  fn set(&mut self, name: &str, new_value: &Any) -> AttrResult<()>;
}

impl<T> Reflectable for T
  where T: Reflect
{
  fn get(&self, name: &str) -> AttrResult<Box<Any>> {
    let t: TypeInfo<T> = Reflect::type_info();
    match t.attributes.get(name) {
      Some(attrfn) => (*attrfn)().get(self),
      None => Err(AttrError::UnknownAttribute)
    }
  }

  fn set(&mut self, name: &str, new_value: &Any) -> AttrResult<()> {
    let t: TypeInfo<T> = Reflect::type_info();
    match t.attributes.get(name) {
      Some(attrfn) => (*attrfn)().set(self, new_value),
      None => Err(AttrError::UnknownAttribute)
    }
  }
}
