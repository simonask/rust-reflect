#![feature(macro_rules)]
#![feature(phase)]

extern crate reflect;
#[phase(plugin)]
extern crate reflect_mac;

mod test_reflect;
