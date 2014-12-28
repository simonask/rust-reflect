use type_info::{Type, TypeInfo};
use attributes::{AttrResult, AttrError};
use std::any::{Any};

pub trait Reflect<'a> {
  fn type_info() -> &'a TypeInfo<Self>;
}

pub trait Reflectable: Any {
  fn type_info(&self) -> &'static Type<'static>;
  fn get(&self, name: &str) -> AttrResult<Box<Any>>;
  fn set(&mut self, name: &str, new_value: &Any) -> AttrResult<()>;
}

impl<'a, T> Reflectable for T
  where T: Reflect<'static> + 'static
{
  fn type_info(&self) -> &'static Type<'static> {
    let t: &TypeInfo<T> = Reflect::type_info();
    t as &'static Type<'static>
  }

  fn get(&self, name: &str) -> AttrResult<Box<Any>> {
    let t: &TypeInfo<T> = Reflect::type_info();
    match t.attributes.get(name) {
      Some(attrfn) => (*attrfn)().get(self),
      None => Err(AttrError::UnknownAttribute)
    }
  }

  fn set(&mut self, name: &str, new_value: &Any) -> AttrResult<()> {
    let t: &TypeInfo<T> = Reflect::type_info();
    match t.attributes.get(name) {
      Some(attrfn) => (*attrfn)().set(self, new_value),
      None => Err(AttrError::UnknownAttribute)
    }
  }
}
