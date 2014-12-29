#![feature(phase)]

extern crate phf;
#[phase(plugin)]
extern crate phf_mac;

pub use attributes::{OwnerAttribute, AnyAttribute, Attribute, AttrResult};
pub use type_info::{TypeInfo, Type};
pub use reflect::{Reflect, Reflectable, ReflectableRefExt, ReflectableMutRefExt};

pub mod attributes;
pub mod type_info;
pub mod reflect;
pub mod types;
