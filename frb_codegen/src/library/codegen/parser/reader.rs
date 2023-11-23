use crate::codegen::dumper::Dumper;
use crate::codegen::ConfigDumpContent;
use crate::library::commands::cargo_expand::cargo_expand;
use anyhow::{Context, Result};
use itertools::Itertools;
use log::debug;
use std::path::Path;

pub(crate) fn read_rust_file(
    rust_file_path: &Path,
    rust_crate_dir: &Path,
    dumper: &Dumper,
) -> Result<String> {
    let module = get_rust_mod(rust_file_path, rust_crate_dir)?;
    debug!("read_rust_file rust_file_path={rust_file_path:?} module={module:?}");
    let ans = cargo_expand(&rust_crate_dir, module, rust_file_path)?;

    dumper.dump_str(
        ConfigDumpContent::Source,
        rust_file_path
            .strip_prefix(rust_crate_dir)
            .context("not prefix")?
            .to_str()
            .unwrap(),
        &ans,
    )?;

    Ok(ans)
}

fn get_rust_mod(rust_file_path: &Path, rust_crate_dir: &Path) -> Result<Option<String>> {
    let relative_path = rust_file_path.strip_prefix(rust_crate_dir.join("src"))?;

    let mut components = relative_path
        .iter()
        .map(|s| s.to_str().unwrap().to_owned())
        .collect_vec();

    strip_suffix_inplace(components.last_mut().unwrap(), ".rs");

    if components.last().unwrap() == "mod" || components.last().unwrap() == "lib" {
        components.pop();
    }

    let ans = components.join("::");
    Ok(if ans.is_empty() { None } else { Some(ans) })
}

fn strip_suffix_inplace(s: &mut String, suffix: &str) {
    if let Some(stripped) = s.strip_suffix(suffix) {
        *s = stripped.to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    pub fn test_get_dir_and_mod_simple_mod() {
        let actual = get_rust_mod(
            &PathBuf::from("/project/src/api.rs"),
            &PathBuf::from("/project"),
        )
        .unwrap();
        assert_eq!(Some("api".to_owned()), actual);
    }

    #[test]
    pub fn test_get_dir_and_mod_sub_mod() {
        let actual = get_rust_mod(
            &PathBuf::from("/project/src/sub/subsub.rs"),
            &PathBuf::from("/project"),
        )
        .unwrap();
        assert_eq!(Some("sub::subsub".to_owned()), actual);
    }

    #[test]
    pub fn test_get_dir_and_mod_lib_rs() {
        let actual = get_rust_mod(
            &PathBuf::from("/project/src/lib.rs"),
            &PathBuf::from("/project"),
        )
        .unwrap();
        assert_eq!(None, actual);
    }

    #[test]
    pub fn test_get_dir_and_mod_mod_rs() {
        let actual = get_rust_mod(
            &PathBuf::from("/project/src/hello/mod.rs"),
            &PathBuf::from("/project"),
        )
        .unwrap();
        assert_eq!(Some("hello".to_owned()), actual);
    }
}
