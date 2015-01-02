use attributes::{OwnerAttribute, AnyAttribute, AttributeMap};

/// Static information about a type.
pub struct TypeInfo {
  pub name: &'static str,
  pub attributes: &'static AttributeMap,
}

/// Static information about type `T`. This is used to disambiguate
/// `TypeInfo`s when requesting information about a specific compile-time type.
pub struct TypeInfoFor<T>(pub &'static TypeInfo);

/// Get static information about type `T`. Implement this to get the `Reflect` trait for free.
pub trait GetTypeInfo {
  fn get_type_info(_ignored: Option<Self>) -> TypeInfoFor<Self>;
}

/// Information about any type.
pub trait Type<'a> {
  fn name(&self) -> &'a str;
  fn find_attribute(&self, name: &str) -> Option<&'a AnyAttribute>;
}

/// Get information about any type.
pub struct GetType;

impl GetType {
  pub fn of<T: GetTypeInfo>() -> &'static TypeInfo {
    let TypeInfoFor(ti) = GetTypeInfo::get_type_info(None::<T>);
    ti
  }
}

impl Type<'static> for TypeInfo {
  fn name(&self) -> &'static str {
    self.name
  }

  fn find_attribute(&self, name: &str) -> Option<&'static AnyAttribute> {
    match self.attributes.get(name) {
      Some(attrfn) => {
        let attr = (*attrfn)();
        Some(attr)
      },
      None => None
    }
  }
}
