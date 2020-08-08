#![allow(clippy::large_enum_variant)]
include!(concat!(env!("OUT_DIR"), "/tensorflow_proto_gen.rs"));

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to encode protobuf to bytes")]
    Encode(#[source] prost::EncodeError),
}

/// Serialize a protobuf message into a vector of bytes.
pub fn into_bytes(msg: impl prost::Message) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![];
    msg.encode(&mut bytes).map_err(Error::Encode)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::{into_bytes, tensorflow};

    #[test]
    fn gpu_options() {
        let config_proto = tensorflow::ConfigProto {
            gpu_options: Some(tensorflow::GpuOptions {
                allow_growth: true,
                ..Default::default()
            }),
            ..Default::default()
        };
        let bytes = into_bytes(config_proto).unwrap();
        assert!(!bytes.is_empty());
    }
}
