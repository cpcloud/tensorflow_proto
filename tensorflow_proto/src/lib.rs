//! `tensorflow_proto`
//!
//! This library exposes protocol buffers from Tensorflow in the form of Rust structs, to allow end
//! users to consume and produce them.
#![allow(clippy::large_enum_variant)]
include!(concat!(env!("OUT_DIR"), "/tensorflow_proto_gen.rs"));

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde-derive")]
    mod serde {
        use crate::tensorflow;

        #[test]
        fn test_serde_roundtrip() {
            let config_proto = tensorflow::ConfigProto {
                gpu_options: Some(tensorflow::GpuOptions {
                    allow_growth: true,
                    ..Default::default()
                }),
                ..Default::default()
            };
            let js = serde_json::to_string(&config_proto).unwrap();
            let result: tensorflow::ConfigProto = serde_json::from_str(&js).unwrap();
            assert_eq!(config_proto, result);
        }

        #[test]
        fn test_deser_with_defaults() {
            let config_proto = tensorflow::ConfigProto {
                gpu_options: Some(tensorflow::GpuOptions {
                    allow_growth: true,
                    ..Default::default()
                }),
                ..Default::default()
            };
            let js = r#"{"gpu_options": {"allow_growth": true}}"#;
            let result: tensorflow::ConfigProto = serde_json::from_str(js).unwrap();
            assert_eq!(config_proto, result);
        }
    }

    #[cfg(feature = "convert")]
    mod convert {
        use crate::tensorflow;
        use std::convert::{TryFrom, TryInto};

        #[test]
        fn test_try_from_message() {
            let config_proto = tensorflow::ConfigProto {
                gpu_options: Some(tensorflow::GpuOptions {
                    allow_growth: true,
                    ..Default::default()
                }),
                ..Default::default()
            };
            let bytes = Vec::try_from(config_proto).unwrap();
            assert!(!bytes.is_empty());
        }

        #[test]
        fn test_try_into_bytes() {
            let config_proto = tensorflow::ConfigProto {
                gpu_options: Some(tensorflow::GpuOptions {
                    allow_growth: true,
                    ..Default::default()
                }),
                ..Default::default()
            };
            let bytes: Vec<_> = config_proto.try_into().unwrap();
            assert!(!bytes.is_empty());
        }

        #[test]
        fn test_try_from_bytes() {
            let config_proto = tensorflow::ConfigProto {
                gpu_options: Some(tensorflow::GpuOptions {
                    allow_growth: true,
                    ..Default::default()
                }),
                ..Default::default()
            };
            let bytes = Vec::try_from(config_proto.clone()).unwrap();
            assert_eq!(
                tensorflow::ConfigProto::try_from(bytes).unwrap(),
                config_proto
            );
        }

        #[test]
        fn test_try_into_message() {
            let config_proto = tensorflow::ConfigProto {
                gpu_options: Some(tensorflow::GpuOptions {
                    allow_growth: true,
                    ..Default::default()
                }),
                ..Default::default()
            };
            let bytes: Vec<_> = config_proto.clone().try_into().unwrap();
            let message: tensorflow::ConfigProto = bytes.try_into().unwrap();
            assert_eq!(message, config_proto);
        }
    }
}
