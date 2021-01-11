# `tensorflow_proto`

[![Continuous Integration](https://github.com/cpcloud/tensorflow_proto/workflows/Continuous%20Integration/badge.svg)](https://github.com/cpcloud/tensorflow_proto/actions?query=branch%3Amaster+event%3Apush+workflow%3A%22Continuous+Integration%22)
[![Crates.io](https://img.shields.io/crates/v/tensorflow_proto)](https://crates.io/crates/tensorflow_proto)
[![docs.rs](https://docs.rs/tensorflow_proto/badge.svg)](https://docs.rs/crate/tensorflow_proto)

`tensorflow_proto` is a crate that uses `prost-build` to generate Rust
`struct`s to be used in serialization/deserialization of protocol buffers
wherever Tensorflow uses them.

In particular, this is useful in the
[`tensorflow/rust`](https://github.com/tensorflow/rust) library when calling
[`SessionOptions::set_config`](https://tensorflow.github.io/rust/tensorflow/struct.SessionOptions.html#method.set_config)
to configure Tensorflow.

**Note**: **This crate is tested against tensorflow 1.15.2 and 2.0.0.**

# Usage

## Default Features

Add

```toml
tensorflow_proto = "0.3.0"
```

to your `Cargo.toml`.

## Serde Support

Serde support can be enabled using the `"serde-derive"` feature:

```toml
tensorflow_proto = { version = "0.3.0", features = ["serde-derive"] }
```

This will add a [`#[derive(serde::Serialize,
serde::Deserialize)]`](https://serde.rs/derive.html) to every generated
`struct`.

You must also depend on `serde` as well.

## Easy conversion to bytes

Finally, you can enable code generation for an implementation of `std::convert::TryFrom`
that encodes a message into a `Vec<u8>` for every `struct` generated by prost:

```toml
tensorflow_proto = { version = "0.3.0", features = ["convert"] }
```

## Use custom Tensorflow `*.proto` sources

To use a different version of Tensorflow protocol buffer sources, define
`TENSORFLOW_PROTO_SOURCE` to be the root of a Tensorflow source tree.
