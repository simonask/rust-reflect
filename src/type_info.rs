use attributes::{OwnerAttribute, AnyAttribute};
use phf;

pub struct TypeInfo {
  pub name: &'static str,
  pub attributes: &'static phf::Map<&'static str, fn() -> &'static AnyAttribute>
}

pub trait Type<'a> {
  fn name(&self) -> &'a str;
  fn find_attribute(&self, name: &str) -> Option<&'a AnyAttribute>;
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
