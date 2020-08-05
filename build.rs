use anyhow::{anyhow, Context, Result};
use std::{collections::HashMap, fmt, io::Write, path::PathBuf};
use walkdir::WalkDir;

const DEFAULT_PROTO_FILE_EXT: &str = "proto";
const GENERATED_FILE_NAME: &str = "tensorflow_proto_gen.rs";

struct ModMap {
    name: String,
    include: Option<String>,
    children: HashMap<String, ModMap>,
}

impl fmt::Display for ModMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pub mod {}", self.name)?;
        writeln!(f, " {{ ")?;
        if let Some(ref include) = self.include {
            writeln!(f, r#"include!("{}");"#, include)?;
        }
        for value in self.children.values() {
            writeln!(f, "{}", value)?;
        }
        write!(f, "}}")
    }
}

fn main() -> Result<()> {
    // collect the files in the schemas directory (and subdirectories) into a vec and separately
    // keep a list of directories encountered
    let source = std::env::var("TENSORFLOW_PROTO_SOURCE")
        .with_context(|| "TENSORFLOW_PROTO_SOURCE not defined")?;
    let suffix =
        std::env::var("PROTO_FILE_EXT").unwrap_or_else(|_| DEFAULT_PROTO_FILE_EXT.to_owned());
    let (schema_files, schema_directories) = WalkDir::new(source.clone())
        .follow_links(true)
        .into_iter()
        .try_fold((vec![], vec![]), |mut containers, result_entry| {
            let entry = result_entry?;
            let (ref mut files, ref mut directories) = containers;
            if entry.file_type().is_dir() {
                directories.push(entry.into_path());
            } else if entry.path().to_string_lossy().ends_with(&suffix) {
                files.push(entry.into_path());
            }
            Ok::<_, anyhow::Error>(containers)
        })?;

    for path in schema_directories.iter().chain(schema_files.iter()) {
        println!(
            "cargo:rerun-if-changed={}",
            path.to_str()
                .ok_or_else(|| anyhow!("path is not valid unicode"))?
        );
    }

    if !schema_files.is_empty() {
        prost_build::compile_protos(&schema_files, &[source.into()])?;
    }

    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

    let mut root = HashMap::new();
    for result_entry in glob::glob(&out_dir.join("*.rs").display().to_string())? {
        let entry = result_entry?;

        let basename = entry
            .file_name()
            .ok_or_else(|| anyhow!("path has no file_name"))?
            .to_str()
            .ok_or_else(|| anyhow!("invalid unicode file_name"))?
            .to_owned();
        if basename != GENERATED_FILE_NAME {
            let base_module_name = entry
                .with_extension("")
                .file_name()
                .ok_or_else(|| anyhow!("path has no file_name"))?
                .to_str()
                .ok_or_else(|| anyhow!("invalid unicode file_name"))?
                .to_owned();
            let mod_path = base_module_name
                .split('.')
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            let top = mod_path[0].clone();
            let rest = &mod_path[1..];
            let mut tree = root.entry(top.clone()).or_insert_with(|| ModMap {
                name: top,
                include: None,
                children: HashMap::new(),
            });
            for module in rest {
                tree.children
                    .entry(module.into())
                    .or_insert_with(|| ModMap {
                        name: module.into(),
                        include: None,
                        children: HashMap::new(),
                    });
                tree = tree
                    .children
                    .get_mut(module)
                    .ok_or_else(|| anyhow!("{} not found", module))?;
            }
            tree.include = Some(basename);
        }
    }

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(out_dir.join(GENERATED_FILE_NAME))?;

    for (module, value) in root.iter() {
        writeln!(file, "{}", value)
            .with_context(|| format!("failed to write module: {}", module))?;
    }

    Ok(())
}
