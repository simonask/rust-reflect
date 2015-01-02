use type_info::{Type, GetType, GetTypeInfo};
use attributes::{AttrResult, AttrError};
use std::any::{Any, AnyRefExt, AnyMutRefExt};

pub trait Reflect: Any {
  fn type_info(&self) -> &'static Type<'static>;
  fn get(&self, name: &str) -> AttrResult<Box<Reflect>>;
  fn set(&mut self, name: &str, new_value: &Reflect) -> AttrResult<()>;

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

pub trait ReflectRefExt<'a> {
  fn is<T: 'static>(self) -> bool;
  fn downcast_ref<T: 'static>(self) -> Option<&'a T>;
}

pub trait ReflectMutRefExt<'a> {
  fn downcast_mut<T: 'static>(self) -> Option<&'a mut T>;
}

impl<'a> ReflectRefExt<'a> for &'a Reflect {
  fn is<T: 'static>(self) -> bool {
    self.as_any_ref().is::<T>()
  }

  fn downcast_ref<T: 'static>(self) -> Option<&'a T> {
    self.as_any_ref().downcast_ref::<T>()
  }
}

impl<'a> ReflectMutRefExt<'a> for &'a mut Reflect {
  fn downcast_mut<T: 'static>(self) -> Option<&'a mut T> {
    self.as_any_mut_ref().downcast_mut::<T>()
  }
}

impl<'a, T> Reflect for T
  where T: GetTypeInfo + 'static
{
  fn type_info(&self) -> &'static Type<'static> {
    let t = GetType::of::<T>();
    t as &'static Type<'static>
  }

  fn get(&self, name: &str) -> AttrResult<Box<Reflect>> {
    let ti = GetType::of::<T>();
    match ti.attributes.get(name) {
      Some(attrfn) => (*attrfn)().get(self),
      None => Err(AttrError::UnknownAttribute)
    }
  }

  fn set(&mut self, name: &str, new_value: &Reflect) -> AttrResult<()> {
    let ti = GetType::of::<T>();
    match ti.attributes.get(name) {
      Some(attrfn) => (*attrfn)().set(self, new_value),
      None => Err(AttrError::UnknownAttribute)
    }
  }
}
