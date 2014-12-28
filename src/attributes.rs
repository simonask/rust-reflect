use std::any::{Any, AnyRefExt, AnyMutRefExt};

pub enum AttrError {
  WrongTargetType,
  WrongValueType,
  UnknownAttribute,
}
impl Copy for AttrError {}

pub type AttrResult<T> = Result<T, AttrError>;

pub trait Attribute<O, T> {
  fn get_(&self, owner: &O) -> AttrResult<T>;
  fn set_(&self, owner: &mut O, new_value: T) -> AttrResult<()>;
}

pub trait FieldAttribute<T> {
  fn get(&self, owner: &Any) -> AttrResult<T>;
  fn set(&self, owner: &mut Any, new_value: T) -> AttrResult<()>;
}

pub trait OwnerAttribute<O>: Sync + 'static {
  fn get(&self, owner: &O) -> AttrResult<Box<Any>>;
  fn set(&self, owner: &mut O, new_value: &Any) -> AttrResult<()>;
}

pub trait AnyAttribute: Sync + 'static {
  fn get(&self, owner: &Any) -> AttrResult<Box<Any>>;
  fn set(&self, owner: &mut Any, new_value: &Any) -> AttrResult<()>;
}

impl<O, T, X> FieldAttribute<T> for X
  where X: Attribute<O, T>, O: Any + 'static
{
  fn get(&self, owner: &Any) -> AttrResult<T> {
    match owner.downcast_ref::<O>() {
      Some(o) => self.get_(o),
      None => Err(AttrError::WrongTargetType)
    }
  }

  fn set(&self, owner: &mut Any, new_value: T) -> AttrResult<()> {
    match owner.downcast_mut::<O>() {
      Some(o) => self.set_(o, new_value),
      None => Err(AttrError::WrongTargetType)
    }
  }
}

impl<O, T, X> OwnerAttribute<O> for X
  where X: Attribute<O, T> + Sync + 'static, T: Any + Clone + 'static
{
  fn get(&self, owner: &O) -> AttrResult<Box<Any>> {
    let v = box try!(self.get_(owner));
    Ok(v as Box<Any>)
  }

  fn set(&self, owner: &mut O, new_value: &Any) -> AttrResult<()> {
    match new_value.downcast_ref::<T>() {
      Some(x) => {
        self.set_(owner, (*x).clone())
      },
      None => Err(AttrError::WrongValueType)
    }
  }
}

impl<O, T, X> AnyAttribute for X
  where X: Attribute<O, T> + Sync + 'static, T: Any + Clone + 'static, O: Any + 'static
{
  fn get(&self, owner: &Any) -> AttrResult<Box<Any>> {
    match owner.downcast_ref::<O>() {
      Some(o) => {
        let v = box try!(self.get_(o));
        Ok(v as Box<Any>)
      },
      None => Err(AttrError::WrongTargetType)
    }
  }

  fn set(&self, owner: &mut Any, new_value: &Any) -> AttrResult<()> {
    match owner.downcast_mut::<O>() {
      Some(o) => {
        match new_value.downcast_ref::<T>() {
          Some(x) => Ok(try!(self.set_(o, x.clone()))),
          None => Err(AttrError::WrongValueType)
        }
      },
      None => Err(AttrError::WrongTargetType)
    }
  }
}
