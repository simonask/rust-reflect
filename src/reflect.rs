use type_info::{Type, TypeInfo};
use attributes::{AttrResult, AttrError};
use std::any::{Any, AnyRefExt, AnyMutRefExt};

pub trait Reflect<'a> {
  fn type_info() -> &'a TypeInfo<Self>;
}

pub trait Reflectable: Any {
  fn type_info(&self) -> &'static Type<'static>;
  fn get(&self, name: &str) -> AttrResult<Box<Reflectable>>;
  fn set(&mut self, name: &str, new_value: &Reflectable) -> AttrResult<()>;

  // The following is because we want to reuse the definition of downcast/is from
  // AnyRefExt+AnyMutRefExt, but we can't cast between traits. :-(
  // In the future this should go away, as component casting is implemented.
  fn as_any_ref(&self) -> &Any {
    self as &Any
  }
  fn as_any_mut_ref(&mut self) -> &mut Any {
    self as &mut Any
  }
}

pub trait ReflectableRefExt<'a> {
  fn is<T: 'static>(self) -> bool;
  fn downcast_ref<T: 'static>(self) -> Option<&'a T>;
}

pub trait ReflectableMutRefExt<'a> {
  fn downcast_mut<T: 'static>(self) -> Option<&'a mut T>;
}

impl<'a> ReflectableRefExt<'a> for &'a Reflectable {
  fn is<T: 'static>(self) -> bool {
    self.as_any_ref().is::<T>()
  }

  fn downcast_ref<T: 'static>(self) -> Option<&'a T> {
    self.as_any_ref().downcast_ref::<T>()
  }
}

impl<'a> ReflectableMutRefExt<'a> for &'a mut Reflectable {
  fn downcast_mut<T: 'static>(self) -> Option<&'a mut T> {
    self.as_any_mut_ref().downcast_mut::<T>()
  }
}

impl<'a, T> Reflectable for T
  where T: Reflect<'static> + 'static
{
  fn type_info(&self) -> &'static Type<'static> {
    let t: &TypeInfo<T> = Reflect::type_info();
    t as &'static Type<'static>
  }

  fn get(&self, name: &str) -> AttrResult<Box<Reflectable>> {
    let t: &TypeInfo<T> = Reflect::type_info();
    match t.attributes.get(name) {
      Some(attrfn) => (*attrfn)().get(self),
      None => Err(AttrError::UnknownAttribute)
    }
  }

  fn set(&mut self, name: &str, new_value: &Reflectable) -> AttrResult<()> {
    let t: &TypeInfo<T> = Reflect::type_info();
    match t.attributes.get(name) {
      Some(attrfn) => (*attrfn)().set(self, new_value),
      None => Err(AttrError::UnknownAttribute)
    }
  }
}
