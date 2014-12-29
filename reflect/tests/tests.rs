#![feature(macro_rules)]
#![feature(phase)]

extern crate reflect;
extern crate phf;
#[phase(plugin)]
extern crate phf_mac;

mod test_reflect;
mod foo;
