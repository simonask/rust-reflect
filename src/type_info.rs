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
