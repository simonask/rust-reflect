use attributes::{OwnerAttribute, AnyAttribute, AttributeMap};

pub struct TypeInfo {
  pub name: &'static str,
  pub attributes: &'static AttributeMap,
}

pub struct TypeInfoFor<T>(pub &'static TypeInfo);

pub trait GetTypeInfo {
  fn get_type_info(_ignored: Option<Self>) -> TypeInfoFor<Self>;
}

pub trait Type<'a> {
  fn name(&self) -> &'a str;
  fn find_attribute(&self, name: &str) -> Option<&'a AnyAttribute>;
}

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
