use reflect::{Reflect, ReflectRefExt, ReflectMutRefExt};
use type_info::{Type, GetType, GetTypeInfo};
use phf;

pub enum AttrError {
  WrongTargetType,
  WrongValueType,
  UnknownAttribute,
}
impl Copy for AttrError {}

pub type AttrResult<T> = Result<T, AttrError>;

/// Attribute where both the owner type and the field type are known.
pub trait Attribute<O, T> {
  fn get(&self, owner: &O) -> AttrResult<T>;
  fn set(&self, owner: &mut O, new_value: T) -> AttrResult<()>;
}

/// Attribute where only the field type is known.
pub trait FieldAttribute<T> {
  fn get(&self, owner: &Reflect) -> AttrResult<T>;
  fn set(&self, owner: &mut Reflect, new_value: T) -> AttrResult<()>;
}

/// Attribute where only the owner type is known.
pub trait OwnerAttribute<O>: Sync + 'static {
  fn get(&self, owner: &O) -> AttrResult<Box<Reflect>>;
  fn set(&self, owner: &mut O, new_value: &Reflect) -> AttrResult<()>;
  fn type_info(&self) -> &'static Type<'static>;
}

/// Attribute where neither the owner type nor the field type are known.
pub trait AnyAttribute: Sync + 'static {
  fn get(&self, owner: &Reflect) -> AttrResult<Box<Reflect>>;
  fn set(&self, owner: &mut Reflect, new_value: &Reflect) -> AttrResult<()>;
  fn type_info(&self) -> &'static Type<'static>;
}

/// A map of named attributes.
pub type AttributeMap = phf::Map<&'static str, fn() -> &'static AnyAttribute>;

impl<O, T, X> FieldAttribute<T> for X
  where X: Attribute<O, T>, O: Reflect + 'static
{
  fn get(&self, owner: &Reflect) -> AttrResult<T> {
    match owner.downcast_ref::<O>() {
      Some(o) => self.get(o),
      None => Err(AttrError::WrongTargetType)
    }
  }

  fn set(&self, owner: &mut Reflect, new_value: T) -> AttrResult<()> {
    match owner.downcast_mut::<O>() {
      Some(o) => self.set(o, new_value),
      None => Err(AttrError::WrongTargetType)
    }
  }
}

impl<O, T, X> OwnerAttribute<O> for X
  where X: Attribute<O, T> + Sync + 'static, T: GetTypeInfo + Reflect + Clone + 'static, O: 'static
{
  fn get(&self, owner: &O) -> AttrResult<Box<Reflect>> {
    let v = box try!((self as &Attribute<O, T>).get(owner));
    Ok(v as Box<Reflect>)
  }

  fn set(&self, owner: &mut O, new_value: &Reflect) -> AttrResult<()> {
    match new_value.downcast_ref::<T>() {
      Some(x) => {
        (self as &Attribute<O, T>).set(owner, (*x).clone())
      },
      None => Err(AttrError::WrongValueType)
    }
  }

  fn type_info(&self) -> &'static Type<'static> {
    GetType::of::<T>() as &Type<'static>
  }
}

impl<O, T, X> AnyAttribute for X
  where X: Attribute<O, T> + Sync + 'static, T: GetTypeInfo + Reflect + Clone + 'static, O: Reflect + 'static
{
  fn get(&self, owner: &Reflect) -> AttrResult<Box<Reflect>> {
    match owner.downcast_ref::<O>() {
      Some(o) => {
        let v = box try!((self as &Attribute<O, T>).get(o));
        Ok(v as Box<Reflect>)
      },
      None => Err(AttrError::WrongTargetType)
    }
  }

  fn set(&self, owner: &mut Reflect, new_value: &Reflect) -> AttrResult<()> {
    match owner.downcast_mut::<O>() {
      Some(o) => {
        match new_value.downcast_ref::<T>() {
          Some(x) => Ok(try!((self as &Attribute<O, T>).set(o, x.clone()))),
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
