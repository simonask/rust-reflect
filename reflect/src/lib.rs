#![feature(phase, macro_rules)]
#![allow(missing_copy_implementations)]

extern crate phf;
#[phase(plugin)]
extern crate phf_mac;

pub use attributes::{OwnerAttribute, AnyAttribute, Attribute, AttrResult, AttributeMap};
pub use type_info::{TypeInfo, TypeInfoFor, Type, GetType};
pub use reflect::{StaticReflection, Reflect, ReflectRefExt, ReflectMutRefExt};

pub mod attributes;
pub mod type_info;
pub mod reflect;
pub mod types;
