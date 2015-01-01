use reflect::{StaticReflection, Reflect, ReflectRefExt, ReflectMutRefExt};
use type_info::{Type, GetType};
use phf;

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
  fn get(&self, owner: &Reflect) -> AttrResult<T>;
  fn set(&self, owner: &mut Reflect, new_value: T) -> AttrResult<()>;
}

pub trait OwnerAttribute<O>: Sync + 'static {
  fn get(&self, owner: &O) -> AttrResult<Box<Reflect>>;
  fn set(&self, owner: &mut O, new_value: &Reflect) -> AttrResult<()>;
  fn type_info(&self) -> &'static Type<'static>;
}

pub trait AnyAttribute: Sync + 'static {
  fn get(&self, owner: &Reflect) -> AttrResult<Box<Reflect>>;
  fn set(&self, owner: &mut Reflect, new_value: &Reflect) -> AttrResult<()>;
  fn type_info(&self) -> &'static Type<'static>;
}

pub type AttributeMap = phf::Map<&'static str, fn() -> &'static AnyAttribute>;

impl<O, T, X> FieldAttribute<T> for X
  where X: Attribute<O, T>, O: Reflect + 'static
{
  fn get(&self, owner: &Reflect) -> AttrResult<T> {
    match owner.downcast_ref::<O>() {
      Some(o) => self.get_(o),
      None => Err(AttrError::WrongTargetType)
    }
  }

  fn set(&self, owner: &mut Reflect, new_value: T) -> AttrResult<()> {
    match owner.downcast_mut::<O>() {
      Some(o) => self.set_(o, new_value),
      None => Err(AttrError::WrongTargetType)
    }
  }
}

impl<O, T, X> OwnerAttribute<O> for X
  where X: Attribute<O, T> + Sync + 'static, T: StaticReflection + Reflect + Clone + 'static, O: 'static
{
  fn get(&self, owner: &O) -> AttrResult<Box<Reflect>> {
    let v = box try!(self.get_(owner));
    Ok(v as Box<Reflect>)
  }

  fn set(&self, owner: &mut O, new_value: &Reflect) -> AttrResult<()> {
    match new_value.downcast_ref::<T>() {
      Some(x) => {
        self.set_(owner, (*x).clone())
      },
      None => Err(AttrError::WrongValueType)
    }
  }

  fn type_info(&self) -> &'static Type<'static> {
    GetType::of::<T>() as &Type<'static>
  }
}

impl<O, T, X> AnyAttribute for X
  where X: Attribute<O, T> + Sync + 'static, T: StaticReflection + Reflect + Clone + 'static, O: Reflect + 'static
{
  fn get(&self, owner: &Reflect) -> AttrResult<Box<Reflect>> {
    match owner.downcast_ref::<O>() {
      Some(o) => {
        let v = box try!(self.get_(o));
        Ok(v as Box<Reflect>)
      },
      None => Err(AttrError::WrongTargetType)
    }
  }

  fn set(&self, owner: &mut Reflect, new_value: &Reflect) -> AttrResult<()> {
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

  fn type_info(&self) -> &'static Type<'static> {
    GetType::of::<T>() as &Type<'static>
  }
}
