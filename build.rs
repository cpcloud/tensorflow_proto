use anyhow::{anyhow, Context, Result};
use std::{collections::HashMap, fmt, io::Write, path::PathBuf};

// The name of the file generated by this crate, used in src/lib.rs
const GENERATED_FILE_NAME: &str = "tensorflow_proto_gen.rs";

// The default name of the tensorflow proto source if the TENSORFLOW_PROTO_SOURCE environment
// variable isn't defined.
const DEFAULT_TENSORFLOW_PROTO_SOURCE: &str = "./proto";

// The directory containing the protocol buffer source tree.
const TENSORFLOW_PROTO_SOURCE: Option<&str> = option_env!("TENSORFLOW_PROTO_SOURCE");

// The default extension to use to find protocol buffer definitions.
const DEFAULT_PROTO_FILE_EXT: &str = ".proto";

// The environment variable referring to the protocol buffer file extension
const PROTO_FILE_EXT: Option<&str> = option_env!("PROTO_FILE_EXT");

struct ModMap {
    name: String,
    include: Option<String>,
    children: HashMap<String, ModMap>,
}

impl fmt::Display for ModMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "pub mod {} {{", self.name)?;
        if let Some(ref include) = self.include {
            writeln!(f, r#"include!("{}");"#, include)?;
        }
        for value in self.children.values() {
            writeln!(f, "{}", value)?;
        }
        write!(f, "}}")
    }
}

fn file_name(path: impl AsRef<std::path::Path>) -> Result<String> {
    Ok(path
        .as_ref()
        .file_name()
        .ok_or_else(|| anyhow!("path has no file_name"))?
        .to_str()
        .ok_or_else(|| anyhow!("invalid unicode file_name"))?
        .to_owned())
}

fn main() -> Result<()> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let suffix = PROTO_FILE_EXT.unwrap_or(DEFAULT_PROTO_FILE_EXT);
    let source = TENSORFLOW_PROTO_SOURCE
        .map_or_else(|| DEFAULT_TENSORFLOW_PROTO_SOURCE.into(), PathBuf::from);
    let schema_files = glob::glob(
        &source
            .join("**")
            .join(format!("*{}", suffix))
            .display()
            .to_string(),
    )?
    .collect::<Result<Vec<_>, _>>()?;

    for path in schema_files.iter() {
        println!("cargo:rerun-if-changed={}", path.display().to_string());
    }

    if !schema_files.is_empty() {
        let mut cfg = prost_build::Config::new();
        cfg.out_dir(&out_dir).compile_well_known_types();

        if std::env::var("CARGO_FEATURE_SERDE_DERIVE").is_ok() {
            cfg.type_attribute(".", "#[tensorflow_proto_derive::serde_default_viable]");
        }

        cfg.compile_protos(&schema_files, &[source])?;
    }

    let mut root = HashMap::new();
    for result_entry in glob::glob(&out_dir.join("*.rs").display().to_string())? {
        let entry = result_entry?;
        let basename = file_name(&entry)?;
        if basename != GENERATED_FILE_NAME {
            let base_module_name = file_name(entry.with_extension(""))?;
            let mod_path = base_module_name
                .split('.')
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>();
            let top = mod_path[0].clone();
            let rest = &mod_path[1..];
            let mut tree = root.entry(top.clone()).or_insert_with(|| ModMap {
                name: top,
                include: Default::default(),
                children: HashMap::new(),
            });
            for module in rest {
                tree.children
                    .entry(module.to_owned())
                    .or_insert_with(move || ModMap {
                        name: module.to_owned(),
                        include: None,
                        children: HashMap::new(),
                    });
                tree = tree
                    .children
                    .get_mut(module)
                    .ok_or_else(|| anyhow!("{} module not found", module))?;
            }
            tree.include = Some(basename);
        }
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(out_dir.join(GENERATED_FILE_NAME))?;

    for (module, value) in root {
        writeln!(file, "{}", value).with_context(move || {
            format!(
                "failed to write rust module for tensorflow protobuf: {}",
                module
            )
        })?;
    }

    Ok(())
}
