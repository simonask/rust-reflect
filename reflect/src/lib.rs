#![feature(phase, macro_rules)]
#![allow(missing_copy_implementations)]

extern crate phf;
#[phase(plugin)]
extern crate phf_mac;

#[doc(inline)]
pub use attributes::{Attribute, AnyAttribute, FieldAttribute, OwnerAttribute, AttrResult, AttrError, AttributeMap};

#[doc(inline)]
pub use type_info::{TypeInfo, GetTypeInfo, TypeInfoFor, Type, GetType};

#[doc(inline)]
pub use reflect::{Reflect, ReflectRefExt, ReflectMutRefExt};

pub mod attributes;
pub mod type_info;
pub mod reflect;
pub mod types;
