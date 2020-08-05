# `tensorflow_proto`

`tensorflow_proto` is a shortish `build.rs` script that uses `prost-build` to
generate Rust `struct`s that can be used in serialization/deserialization of protocol buffers
wherever tensorflow uses them.

In particular, this is useful in rust/tensorflow library when calling `SessionOptions::set_config`
to configure different parts of tensorflow.

# Usage

To use this module, you need to define the `TENSORFLOW_PROTO_SOURCE` _at build
time_, so that the` build.rs` script of this crate knows where to find
tensorflow's `.proto` files.
