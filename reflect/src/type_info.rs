use attributes::{OwnerAttribute, AnyAttribute, AttributeMap};
use reflect::{StaticReflection};

pub struct TypeInfo {
  pub name: &'static str,
  pub attributes: &'static AttributeMap,
}

pub struct TypeInfoFor<T>(pub &'static TypeInfo);

pub trait Type<'a> {
  fn name(&self) -> &'a str;
  fn find_attribute(&self, name: &str) -> Option<&'a AnyAttribute>;
}

pub struct GetType;

impl GetType {
  pub fn of<T: StaticReflection>() -> &'static TypeInfo {
    let TypeInfoFor(ti) = StaticReflection::type_info_for(None::<T>);
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
