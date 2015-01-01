# Reflection for Rust

This is a small reflection library for the Rust programming language.

It is still in very early development, and is far from feature complete. The library incurs zero
run-time penalty for features you don't use, but may slightly increase your binary sizes.

The library is meant to aid in the implementation of things like web frameworks and ORMs.

WARNING: rust-reflect currently uses experimental and unstable compiler features, and things may break
unexpectedly as new versions of the Rust compiler are released.

## Example

```rust
#![feature(phase)]

extern crate reflect;
#[phase(plugin)]
extern crate reflect_mac;

use reflect::{Reflect, ReflectRefExt, GetType, Type};

#[reflect]
struct Foo {
  foo: i32
}

fn main() {
  let foo = Foo { foo: 123 };

  println!("Name of type: {}", GetType::of::<Foo>().name());

  match foo.get("foo") {
    Ok(x) => match (*x).downcast_ref::<i32>() {
      Some(n) => println!("foo.foo = {}", n),
      None => println!("foo.foo was not an i32!")
    },
    Err(_) => println!("foo.foo does not exist")
  }
}
```

# TODOs

- Enums
- Observers
- Signals/slots
- Getter/setter attributes
- `#[omit_reflect]` attribute
- Serialization/deserialization interface
