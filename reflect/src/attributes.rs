use reflect::{Reflectable, ReflectableRefExt, ReflectableMutRefExt};

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
  fn get(&self, owner: &Reflectable) -> AttrResult<T>;
  fn set(&self, owner: &mut Reflectable, new_value: T) -> AttrResult<()>;
}

pub trait OwnerAttribute<O>: Sync + 'static {
  fn get(&self, owner: &O) -> AttrResult<Box<Reflectable>>;
  fn set(&self, owner: &mut O, new_value: &Reflectable) -> AttrResult<()>;
}

pub trait AnyAttribute: Sync + 'static {
  fn get(&self, owner: &Reflectable) -> AttrResult<Box<Reflectable>>;
  fn set(&self, owner: &mut Reflectable, new_value: &Reflectable) -> AttrResult<()>;
}

impl<O, T, X> FieldAttribute<T> for X
  where X: Attribute<O, T>, O: Reflectable + 'static
{
  fn get(&self, owner: &Reflectable) -> AttrResult<T> {
    match owner.downcast_ref::<O>() {
      Some(o) => self.get_(o),
      None => Err(AttrError::WrongTargetType)
    }
  }

  fn set(&self, owner: &mut Reflectable, new_value: T) -> AttrResult<()> {
    match owner.downcast_mut::<O>() {
      Some(o) => self.set_(o, new_value),
      None => Err(AttrError::WrongTargetType)
    }
  }
}

impl<O, T, X> OwnerAttribute<O> for X
  where X: Attribute<O, T> + Sync + 'static, T: Reflectable + Clone + 'static, O: 'static
{
  fn get(&self, owner: &O) -> AttrResult<Box<Reflectable>> {
    let v = box try!(self.get_(owner));
    Ok(v as Box<Reflectable>)
  }

  fn set(&self, owner: &mut O, new_value: &Reflectable) -> AttrResult<()> {
    match new_value.downcast_ref::<T>() {
      Some(x) => {
        self.set_(owner, (*x).clone())
      },
      None => Err(AttrError::WrongValueType)
    }
  }
}

impl<O, T, X> AnyAttribute for X
  where X: Attribute<O, T> + Sync + 'static, T: Reflectable + Clone + 'static, O: Reflectable + 'static
{
  fn get(&self, owner: &Reflectable) -> AttrResult<Box<Reflectable>> {
    match owner.downcast_ref::<O>() {
      Some(o) => {
        let v = box try!(self.get_(o));
        Ok(v as Box<Reflectable>)
      },
      None => Err(AttrError::WrongTargetType)
    }
  }

  fn set(&self, owner: &mut Reflectable, new_value: &Reflectable) -> AttrResult<()> {
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
