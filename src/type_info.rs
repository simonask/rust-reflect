use attributes::{OwnerAttribute, AnyAttribute};
use phf;

pub struct TypeInfo<T> {
  pub name: &'static str,
  pub attributes: &'static phf::Map<&'static str, fn() -> &'static OwnerAttribute<T>>
}

pub trait Type<'a> {
  fn name(&self) -> &'a str;
  fn find_attribute(&self, name: &str) -> Option<&'a AnyAttribute>;
}

impl<T> Type<'static> for TypeInfo<T> {
  fn name(&self) -> &'static str {
    self.name
  }

  fn find_attribute(&self, name: &str) -> Option<&'static AnyAttribute> {
    match self.attributes.get(name) {
      Some(attrfn) => {
        let attr = (*attrfn)();
        Some(attr.to_any_attr())
      },
      None => None
    }
  }
}
