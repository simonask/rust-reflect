#![feature(phase)]
#![allow(missing_copy_implementations)]

extern crate phf;
#[phase(plugin)]
extern crate phf_mac;

pub use attributes::{OwnerAttribute, AnyAttribute, Attribute, AttrResult, AttributeMap};
pub use type_info::{TypeInfo, Type};
pub use reflect::{GetType, Reflect, ReflectStatic, Reflectable, ReflectableRefExt, ReflectableMutRefExt, StaticTypeInfo};

pub mod attributes;
pub mod type_info;
pub mod reflect;
pub mod types;
