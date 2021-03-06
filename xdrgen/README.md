Rust XDR library
================

This crate provides xdrgen, which takes an XDR specification in a .x
file, and produces Rust code to serialize and deserialize the
specified types. It is intended to be used in conjunction with
[xdr-codec](https://github.com/jsgf/rust-xdr-codec).

The syntax of the .x file follows RFC4506. This has type definitions for
XDR but does not include RPC protocol specifications. Correspondingly, xdrgen
does not support auto-generation of RPC clients/servers.

Usage
-----
Usage is straightforward. You can generate the Rust code from a spec a build.rs:
```
extern crate xdrgen;

fn main() {
    xdrgen::compile("src/simple.x").unwrap();
}
```

This code can then be included into a module:
```
mod simple {
    use xdr_codec;
    
    #[allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/simple_xdr.rs"));
}
```

Once you have this, you can call `xdr_codec::pack(&mytype, &mut output)`, and
`let mything: MyThing = xdr_codec::unpack(&mut input).unwrap()`.

The serializers require your types to implement the `Pack` and `Unpack`
traits, and generate code to write to `std::io::Write` implementation, and
read from `std::io::Read`.

All types and fields are generated public, so you can control their access
outside your module or crate. If your spec references other types which are
not defined within the spec, then you can define them within the module
as well, either by aliasing them with other defined types, or implementing
the `Pack` and `Unpack` traits yourself.

Limitations
-----------
There are currently a few limitations:
   * The generated code uses identifiers as specified in the .x file, so the
     Rust code will not use normal formatting conventions.
   * It also does not filter for rust keywords, so XDR specifications intended
     for C may use identifiers like `type`.
   * XDR has discriminated unions, which are a good match for Rust enums.
     However, it also supports a `default` case if an unknown discriminator
     is encountered. This crate supports this for unpacking, but not for
     packing, as Rust does not allow enums to have unknown values.
   * Code generated for unpacking fixed-sized arrays is always fully unwound,
     since it represents these as Rust fixed-sized arrays, and there's currently
     no way to initialize them safely except by enumerating all the elements.
